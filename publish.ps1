# Copyright (c) Microsoft Corporation.
# Licensed under the MIT License.

# Exit on error
$ErrorActionPreference = "Stop"

Write-Host "Publish crate to crates.io"
Set-Location azure_devops_rust_api
cargo publish --verbose --all-features
Set-Location ..

Write-Host "Done"