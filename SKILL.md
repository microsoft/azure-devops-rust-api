# Skills

## Creating a release

### 1. Ensure changes are on main

All changes for the release (dependency updates, bug fixes, etc.) should already be merged to `main`.

### 2. Create a release branch

```sh
git checkout main
git pull
git checkout -b release-X.Y.0
```

### 3. Bump the crate version

Edit `azure_devops_rust_api/Cargo.toml`:
- Update `version = "X.Y.0"`

### 4. Update the README version

Edit `azure_devops_rust_api/README.md`:
- Update the example `Cargo.toml` snippet: `azure_devops_rust_api = { version = "X.Y.0", ... }`

### 5. Update the CHANGELOG

Edit `CHANGELOG.md`:
- Insert `## [X.Y.0]` between `## [Unreleased]` and the existing `### Changes` section (the changes stay, they just move under the new version heading)
- At the bottom of the file, update the version comparison links:
  - Change `[Unreleased]` to compare from the new version: `compare/X.Y.0...HEAD`
  - Add a new link: `[X.Y.0]: https://github.com/microsoft/azure-devops-rust-api/compare/PREV.0...X.Y.0`

### 6. Commit and push

```sh
git add CHANGELOG.md azure_devops_rust_api/Cargo.toml azure_devops_rust_api/README.md
git commit -m "Release X.Y.0"
git push -u origin release-X.Y.0
```

### 7. Create a PR and merge

Create a PR to `main`. After merging, the crate can be published to crates.io.

---

# Updating azure_core and azure_identity

## Overview

The `azure_devops_rust_api` crate depends on `azure_core` (runtime) and `azure_identity` (dev/examples). These are updated periodically to track the latest Azure SDK for Rust releases.

## Process

### 1. Find the latest versions

Check crates.io for the latest versions. They are released in lockstep and should use the same version.
- https://crates.io/crates/azure_core
- https://crates.io/crates/azure_identity

Watch out for yanked versions (e.g. 0.30.0 was yanked, 0.30.1 replaced it).

### 2. Update version strings

Edit `azure_devops_rust_api/Cargo.toml`:
- `azure_core = { version = "X.Y", ... }` in `[dependencies]`
- `azure_identity = "X.Y"` in `[dev-dependencies]`

### 3. Build and fix

```sh
cargo check --all-features
cargo check --all-features --examples
```

If there are no compilation errors, you're done with code changes.

If there are errors, fixes may be needed in two places:

**Code generator** (for generated code errors — do NOT edit generated files directly):
- `autorust/codegen/src/codegen_operations/create_client_and_builder.rs` — Pipeline, Transport, RetryOptions usage
- `autorust/codegen/src/codegen_operations/request_builder_send.rs` — Pipeline::send() calls, api-version parameter
- `autorust/codegen/src/codegen_operations/request_builder_into_future.rs` — into_body()/into_model() calls
- `autorust/codegen/src/codegen_operations/response_code.rs` — Response type definitions

After fixing codegen, regenerate with `./build.sh`, then re-check.

**Hand-written source** (fix directly):
- `azure_devops_rust_api/src/auth.rs` — TokenCredential, error handling
- `azure_devops_rust_api/src/date_time.rs` — error handling
- `azure_devops_rust_api/src/telemetry.rs` — Policy trait, Response types

### 4. Update CHANGELOG.md

Add an entry under `## [Unreleased]`:

```markdown
### Changes

- Update `azure_core` and `azure_identity` to X.Y.
```

If code changes were needed, list them (see the 0.33.0 entry for a detailed example).

### 5. Branch, commit, push

```sh
git checkout -b azure_core-X.Y
git add CHANGELOG.md azure_devops_rust_api/Cargo.toml [any other changed files]
git commit -m "Update azure_core, azure_identity to X.Y"
git push -u origin azure_core-X.Y
```

The README (`azure_devops_rust_api/README.md`) version string is only updated at release time, not during dependency updates.

## Key azure_core types used by this crate

These are the areas most likely to break across versions:
- `azure_core::http::Pipeline` — HTTP pipeline for sending requests
- `azure_core::http::Request` / `RawResponse` — request/response types
- `azure_core::http::RetryOptions` / `Transport` — client configuration
- `azure_core::http::Context` — request context
- `azure_core::http::policies::Policy` — pipeline policies (telemetry.rs)
- `azure_core::credentials::TokenCredential` — authentication (auth.rs)
- `azure_core::error::{Error, ErrorKind, Result}` — error types
