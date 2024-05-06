# AutoRust [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)

A code generator similar to [AutoRest](https://github.com/azure/autorest), but is written in Rust to generate Rust code.

This is a fork of [`autorust` in azure-sdk-for-rust](https://github.com/Azure/azure-sdk-for-rust/tree/main/services/autorust), and has a few modifications to work for the Azure DevOps REST API specifications.

## Building

To build the autorust code:

```sh
cargo build
```

## Running

To autogenerate the `azure_devops_rust_api` code:

```sh
cargo run --bin ado-autorust --release
```

