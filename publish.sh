#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation.
# Licensed under the MIT License.

# Exit on error
set -ex

echo "Publish crate to crates.io"
cd azure_devops_rust_api
cargo publish --verbose --all-features
cd ..

echo "Done"
