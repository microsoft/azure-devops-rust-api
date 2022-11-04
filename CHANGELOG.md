# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## Added

- New examples:
  - `git_items_list`
  - `git_items_get`

- New features:
  - `enable_reqwest` [default]
  - `enable_reqwest_rustls`
  - Both of the above enable the corresponding feature in `azure_core`

## [0.6.1]

## Added

- New examples:
  - `git_repo_get_raw_rsp`
  - `artifacts_list`
  - `build_list_continuation_token`
  - `release_logs`
  - `wiki_pages_create_or_update`
  - `search_repositories`
  - `permissions_report`
- Added new fields to `GitPullRequestCreateOptions`
  - `merge_options`
  - `completion_options`

## Fixed

- Fix `distributedTask` `variableGroupProjectReferences` deserialization of `null` value
- Fix `extensionManagement` parsing of `flags` fields
  - Change type from an `enum` to a `String`, as field value is a comma-separated list

## [0.6.0]

## Breaking changes

- Revert all `GraphSubjectBase` fields to be wrapped in `Option`
- Upgrade `azure_core` to 0.5, `azure_identity` to 0.6
  - `AzureCliCredential` must now be created via `azure_identity::AzureCliCredential::new()`

## Fixed

- Upgrade `autorust` code generator
  - New `send()` function on operations that enables access to full response details
    (headers, raw body data)
- Add missing `distributedTask` `ElasticPool` operations
- Patch `hooks` spec:
  - `InputValidation` fields `minValue` and `maxValue` need to be `number`/`float`
  - `Subscription` field `probationRetries` needs to be `integer`/`int32`
- New examples:
  - `code_search`
  - `hooks_list`
  - `ims_query`
  - `extension_management_list`
  - `test_rust_list`

## [0.5.3]

## Fixed

- Fixed up `GitCommitRef` `change_counts` field type
- Added back some required `build` structure Option wrappers
- Fixed formatting of date-time URL parameters
- Added `ims` (Identity Management) example: `ims_query`
- Fixed Pull Request create function `git::pull_requests::create()`
- Added `git_pr_create` example

## [0.5.2]

## Fixed

- Implement custom date-time serde module to gracefully handle `0001-01-01T00:00:00`
- Fixed response types for git::commits::get_changes()
- If response deserialization fails, include response content in error

## [0.5.1]

## Breaking changes

- Removed Option wrappers on selected structs in `wit` and `status`

## Added

- Example improvements:
  - New examples:
    - wit (work items)
    - status
    - client_pipeline_policy
    - policy
  - Updated examples to use `ClientBuilder`, eliminating default client arguments
  - Added logging via `env_logger`
- Added `Credential::Unauthenticated` to enable unauthenticated operations

## [0.5.0]

### Breaking changes

- Upgraded to latest `autorust` code generator from `azure-sdk-for-rust`
  - Removed `operation` modules - all operations move up one level in the module hierarchy.
  - Second-level client functions changed to have a `_client` suffix, e.g.
    - `client.repositories().get_repository(...)` => `client.repositories_client().get_repository(...)`.
  - New `options`(`azure_core::ClientOptions`) parameter required when creating clients, which allows the HTTP pipeline to be customised with middleware.
  - Fields declared in the spec as `date-time` format now parsed into Rust `time::OffsetDateTime` types rather than `String`.

## [0.4.0]

### Breaking change

- Updated some struct fields to remove Option wrappers to make the values easier to use.
- Updated some `links` fields to have a struct with known fields, rather than a Json `Value`.

### Added

- `pipeline_preview` example

## [0.3.1]

### Added

- API documentation: autogenerate builder function descriptions.

### Fixed

- Upgrade Azure SDK dependencies:
  - azure_core: 0.3
  - azure_identity: 0.4

## [0.3.0]

### Breaking change

- Move `Credential` definition from `auth` module to root, to separate it from all the
  feature modules. `auth` module is now private.
  - To migrate change `use azure_devops_rust_api::auth::Credential` to `use azure_devops_rust_api::Credential`

### Added

- API documentation: autogenerate function description and parameter descriptions from fields
  in the OpenAPI spec.

## [0.2.0]

### Breaking change

- Credentials now need to be provided via a new type `auth::Credential` that supports
  both PAT and TokenCredential types. For more details see examples and docs.rs.

### Added

- Support for PAT authentication

### Fixed

- Remove use of `unwrap()` from examples.

## [0.1.3] - 2022-07-29

### Fixed

- Remove `--no-deps` flag for docs.rs documentation generation.

## [0.1.2] - 2022-07-29

### Fixed

- Set `--no-deps` flag for docs.rs documentation generation.

## [0.1.1] - 2022-07-29

### Fixed

- Enable `all-features` flag for docs.rs documentation generation.

## [0.1.0] - 2022-07-29

- Initial release.

[Unreleased]: https://github.com/microsoft/azure-devops-rust-api/compare/0.6.1...HEAD
[0.6.1]: https://github.com/microsoft/azure-devops-rust-api/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.5.3...0.6.0
[0.5.3]: https://github.com/microsoft/azure-devops-rust-api/compare/0.5.2...0.5.3
[0.5.2]: https://github.com/microsoft/azure-devops-rust-api/compare/0.5.1...0.5.2
[0.5.1]: https://github.com/microsoft/azure-devops-rust-api/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/microsoft/azure-devops-rust-api/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.1.3...0.2.0
[0.1.3]: https://github.com/microsoft/azure-devops-rust-api/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/microsoft/azure-devops-rust-api/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/microsoft/azure-devops-rust-api/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/microsoft/azure-devops-rust-api/tree/0.1.0
