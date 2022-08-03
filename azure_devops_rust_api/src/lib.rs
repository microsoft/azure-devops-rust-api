// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! ## Overview
//!
//! `azure_devops_rust_api` implements a Rust interface to the Azure DevOps REST API (version 7.1).
//!
//! The crate is autogenerated from the [Azure DevOps OpenAPI spec](https://github.com/MicrosoftDocs/vsts-rest-api-specs).
//!
//! For usage and examples see the [`azure_devops_rust_api` repo](https://github.com/microsoft/azure-devops-rust-api/tree/main/azure_devops_rust_api):
//!
//! - [README.md](https://github.com/microsoft/azure-devops-rust-api/blob/main/azure_devops_rust_api/README.md)
//! - [examples](https://github.com/microsoft/azure-devops-rust-api/tree/main/azure_devops_rust_api/examples)

#[cfg(feature = "accounts")]
pub mod accounts;
#[cfg(feature = "artifacts")]
pub mod artifacts;
#[cfg(feature = "artifacts_package_types")]
pub mod artifacts_package_types;
#[cfg(feature = "audit")]
pub mod audit;
#[cfg(feature = "build")]
pub mod build;
#[cfg(feature = "clt")]
pub mod clt;
#[cfg(feature = "core")]
pub mod core;
#[cfg(feature = "dashboard")]
pub mod dashboard;
#[cfg(feature = "distributed_task")]
pub mod distributed_task;
#[cfg(feature = "extension_management")]
pub mod extension_management;
#[cfg(feature = "git")]
pub mod git;
#[cfg(feature = "graph")]
pub mod graph;
#[cfg(feature = "hooks")]
pub mod hooks;
#[cfg(feature = "ims")]
pub mod ims;
#[cfg(feature = "member_entitlement_management")]
pub mod member_entitlement_management;
#[cfg(feature = "operations")]
pub mod operations;
#[cfg(feature = "permissions_report")]
pub mod permissions_report;
#[cfg(feature = "pipelines")]
pub mod pipelines;
#[cfg(feature = "policy")]
pub mod policy;
#[cfg(feature = "processadmin")]
pub mod processadmin;
#[cfg(feature = "processes")]
pub mod processes;
#[cfg(feature = "profile")]
pub mod profile;
#[cfg(feature = "release")]
pub mod release;
#[cfg(feature = "search")]
pub mod search;
#[cfg(feature = "security")]
pub mod security;
#[cfg(feature = "service_endpoint")]
pub mod service_endpoint;
#[cfg(feature = "status")]
pub mod status;
#[cfg(feature = "symbol")]
pub mod symbol;
#[cfg(feature = "test")]
pub mod test;
#[cfg(feature = "test_plan")]
pub mod test_plan;
#[cfg(feature = "test_results")]
pub mod test_results;
#[cfg(feature = "tfvc")]
pub mod tfvc;
#[cfg(feature = "token_admin")]
pub mod token_admin;
#[cfg(feature = "wiki")]
pub mod wiki;
#[cfg(feature = "wit")]
pub mod wit;
#[cfg(feature = "work")]
pub mod work;

mod auth;
pub use auth::Credential;
