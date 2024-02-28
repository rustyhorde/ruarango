// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Cursor Output Structs

use getset::Getters;
use serde::{Deserialize, Serialize};

/// Cursor metadata output
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct CursorMeta<T> {
    /// id of temporary cursor created on the server
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// An array of result documents (might be empty if query has no results)
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Vec<T>>,
    /// Extra information about the query result contained in its
    /// stats sub-attribute. For data-modification queries, the extra.stats
    /// sub-attribute will contain the number of modified documents and
    /// the number of documents that could not be modified due to an error if
    /// `ignore_errors` query option is specified
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<Extra>,
    /// The total number of result documents available (only available
    /// if the query was executed with the count attribute set)
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<usize>,
    /// The HTTP status code
    code: u16,
    /// A boolean flag indicating whether the query result was served
    /// from the query cache or not. If the query result is served from the query
    /// cache, the extra return attribute will not contain any stats sub-attribute
    /// and no profile sub-attribute.
    cached: bool,
    /// A boolean indicator whether there are more results available for
    /// the cursor on the server
    #[serde(rename = "hasMore")]
    has_more: bool,
    /// A flag to indicate that an error occurred
    error: bool,
}

/// Cursor metadata extra output
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct Extra {
    /// Query statistics
    stats: Stats,
    /// Any generated warnings,
    warnings: Vec<String>,
    /// Optional profile information
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<Profile>,
}

/// Cursor metadata extra stats output
#[derive(Clone, Copy, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct Stats {
    /// writes executed
    #[serde(rename = "writesExecuted")]
    writes_executed: usize,
    /// writes ignored
    #[serde(rename = "writesIgnored")]
    writes_ignored: usize,
    /// scanned full
    #[serde(rename = "scannedFull")]
    scanned_full: usize,
    /// scannned index
    #[serde(rename = "scannedIndex")]
    scanned_index: usize,
    /// filtered
    filtered: usize,
    /// HTTP requests
    #[serde(rename = "httpRequests")]
    http_requests: usize,
    /// execution time
    #[serde(rename = "executionTime")]
    execution_time: f64,
    /// peak memory usage
    #[serde(rename = "peakMemoryUsage")]
    peak_memory_usage: usize,
}

/// Extra profile information
#[derive(Clone, Copy, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct Profile {
    /// Initializing
    initializing: f64,
    /// parsing
    parsing: f64,
    /// optimizing ast
    #[serde(rename = "optimizing ast")]
    optimizing_ast: f64,
    /// loading collections
    #[serde(rename = "loading collections")]
    loading_collections: f64,
    /// instantiating plan
    #[serde(rename = "instantiating plan")]
    instantiating_plan: f64,
    /// optimizing plan
    #[serde(rename = "optimizing plan")]
    optimizing_plan: f64,
    /// executing
    executing: f64,
    /// finalizing
    finalizing: f64,
}
