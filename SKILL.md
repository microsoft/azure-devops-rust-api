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
