# Azure DevOps Rust API Generator

> This is an unofficial Azure DevOps Rust API crate generator.
> It is in early development and only a small subset of function has been tested, so there will be issues and breaking changes.

This repo auto-generates a Rust Azure DevOps API crate (`azure_devops_rust_api`) from the Azure DevOps OpenAPI spec [`vsts-rest-api-specs`](https://github.com/MicrosoftDocs/vsts-rest-api-specs).

![Status](https://github.com/microsoft/azure-devops-rust-api/actions/workflows/rust.yml/badge.svg)

## Build

- Checkout this repo
- Run `build.sh`
- The API crate is generated into the `azure-devops-rust-api` directory.

## Publishing

The generated crate is manually published to the public Rust crate registry ([crates.io](https://crates.io/)) as `azure_devops_rust_api`.

## Notes

- The Azure DevOps OpenAPI spec `vsts-rest-api-specs` is pulled in at build time via a git submodule.
- There are issues/bugs with the OpenAPI spec, so it is patched using `vsts-api-patcher` to generate `vsts-rest-api-specs.patched`, which is used for the code generation.
- The client code generation is done using a modified version of `autorust`, a component from [azure-sdk-for-rust](https://github.com/Azure/azure-sdk-for-rust).
  - `autorust` is MIT licensed
  - `autorust` includes an `openapi` module, which is also MIT licensed
- Generates crate for API version 7.1 (latest API version).

## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

## Trademarks

This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft 
trademarks or logos is subject to and must follow 
[Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general).
Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship.
Any use of third-party trademarks or logos are subject to those third-party's policies.
