// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Azure DevOps rate limit headers.
//!
//! For more information see [Azure DevOps Rate and usage limits](https://learn.microsoft.com/en-us/azure/devops/integrate/concepts/rate-limits?view=azure-devops).

use azure_core::http::headers::HeaderName;

/// A custom header indicating the service and type of threshold that was reached. Threshold types and service names might vary over time and without warning.
/// We recommend displaying this string to a human, but not relying on it for computation.
pub const X_RATELIMIT_RESOURCE: HeaderName = HeaderName::from_static("x-ratelimit-resource");

/// How long the request was delayed. Units: seconds with up to three decimal places (milliseconds).
pub const X_RATELIMIT_DELAY: HeaderName = HeaderName::from_static("x-ratelimit-delay");

/// Total number of TSTUs allowed before delays are imposed.
pub const X_RATELIMIT_LIMIT: HeaderName = HeaderName::from_static("x-ratelimit-limit");

/// Number of TSTUs remaining before being delayed. If requests are already being delayed or blocked, it's 0.
pub const X_RATELIMIT_REMAINING: HeaderName = HeaderName::from_static("x-ratelimit-remaining");

/// Time at which, if all resource consumption stopped immediately, tracked usage would return to 0 TSTUs. Expressed in Unix epoch time.
pub const X_RATELIMIT_RESET: HeaderName = HeaderName::from_static("x-ratelimit-reset");
