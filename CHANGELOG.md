# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changes

- Upgrade `azure_core`, `azure_identity` to 0.18
  - Note: `AutoRefreshingTokenCredential` is no longer required (and has been removed from `azure_identity`), as token refreshing is now built in to each credential provider

### [0.14.3]

### Added

- New value added to git `ChangeType` enum: `edit, rename`

### [0.14.2]

### Added

- New value added to git `ChangeType` enum: `delete, sourceRename`
- New examples:
  - `git_repo_download_zip`
  - `build_source_providers_list`
- Updated example: `git_items_get` now includes downloading entire repo as a zip archive

## [0.14.1]

### Added

- Enable configuration of per-retry policies on all client builders:
  - new `ClientBuilder` method: `per_retry_policies(...)`
- New example: `work`

## [0.14.0]

### Added

- New example: `wit_wiql`

### Changes

- Upgrade `azure_core`, `azure_identity` to 0.17

## [0.13.0]

### Added

- Add support for WASM builds

## [0.12.0]

### Changes

- Upgrade `azure_core`, `azure_identity` to 0.16

## [0.11.0]

### Changes

- Upgrade `azure_core`, `azure_identity` to 0.15

## [0.10.0]

### Changes

- Update `GitPullRequestUpdateOptions` to add new field `autoCompleteSetBy`
- Update `GitCommitRef` to make `commitId` and `url` fields optional

## [0.9.0]

### Changes

- Upgrade `azure_core`, `azure_identity` to 0.14
- Update `vsts-rest-api-specs` to latest version
  - Picks up API and docs updates
  - Adds new modules:
    - `approvals_and_checks`
    - `favorite`
    - `security_roles`
  - Removes module:
    - `clt`

## [0.8.0]

### Fixed

- Added missing `IntoFuture` implementations for operations that return an empty response
- Upgrade `azure_core`, `azure_identity` to 0.13

## [0.7.7]

### Added

- New `tokens` module
- Upgrade `azure_core`, `azure_identity` to 0.12

## [0.7.6]

### Fixed

- Specify correct token scope for ADO by default
- Fixed multiple `policy` API issues

## [0.7.5]

### Fixed

- Fixed definition of git `PolicyConfiguration`
- Fixed pipeline `Run` required fields
  - Changed `finishedDate` and `result` to optional, as they are not present if run is in progress
- Change pipeline `Repository` fields to be optional: `id`, `type`

### Added

- Upgrade `azure_core`, `azure_identity` to 0.11
- New examples:
  - `member_entitlement_management`
  - `git_policy_config_list`

## [0.7.4]

### Fixed

- Fixed definition of wit `IdentityReference`

### Added

- New example: `wit_work_item_queries`
- New example: `test_plan`

## [0.7.3]

### Breaking change

- Wrapped `WorkItemTrackingResource` `links` in an `Option`

### Fixed

- Updated dependencies:
  - `azure_core` to 0.10
  - `azure_identity` to 0.10

### Added

- Renamed `wit` examples:
  - `wit` => `wit_work_item_get`
  - `wit_create_work_item` => `wit_work_item_create`
- Improved `wit_work_item_get` to display all work item relations
- Improved `wit_work_item_get` to demonstrate `get_work_items_batch()`

## [0.7.2]

### Fixed

- Fix creating and updating work items
  - New example: `wit_create_work_item`

## [0.7.1]

### Fixed

- Improved `RequestBuilder` documentation

## [0.7.0]

### Breaking change

- Implemented `std::future::IntoFuture` for request builder objects.
  - Removes need to call `into_future()` to finalize request builders.
    `await` will implicitly call the `IntoFuture::into_future()` method.
    This simplifies request calls.
  - `IntoFuture` support requires Rust 1.64.0 or higher, so this is now enforced
    by setting `rust-version` in `Cargo.toml`.

### Added

- Examples: created new `utils` module with common authentication code.
  - Updated example authentication code to use `AutoRefreshingTokenCredential`
    and `DefaultAzureCredentialBuilder`.

### Fixed

- Updated dependency versions.

## [0.6.2]

### Added

- New examples:
  - `git_items_list`
  - `git_items_get`
  - `build_list_sync`

- New features:
  - `enable_reqwest` [default]
  - `enable_reqwest_rustls`
  - Both of the above enable the corresponding feature in `azure_core`

## [0.6.1]

### Added

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

### Fixed

- Fix `distributedTask` `variableGroupProjectReferences` deserialization of `null` value
- Fix `extensionManagement` parsing of `flags` fields
  - Change type from an `enum` to a `String`, as field value is a comma-separated list

## [0.6.0]

### Breaking changes

- Revert all `GraphSubjectBase` fields to be wrapped in `Option`
- Upgrade `azure_core` to 0.5, `azure_identity` to 0.6
  - `AzureCliCredential` must now be created via `azure_identity::AzureCliCredential::new()`

### Fixed

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

### Fixed

- Fixed up `GitCommitRef` `change_counts` field type
- Added back some required `build` structure Option wrappers
- Fixed formatting of date-time URL parameters
- Added `ims` (Identity Management) example: `ims_query`
- Fixed Pull Request create function `git::pull_requests::create()`
- Added `git_pr_create` example

## [0.5.2]

### Fixed

- Implement custom date-time serde module to gracefully handle `0001-01-01T00:00:00`
- Fixed response types for git::commits::get_changes()
- If response deserialization fails, include response content in error

## [0.5.1]

### Breaking changes

- Removed Option wrappers on selected structs in `wit` and `status`

### Added

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

[Unreleased]: https://github.com/microsoft/azure-devops-rust-api/compare/0.14.3...HEAD
[0.14.3]: https://github.com/microsoft/azure-devops-rust-api/compare/0.14.2...0.14.3
[0.14.2]: https://github.com/microsoft/azure-devops-rust-api/compare/0.14.1...0.14.2
[0.14.1]: https://github.com/microsoft/azure-devops-rust-api/compare/0.14.0...0.14.1
[0.14.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.13.0...0.14.0
[0.13.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.12.0...0.13.0
[0.12.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.11.0...0.12.0
[0.11.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.10.0...0.11.0
[0.10.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.9.0...0.10.0
[0.9.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.8.0...0.9.0
[0.8.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.7.7...0.8.0
[0.7.7]: https://github.com/microsoft/azure-devops-rust-api/compare/0.7.6...0.7.7
[0.7.6]: https://github.com/microsoft/azure-devops-rust-api/compare/0.7.5...0.7.6
[0.7.5]: https://github.com/microsoft/azure-devops-rust-api/compare/0.7.4...0.7.5
[0.7.4]: https://github.com/microsoft/azure-devops-rust-api/compare/0.7.3...0.7.4
[0.7.3]: https://github.com/microsoft/azure-devops-rust-api/compare/0.7.2...0.7.3
[0.7.2]: https://github.com/microsoft/azure-devops-rust-api/compare/0.7.1...0.7.2
[0.7.1]: https://github.com/microsoft/azure-devops-rust-api/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.6.2...0.7.0
[0.6.2]: https://github.com/microsoft/azure-devops-rust-api/compare/0.6.1...0.6.2
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
