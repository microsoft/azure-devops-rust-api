// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

mod patcher;

use patcher::Patcher;

// Directories containing the original and patched VSTS REST API specs.
const ORIGINAL_VSTS_SPECS_DIR: &str = "vsts-rest-api-specs";
const PATCHED_VSTS_SPECS_DIR: &str = "vsts-rest-api-specs.patched";

// JSON pretty-printing indent size.
const JSON_INDENT: u16 = 2;

// Return true if the specified `entry` appears to be a an OpenAPI specification filename.
fn is_spec(entry: &DirEntry) -> bool {
    let path = entry.path().to_string_lossy().to_string();
    path.ends_with(".json") && !path.contains("httpExamples")
}

// Performs preprocessing of spec text immediately after loading.
fn preprocess_spec(spec_path: &Path, data: String) -> String {
    if spec_path.ends_with("workItemTracking.json") {
        // Fix up formatting of `$filter` query parameter - codegen fails with the $ prefix in the template.
        data.replace(
            "/{organization}/{project}/_apis/wit/queries?$filter={$filter}",
            "/{organization}/{project}/_apis/wit/queries?$filter={filter}",
        )
    } else {
        data
    }
}

// Strip BOM (Byte Order Mark) if present.
// https://en.wikipedia.org/wiki/Byte_order_mark
// Spoiler alert: It is present in vsts-rest-api-specs!
fn maybe_strip_bom(bytes: &[u8]) -> &[u8] {
    match bytes[..] {
        [0xEF, 0xBB, 0xBF, ..] => &bytes[3..],
        _ => bytes,
    }
}

// Return the path of the patched spec file.
//
// The patched spec files are written to a parallel directory tree
// where the original `/vsts-rst-api-specs/` path segment is replaced with
// `/vsts-rest-api-specs.patched/`.
fn patched_spec_path(spec_path: &Path) -> PathBuf {
    let new_spec_name = &spec_path.to_string_lossy().replace(
        &format!("/{ORIGINAL_VSTS_SPECS_DIR}"),
        &format!("/{PATCHED_VSTS_SPECS_DIR}/"),
    );

    Path::new(&new_spec_name).to_path_buf()
}

fn main() -> Result<()> {
    let spec_root_path = Path::new("..")
        .join(ORIGINAL_VSTS_SPECS_DIR)
        .join("specification");

    let spec_paths: Vec<PathBuf> = WalkDir::new(spec_root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_spec)
        .map(|dir_entry| dir_entry.path().to_path_buf())
        .collect();

    println!("Spec paths:\n{spec_paths:#?}");

    for spec_path in &spec_paths {
        let bytes = std::fs::read(spec_path)?;
        let bytes = maybe_strip_bom(&bytes);

        let data =
            std::str::from_utf8(bytes).context(format!("File is not valid UTF8: {spec_path:?}"))?;
        let data = preprocess_spec(spec_path, data.to_string());

        let mut json = json::parse(&data).context("Failed to parse JSON data: {spec_path:?}")?;

        // Apply patches to the JSON spec.
        let mut patcher = Patcher::new(spec_path);
        patcher.run(&mut json);

        // Format the patched JSON nicely.
        let json_data = json::stringify_pretty(json, JSON_INDENT);

        // Write the patched JSON to a new file.
        let new_spec_path = patched_spec_path(spec_path);
        let new_spec_dir = new_spec_path
            .parent()
            .context("Failed to get parent dir for {new_spec_path}")?;
        std::fs::create_dir_all(new_spec_dir)
            .context(format!("Failed to create {}", new_spec_dir.display()))?;

        let mut f = File::create(&new_spec_path)?;
        f.write_all(format!("{json_data}\n").as_bytes())?;
        println!("{spec_path:#?} -> {new_spec_path:#?}");
    }

    Ok(())
}
