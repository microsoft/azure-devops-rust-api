// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Download Universal Packages from Azure DevOps Artifacts.
//!
//! This module implements the dedup-based download protocol used by
//! Azure DevOps Artifacts for universal packages.
//!
//! # Protocol overview
//!
//! 1. Discover service URLs via the ResourceAreas API
//! 2. Get package metadata (manifestId, superRootId) from the packaging endpoint
//! 3. Resolve blob IDs to download URLs via the dedup service
//! 4. Download and parse the manifest to get the file/chunk structure
//! 5. Download content chunks, decompress, and reassemble files

mod decompress;

use azure_core::error::{Error, ErrorKind, Result, ResultExt};
use azure_core::headers::{self, HeaderValue};
use azure_core::{Method, Request, Url};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

pub use decompress::decompress_chunk;

// --- Data structures ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResourceArea {
    #[allow(dead_code)]
    id: String,
    name: String,
    location_url: String,
}

#[derive(Debug, Deserialize)]
struct ResourceAreasResponse {
    value: Vec<ResourceArea>,
}

/// Package metadata returned by the packaging endpoint.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMetadata {
    /// Package version string.
    pub version: String,
    /// Blob ID of the dedup manifest.
    pub manifest_id: String,
    /// Blob ID of the super-root node.
    pub super_root_id: String,
    /// Total package size in bytes.
    pub package_size: u64,
}

/// A file entry in the dedup manifest.
#[derive(Debug, Deserialize)]
pub struct ManifestItem {
    /// File path within the package (e.g. "/myfile.bin").
    pub path: String,
    /// Reference to the dedup blob for this file.
    pub blob: DedupBlobRef,
}

/// A reference to a dedup blob (hash ID + logical size).
#[derive(Debug, Deserialize)]
pub struct DedupBlobRef {
    /// Hex-encoded blob ID with type suffix ("01" = content, "02" = node).
    pub id: String,
    /// Decompressed size in bytes.
    pub size: u64,
}

/// Parsed manifest listing all files in a package.
#[derive(Debug, Deserialize)]
pub struct Manifest {
    /// The files contained in the package.
    pub items: Vec<ManifestItem>,
}

// --- Client ---

/// Client for downloading Universal Packages from Azure Artifacts.
#[derive(Clone)]
pub struct Client {
    credential: crate::Credential,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}

/// Builder for creating an artifacts download [`Client`].
#[derive(Clone)]
pub struct ClientBuilder {
    credential: crate::Credential,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}

impl ClientBuilder {
    /// Create a new `ClientBuilder`.
    #[must_use]
    pub fn new(credential: crate::Credential) -> Self {
        Self {
            credential,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }

    /// Set the authentication scopes.
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }

    /// Set the retry options.
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }

    /// Set the transport options.
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }

    /// Build the [`Client`].
    pub fn build(self) -> Client {
        let scopes = self
            .scopes
            .unwrap_or_else(|| vec![crate::ADO_SCOPE.to_string()]);
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            self.options,
            Vec::new(),
            Vec::new(),
        );
        Client {
            credential: self.credential,
            scopes,
            pipeline,
        }
    }
}

impl Client {
    /// Create a new `ClientBuilder`.
    #[must_use]
    pub fn builder(credential: crate::Credential) -> ClientBuilder {
        ClientBuilder::new(credential)
    }

    /// Get the authorization header value for the current credential.
    async fn auth_header(&self) -> Result<String> {
        let scopes: Vec<&str> = self.scopes.iter().map(String::as_str).collect();
        self.credential
            .http_authorization_header(&scopes)
            .await?
            .ok_or_else(|| Error::message(ErrorKind::Credential, "No credential configured"))
    }

    /// Send a request through the pipeline.
    async fn send(&self, request: &mut Request) -> Result<azure_core::Response> {
        let context = azure_core::Context::default();
        self.pipeline.send(&context, request).await
    }

    /// Send an authenticated GET request and parse the JSON response.
    async fn get_json<T: serde::de::DeserializeOwned>(&self, url: Url) -> Result<T> {
        let mut req = Request::new(url, Method::Get);
        let auth = self.auth_header().await?;
        req.insert_header(headers::AUTHORIZATION, HeaderValue::from(auth));
        req.insert_header(
            headers::ACCEPT,
            HeaderValue::from("application/json; api-version=7.1-preview.1"),
        );
        req.insert_header("x-tfs-fedauthredirect", HeaderValue::from("Suppress"));
        req.set_body(azure_core::EMPTY_BODY);

        let resp = self.send(&mut req).await?;
        let bytes = resp.into_body().collect().await?;
        serde_json::from_slice(&bytes).map_err(|e| {
            Error::full(
                ErrorKind::DataConversion,
                e,
                format!(
                    "Failed to deserialize response:\n{}",
                    String::from_utf8_lossy(&bytes)
                ),
            )
        })
    }

    /// Send an unauthenticated GET request and return the raw bytes.
    async fn get_bytes(&self, url: Url) -> Result<Vec<u8>> {
        let mut req = Request::new(url, Method::Get);
        req.set_body(azure_core::EMPTY_BODY);
        let resp = self.send(&mut req).await?;
        let bytes = resp.into_body().collect().await?;
        Ok(bytes.to_vec())
    }

    // --- Service discovery ---

    /// Discover Azure DevOps service URLs via the ResourceAreas API.
    /// Returns a map of service name -> location URL.
    pub async fn discover_services(
        &self,
        organization: &str,
    ) -> Result<HashMap<String, String>> {
        let url = Url::parse(&format!(
            "https://dev.azure.com/{}/_apis/ResourceAreas",
            organization
        ))
        .context(ErrorKind::DataConversion, "invalid organization URL")?;

        let areas: ResourceAreasResponse = self.get_json(url).await?;
        let map: HashMap<String, String> = areas
            .value
            .into_iter()
            .map(|a| (a.name.to_lowercase(), a.location_url))
            .collect();
        Ok(map)
    }

    /// Find the packages service URL from discovered services.
    pub fn find_packages_url(
        services: &HashMap<String, String>,
        organization: &str,
    ) -> String {
        services
            .values()
            .find(|url| url.contains("pkgs."))
            .cloned()
            .unwrap_or_else(|| format!("https://pkgs.dev.azure.com/{}", organization))
    }

    /// Find the blob/dedup service URL from discovered services.
    pub fn find_blob_url(services: &HashMap<String, String>) -> Result<String> {
        services
            .get("dedup")
            .cloned()
            .ok_or_else(|| {
                Error::message(
                    ErrorKind::Other,
                    "Could not find 'dedup' service in ResourceAreas",
                )
            })
    }

    // --- Package metadata ---

    /// Get package download metadata from the packaging endpoint.
    pub async fn get_package_metadata(
        &self,
        packages_url: &str,
        project: &str,
        feed: &str,
        name: &str,
        version: &str,
    ) -> Result<PackageMetadata> {
        let mut url = Url::parse(&format!(
            "{}/{}/_packaging/{}/upack/packages/{}/versions/{}",
            packages_url.trim_end_matches('/'),
            project,
            feed,
            name,
            version,
        ))
        .context(ErrorKind::DataConversion, "invalid package metadata URL")?;

        url.query_pairs_mut().append_pair("intent", "Download");
        self.get_json(url).await
    }

    // --- Dedup blob operations ---

    /// Resolve dedup blob IDs to download URLs via the dedup service.
    pub async fn resolve_blob_urls(
        &self,
        blob_service_url: &str,
        blob_ids: &[String],
    ) -> Result<HashMap<String, String>> {
        let mut url = Url::parse(&format!(
            "{}/_apis/dedup/urls",
            blob_service_url.trim_end_matches('/')
        ))
        .context(ErrorKind::DataConversion, "invalid dedup URL")?;

        url.query_pairs_mut().append_pair("allowEdge", "true");

        let mut req = Request::new(url, Method::Post);
        let auth = self.auth_header().await?;
        req.insert_header(headers::AUTHORIZATION, HeaderValue::from(auth));
        req.insert_header(
            headers::CONTENT_TYPE,
            HeaderValue::from("application/json; charset=utf-8; api-version=1.0-preview"),
        );
        req.insert_header(
            headers::ACCEPT,
            HeaderValue::from("application/json; api-version=1.0"),
        );
        req.insert_header("x-tfs-fedauthredirect", HeaderValue::from("Suppress"));
        let body = azure_core::to_json(blob_ids)?;
        req.set_body(body);

        let resp = self.send(&mut req).await?;
        let bytes = resp.into_body().collect().await?;
        serde_json::from_slice(&bytes).map_err(|e| {
            Error::full(
                ErrorKind::DataConversion,
                e,
                "Failed to parse blob URL response",
            )
        })
    }

    /// Download a blob from a SAS URL (no auth required).
    pub async fn download_blob(&self, url: &str) -> Result<Vec<u8>> {
        let parsed = Url::parse(url)
            .context(ErrorKind::DataConversion, "invalid blob download URL")?;
        self.get_bytes(parsed).await
    }

    // --- Manifest parsing ---

    /// Parse the dedup manifest blob (JSON) to extract file entries.
    pub fn parse_manifest(data: &[u8]) -> Result<Manifest> {
        serde_json::from_slice(data).map_err(|e| {
            Error::full(ErrorKind::DataConversion, e, "Failed to parse manifest JSON")
        })
    }

    /// Parse a dedup node blob (binary format) to extract chunk references.
    ///
    /// A dedup node (ID ending in "02") contains references to child blobs.
    /// The binary format is:
    /// - 4-byte header
    /// - N entries of: 4-byte metadata + 32-byte hash
    ///
    /// Content chunk IDs are formed by hex-encoding the 32-byte hash
    /// and appending "01" (content type marker).
    pub fn parse_dedup_node(data: &[u8]) -> Result<Vec<String>> {
        const HEADER_SIZE: usize = 4;
        const HASH_SIZE: usize = 32;
        const METADATA_SIZE: usize = 4;
        const ENTRY_SIZE: usize = METADATA_SIZE + HASH_SIZE;

        if data.len() < HEADER_SIZE + ENTRY_SIZE {
            return Err(Error::message(
                ErrorKind::DataConversion,
                format!(
                    "Dedup node blob too small: {} bytes (minimum {})",
                    data.len(),
                    HEADER_SIZE + ENTRY_SIZE
                ),
            ));
        }

        let data_portion = data.len() - HEADER_SIZE;
        if data_portion % ENTRY_SIZE != 0 {
            return Err(Error::message(
                ErrorKind::DataConversion,
                format!(
                    "Dedup node blob has unexpected size: {} bytes \
                     (data portion {} is not a multiple of entry size {})",
                    data.len(),
                    data_portion,
                    ENTRY_SIZE
                ),
            ));
        }

        let num_entries = data_portion / ENTRY_SIZE;
        let mut chunk_ids = Vec::with_capacity(num_entries);

        for i in 0..num_entries {
            let offset = HEADER_SIZE + i * ENTRY_SIZE;
            let hash_bytes = &data[offset + METADATA_SIZE..offset + METADATA_SIZE + HASH_SIZE];
            let hex_hash: String = hash_bytes.iter().map(|b| format!("{:02X}", b)).collect();
            chunk_ids.push(format!("{}01", hex_hash));
        }

        if chunk_ids.is_empty() {
            return Err(Error::message(
                ErrorKind::DataConversion,
                format!(
                    "No chunk references found in dedup node blob ({} bytes)",
                    data.len()
                ),
            ));
        }
        Ok(chunk_ids)
    }

    // --- High-level download ---

    /// Download a universal package to the specified output directory.
    ///
    /// Performs the full download protocol: service discovery, metadata fetch,
    /// manifest download, chunk download with decompression, and file assembly.
    pub async fn download_universal_package(
        &self,
        organization: &str,
        project: &str,
        feed: &str,
        name: &str,
        version: &str,
        output_path: &Path,
    ) -> Result<PackageMetadata> {
        // Step 1: Discover service URLs
        let services = self.discover_services(organization).await?;
        let packages_url = Self::find_packages_url(&services, organization);
        let blob_service_url = Self::find_blob_url(&services)?;

        // Step 2: Get package metadata
        let metadata = self
            .get_package_metadata(&packages_url, project, feed, name, version)
            .await?;

        // Step 3: Download the manifest blob
        let manifest_urls = self
            .resolve_blob_urls(&blob_service_url, &[metadata.manifest_id.clone()])
            .await?;
        let manifest_url = manifest_urls.get(&metadata.manifest_id).ok_or_else(|| {
            Error::message(ErrorKind::Other, "Manifest URL not found in response")
        })?;
        let manifest_data = self.download_blob(manifest_url).await?;
        let manifest = Self::parse_manifest(&manifest_data)?;

        // Step 4: Create output directory
        std::fs::create_dir_all(output_path).map_err(|e| {
            Error::full(
                ErrorKind::Io,
                e,
                format!("Failed to create output directory: {:?}", output_path),
            )
        })?;

        // Step 5: Download each file
        for item in &manifest.items {
            let file_root_urls = self
                .resolve_blob_urls(&blob_service_url, &[item.blob.id.clone()])
                .await?;
            let file_root_url = file_root_urls.get(&item.blob.id).ok_or_else(|| {
                Error::message(ErrorKind::Other, "File root URL not found")
            })?;
            let file_root_data = self.download_blob(file_root_url).await?;

            let is_node = item.blob.id.ends_with("02");

            let file_data = if is_node {
                let chunk_ids = Self::parse_dedup_node(&file_root_data)?;
                let chunk_urls = self
                    .resolve_blob_urls(&blob_service_url, &chunk_ids)
                    .await?;

                let mut file_data = Vec::with_capacity(item.blob.size as usize);
                for chunk_id in &chunk_ids {
                    let chunk_url = chunk_urls.get(chunk_id).ok_or_else(|| {
                        Error::message(
                            ErrorKind::Other,
                            format!("Chunk URL not found for {}", chunk_id),
                        )
                    })?;
                    let chunk_data = self.download_blob(chunk_url).await?;
                    let decompressed = decompress_chunk(&chunk_data)?;
                    file_data.extend_from_slice(&decompressed);
                }
                file_data
            } else {
                file_root_data
            };

            let relative_path = item.path.trim_start_matches('/');
            let file_path = output_path.join(relative_path);
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    Error::full(
                        ErrorKind::Io,
                        e,
                        format!("Failed to create directory: {:?}", parent),
                    )
                })?;
            }
            let mut file = std::fs::File::create(&file_path).map_err(|e| {
                Error::full(
                    ErrorKind::Io,
                    e,
                    format!("Failed to create file: {:?}", file_path),
                )
            })?;
            file.write_all(&file_data).map_err(|e| {
                Error::full(ErrorKind::Io, e, "Failed to write file data")
            })?;
        }

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- find_packages_url ---

    #[test]
    fn test_find_packages_url_with_pkgs_service() {
        let mut services = HashMap::new();
        services.insert("packaging".to_string(), "https://pkgs.dev.azure.com/myorg/".to_string());
        services.insert("dedup".to_string(), "https://vsblob.dev.azure.com/myorg/".to_string());

        let url = Client::find_packages_url(&services, "myorg");
        assert!(url.contains("pkgs."));
    }

    #[test]
    fn test_find_packages_url_fallback() {
        let services = HashMap::new();
        let url = Client::find_packages_url(&services, "myorg");
        assert_eq!(url, "https://pkgs.dev.azure.com/myorg");
    }

    // --- find_blob_url ---

    #[test]
    fn test_find_blob_url_found() {
        let mut services = HashMap::new();
        services.insert("dedup".to_string(), "https://vsblob.dev.azure.com/myorg/".to_string());

        let url = Client::find_blob_url(&services).unwrap();
        assert_eq!(url, "https://vsblob.dev.azure.com/myorg/");
    }

    #[test]
    fn test_find_blob_url_missing() {
        let services = HashMap::new();
        assert!(Client::find_blob_url(&services).is_err());
    }

    // --- parse_manifest ---

    #[test]
    fn test_parse_manifest_valid() {
        let json = br#"{"items":[{"path":"/file1.txt","blob":{"id":"ABC01","size":100}},{"path":"/dir/file2.bin","blob":{"id":"DEF02","size":200}}]}"#;
        let manifest = Client::parse_manifest(json).unwrap();
        assert_eq!(manifest.items.len(), 2);
        assert_eq!(manifest.items[0].path, "/file1.txt");
        assert_eq!(manifest.items[0].blob.id, "ABC01");
        assert_eq!(manifest.items[0].blob.size, 100);
        assert_eq!(manifest.items[1].path, "/dir/file2.bin");
        assert_eq!(manifest.items[1].blob.id, "DEF02");
        assert_eq!(manifest.items[1].blob.size, 200);
    }

    #[test]
    fn test_parse_manifest_empty_items() {
        let json = br#"{"items":[]}"#;
        let manifest = Client::parse_manifest(json).unwrap();
        assert!(manifest.items.is_empty());
    }

    #[test]
    fn test_parse_manifest_invalid_json() {
        assert!(Client::parse_manifest(b"not json").is_err());
    }

    #[test]
    fn test_parse_manifest_missing_field() {
        let json = br#"{"items":[{"path":"/f"}]}"#;
        assert!(Client::parse_manifest(json).is_err());
    }

    // --- parse_dedup_node ---

    #[test]
    fn test_parse_dedup_node_single_entry() {
        // 4-byte header + 1 entry (4-byte meta + 32-byte hash)
        let mut data = vec![0x00, 0x01, 0x00, 0x00]; // header
        data.extend_from_slice(&[0x00; 4]); // metadata
        let hash: Vec<u8> = (0..32).collect();
        data.extend_from_slice(&hash);

        let ids = Client::parse_dedup_node(&data).unwrap();
        assert_eq!(ids.len(), 1);
        let expected: String = hash.iter().map(|b| format!("{:02X}", b)).collect::<String>() + "01";
        assert_eq!(ids[0], expected);
    }

    #[test]
    fn test_parse_dedup_node_two_entries() {
        let mut data = vec![0x00, 0x01, 0x00, 0x00]; // header
        // Entry 1
        data.extend_from_slice(&[0x00; 4]); // metadata
        let hash1: Vec<u8> = (0..32).collect();
        data.extend_from_slice(&hash1);
        // Entry 2
        data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // metadata
        let hash2: Vec<u8> = (32..64).collect();
        data.extend_from_slice(&hash2);

        let ids = Client::parse_dedup_node(&data).unwrap();
        assert_eq!(ids.len(), 2);
        assert!(ids[0].ends_with("01"));
        assert!(ids[1].ends_with("01"));
    }

    #[test]
    fn test_parse_dedup_node_too_small() {
        assert!(Client::parse_dedup_node(&[0; 10]).is_err());
    }

    #[test]
    fn test_parse_dedup_node_invalid_size() {
        // 4 header + 37 bytes (not a multiple of 36)
        let data = vec![0u8; 4 + 37];
        assert!(Client::parse_dedup_node(&data).is_err());
    }

    #[test]
    fn test_parse_dedup_node_chunk_ids_are_content_type() {
        let mut data = vec![0x00; 4]; // header
        data.extend_from_slice(&[0x00; 4]); // metadata
        data.extend_from_slice(&[0xFF; 32]); // all-FF hash

        let ids = Client::parse_dedup_node(&data).unwrap();
        assert_eq!(ids.len(), 1);
        // Should end with "01" (content type), not "02" (node type)
        assert!(ids[0].ends_with("01"));
        assert_eq!(ids[0].len(), 66); // 64 hex chars + "01"
    }
}
