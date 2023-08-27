#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation.
# Licensed under the MIT License.

# Exit on error
set -ex

# Fetch the vsts-rest-api-specs submodule
echo "Fetch vsts-rest-api-specs"
git submodule init
git submodule update

# Patch the API
echo "Crate vsts-rest-api-specs.patched"
rm -rf vsts-rest-api-specs.patched
cd vsts-api-patcher
cargo run --release
cd ..

# Autogen the Rust crate
echo "Autogen Rust crate"
rm -rf azure_devops_rust_api/target
cd autorust/codegen
cargo run --release
cd ../..

# Format and build Rust crate
cd azure_devops_rust_api
echo "Format azure_devops_rust_api"
cargo fmt
echo "Build azure_devops_rust_api"
cargo build --all-features
cd ..

echo "Done"
