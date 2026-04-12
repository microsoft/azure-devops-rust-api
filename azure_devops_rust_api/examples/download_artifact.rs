// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// Download a Universal Package from Azure Artifacts.
//
// Usage:
//   export ADO_ORGANIZATION=<org>
//   export ADO_PROJECT=<project>
//   cargo run --example download_artifact --features="artifacts_download" -- \
//       --feed <feed> --name <package-name> --version <version> --path <output-dir>

use anyhow::{Context, Result};
use azure_devops_rust_api::artifacts_download;
use std::env;
use std::path::PathBuf;

mod utils;

// --- CLI argument parsing ---

struct Args {
    organization: String,
    project: String,
    feed: String,
    name: String,
    version: String,
    path: PathBuf,
}

fn parse_args() -> Result<Args> {
    let organization = env::var("ADO_ORGANIZATION").context("Must define ADO_ORGANIZATION")?;
    let project = env::var("ADO_PROJECT").context("Must define ADO_PROJECT")?;

    let args: Vec<String> = env::args().collect();
    let mut feed = None;
    let mut name = None;
    let mut version = None;
    let mut path = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--feed" => {
                feed = Some(args.get(i + 1).context("--feed requires a value")?.clone());
                i += 2;
            }
            "--name" => {
                name = Some(args.get(i + 1).context("--name requires a value")?.clone());
                i += 2;
            }
            "--version" => {
                version = Some(
                    args.get(i + 1)
                        .context("--version requires a value")?
                        .clone(),
                );
                i += 2;
            }
            "--path" => {
                path = Some(args.get(i + 1).context("--path requires a value")?.clone());
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(Args {
        organization,
        project,
        feed: feed.context("--feed is required")?,
        name: name.context("--name is required")?,
        version: version.context("--version is required")?,
        path: PathBuf::from(path.context("--path is required")?),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_args()?;
    let credential = utils::get_credential()?;

    println!(
        "Downloading Universal Package: {}@{} from {}/{}",
        args.name, args.version, args.organization, args.project
    );

    let client = artifacts_download::ClientBuilder::new(credential).build();

    let metadata = client
        .download_universal_package(
            &args.organization,
            &args.project,
            &args.feed,
            &args.name,
            &args.version,
            &args.path,
        )
        .await?;

    println!(
        "Downloaded {} v{} ({} bytes) to {:?}",
        args.name, metadata.version, metadata.package_size, args.path
    );

    Ok(())
}
