# Copyright (c) Microsoft Corporation.
# Licensed under the MIT License.

# Exit on error
$ErrorActionPreference = "Stop"

# Fetch the vsts-rest-api-specs submodule
Write-Host "Fetch vsts-rest-api-specs"
git submodule init
git submodule update

# Patch the API
Write-Host "Create vsts-rest-api-specs.patched"
Remove-Item -Recurse -Force vsts-rest-api-specs.patched -ErrorAction Ignore
Set-Location vsts-api-patcher
cargo run --release
Set-Location ..

# Autogen the Rust crate
Write-Host "Autogen Rust crate"
Remove-Item -Recurse -Force azure_devops_rust_api/target -ErrorAction Ignore
Set-Location autorust
cargo run --bin ado-autorust --release
Set-Location ..

# Build Rust crate
Set-Location azure_devops_rust_api
Write-Host "Build azure_devops_rust_api"
cargo build --all-features
Set-Location ..

Write-Host "Done"
