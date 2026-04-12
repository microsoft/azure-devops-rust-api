# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo does

Auto-generates a Rust Azure DevOps API crate (`azure_devops_rust_api`) from the Azure DevOps OpenAPI spec (`vsts-rest-api-specs`, included as a git submodule). The pipeline is:

1. **`vsts-api-patcher`** — reads `vsts-rest-api-specs/`, patches known issues in the OpenAPI JSON, writes `vsts-rest-api-specs.patched/`
2. **`autorust`** — reads `vsts-rest-api-specs.patched/`, generates Rust source into `azure_devops_rust_api/src/`
3. **`azure_devops_rust_api`** — the generated crate, compiled and published to crates.io

## Commands

### Full build (patch → generate → compile)
```sh
./build.sh
```

### Build individual components
```sh
# Run patcher only (from repo root)
cd vsts-api-patcher && cargo run --release && cd ..

# Run code generator only (from repo root)
cd autorust && cargo run --bin ado-autorust --release && cd ..

# Build generated crate
cd azure_devops_rust_api && cargo build --all-features && cd ..
```

### Lint
```sh
cargo clippy --all-features -- --deny warnings
cargo clippy --all-features --examples -- --deny warnings
```

### Check (faster than build, no generated output)
```sh
cd azure_devops_rust_api
cargo check --all-features
cargo check --all-features --examples
```

### Publish to crates.io
```sh
./publish.sh
```

## Architecture

### Key distinction: generated vs. hand-written code

**Do NOT edit generated files directly** — they are overwritten by `./build.sh`:
- `azure_devops_rust_api/src/<module>/mod.rs` — generated API operations
- `azure_devops_rust_api/src/<module>/models.rs` — generated model types

**Hand-written source** (edit these directly):
- `azure_devops_rust_api/src/auth.rs` — `Credential` enum (PAT + OAuth)
- `azure_devops_rust_api/src/date_time.rs` — datetime serde helpers
- `azure_devops_rust_api/src/telemetry.rs` — telemetry pipeline policy
- `azure_devops_rust_api/src/headers.rs` — common header helpers
- `azure_devops_rust_api/src/artifacts_download/` — Universal Package download client (dedup protocol)

### Code generator internals (`autorust/`)

- `autorust/ado-autorust/src/main.rs` — entry point; maps each OpenAPI spec file to an output module
- `autorust/codegen/src/codegen_operations/` — generates API operation code (clients, builders, responses)
  - `create_client_and_builder.rs` — Pipeline/Transport/RetryOptions usage
  - `request_builder_send.rs` — Pipeline::send() calls
  - `request_builder_into_future.rs` — into_body()/into_model() calls
  - `response_code.rs` — Response type definitions
- `autorust/codegen/src/codegen_models.rs` — generates model structs/enums
- `autorust/autorust.toml` — config for boxed/optional/invalid properties (fixes recursive type issues)

### Patcher (`vsts-api-patcher/`)

- `src/main.rs` — walks spec files, calls patcher, writes patched JSON
- `src/patcher.rs` — all spec patches; patching logic keyed on spec filename

### Generated crate structure

Each Azure DevOps API area is a separate feature-gated module (e.g. `git`, `build`, `wit`). All features are listed in `azure_devops_rust_api/Cargo.toml`. API version is 7.1.

## Updating `azure_core` / `azure_identity`

See `SKILL.md` for the full process. Key files affected by breaking changes:
- Codegen: `autorust/codegen/src/codegen_operations/` files (then re-run `./build.sh`)
- Hand-written: `azure_devops_rust_api/src/auth.rs`, `telemetry.rs`, `date_time.rs`

## Releasing

See `SKILL.md` for the full process. Files to update: `azure_devops_rust_api/Cargo.toml` (version), `azure_devops_rust_api/README.md` (version in example snippet), `CHANGELOG.md`.

## Updating the OpenAPI spec submodule

```sh
git submodule update --remote vsts-rest-api-specs
git add .
git commit -m "Updated vsts-rest-api-specs to latest revision"
./build.sh
cargo clippy --all-features -- --deny warnings
cargo clippy --all-features --examples -- --deny warnings
```

## Commit message style

- Dependency updates: `Update azure_core, azure_identity to X.Y`
- Releases: `Release X.Y.0`
- No conventional commit prefixes
