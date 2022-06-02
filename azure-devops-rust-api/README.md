# azure-devops-rust-api

Rust interface to Azure DevOps API.

## Overview

azure-devops-rust-api implements a Rust interface to the Azure DevOps REST API (version 7.1).

## Usage

Individual components in the API are enabled via Rust [`features`](https://doc.rust-lang.org/cargo/reference/features.html).

See the `features` section of [Cargo.toml](Cargo.toml) for the full list of features.

Example application `Cargo.toml` dependency spec showing how to specify desired features:

```toml
[dependencies]
...
azure-devops-rust-api = { version = "0.1.0", features = ["git", "pipelines"] }
```

## Examples

See [examples](examples) directory.

Define environment variables:

```sh
export ADO_SERVICE_ENDPOINT=https://dev.azure.com
export ADO_ORGANIZATION=msazuredev
export ADO_PROJECT=AzureForOperators
```

Note that you need to authenticate via `az login` before running the examples.

```sh
az login
cargo run --example repo_get --features="git" <repo-name>
```

## Useful links

- [Azure DevOps REST API Reference](https://docs.microsoft.com/en-us/rest/api/azure/devops/)
- [vsts-rest-api-specs](https://github.com/MicrosoftDocs/vsts-rest-api-specs)
