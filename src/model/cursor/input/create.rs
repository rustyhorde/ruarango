// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Cursor Create Input Struct

use crate::{model::BuildUrl, Connection};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde::{Serialize as Ser, Serializer};
use std::collections::HashMap;

const BATCH_SIZE_ZERO_ERR: &str = "batch_size cannot be 0!";

/// Cursor creation configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Config {
    /// Contains the query string to be executed
    #[builder(setter(into))]
    query: String,
    /// key/value pairs representing the bind parameters.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "bindVars", skip_serializing_if = "Option::is_none")]
    bind_vars: Option<HashMap<String, String>>,
    /// Indicates whether the number of documents in the result set
    /// should be returned in the "count" attribute of the result.
    /// Calculating the "count" attribute might have a performance
    /// impact for some queries in the future so this option is
    /// turned off by default, and "count" is only returned when requested.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<bool>,
    /// Maximum number of result documents to be transferred from
    /// the server to the client in one roundtrip. If this attribute is
    /// not set, a server-controlled default value will be used.
    /// A `batch_size` value of 0 is disallowed.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "batchSize", skip_serializing_if = "Option::is_none")]
    batch_size: Option<usize>,
    /// Flag to determine whether the AQL query results cache
    /// shall be used. If set to false, then any query cache lookup
    /// will be skipped for the query. If set to true, it will lead
    /// to the query cache being checked for the query if the query
    /// cache mode is either on or demand.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<bool>,
    /// The maximum amount of memory (measured in bytes) that the
    /// query is allowed to use. If set, then the query will fail
    /// with error "resource limit exceeded" in case it allocates too
    /// much memory. A value of 0 indicates that there is no memory limit.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "memoryLimit", skip_serializing_if = "Option::is_none")]
    memory_limit: Option<usize>,
    /// The time-to-live for the cursor (in seconds). The cursor will be
    /// removed on the server automatically after the specified amount of
    /// time. This is useful to ensure garbage collection of cursors that
    /// are not fully fetched by clients. If not set, a server-defined
    /// value will be used (default: 30 seconds).
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<usize>,
    /// Additional cursor options
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Options>,
}

impl ConfigBuilder {
    fn validate(&self) -> std::result::Result<(), String> {
        self.batch_size.as_ref().map_or(Ok(()), |bs_opt| {
            if let Some(0) = bs_opt {
                Err(BATCH_SIZE_ZERO_ERR.into())
            } else {
                Ok(())
            }
        })
    }
}

impl BuildUrl for Config {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = base.to_string();
        conn.db_url()
            .join(&suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

/// The profile kind
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ProfileKind {
    /// Generate the profiling data only
    ProfileOnly,
    /// Generate executions stats per query plan node
    WithStats,
}

impl Ser for ProfileKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ProfileKind::ProfileOnly => serializer.serialize_i8(1),
            ProfileKind::WithStats => serializer.serialize_i8(2),
        }
    }
}

/// Cursor creation options
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Options {
    /// When set to true, the query will throw an exception and abort
    /// instead of producing a warning. This option should be used during
    /// development to catch potential issues early. When the attribute
    /// is set to false, warnings will not be propagated to exceptions
    /// and will be returned with the query result. There is also a server
    /// configuration option `--query.fail-on-warning` for setting the
    /// default value for `fail_on_warning` so it does not need to be set
    /// on a per-query level.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "failOnWarning")]
    fail_on_warning: Option<bool>,
    /// If set to [`ProfileOnly`](ProfileKind::ProfileOnly), then the additional
    /// query profiling information will be returned in the sub-attribute
    /// `profile` of the extra return attribute, if the query result is
    /// not served from the query cache.
    /// If set to [`WithStats`](ProfileKind::WithStats) the query will include
    /// execution stats per query plan node in sub-attribute `stats.nodes` of
    /// the extra return attribute. Additionally the query plan is returned
    /// in the sub-attribute `extra.plan`.
    #[builder(setter(strip_option), default)]
    profile: Option<ProfileKind>,
    /// Transaction size limit in bytes. Honored by the RocksDB storage
    /// engine only.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "maxTransactionSize")]
    max_txn_size: Option<usize>,
    /// Optimizer rules
    #[builder(setter(strip_option), default)]
    optimizer: Option<Rules>,
    /// Specify `true` and the query will be executed in a streaming fashion.
    /// The query result is not stored on the server, but calculated on the fly.
    /// **Beware**: long-running queries will need to hold the collection
    /// locks for as long as the query cursor exists.
    ///
    /// When set to `false` a query will be executed right away in its entirety.
    /// In that case query results are either returned right away (if the
    /// result set is small enough), or stored on the arangod instance and
    /// accessible via the cursor API (with respect to the ttl).
    ///
    /// It is advisable to only use this option on short-running queries or
    /// without exclusive locks (write-locks on MMFiles).
    ///
    /// Please note that the query options cache, count and fullCount will
    /// not work on streaming queries.
    ///
    /// Additionally query statistics, warnings and profiling data will
    /// only be available after the query is finished.
    ///
    /// The default value is `false`
    #[builder(setter(strip_option), default)]
    stream: Option<bool>,
    /// The query has to be executed within the given runtime or it
    /// will be killed. The value is specified in seconds. The default
    /// value is 0 (no timeout).
    #[builder(setter(strip_option), default)]
    #[serde(rename = "maxRuntime")]
    max_runtime: Option<usize>,
    /// Limits the maximum number of warnings a query will return.
    /// The number of warnings a query will return is limited to 10 by
    /// default, but that number can be increased or decreased by setting
    /// this attribute.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "maxWarningCount")]
    max_warning_count: Option<usize>,
    /// Maximum number of operations after which an intermediate
    /// commit is performed automatically. Honored by the RocksDB
    /// storage engine only.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "intermediateCommitCount")]
    intermediate_commit_count: Option<usize>,
    /// Maximum total size of operations after which an intermediate
    /// commit is performed automatically. Honored by the RocksDB
    /// storage engine only.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "intermediateCommitSize")]
    intermediate_commit_size: Option<usize>,
    /// Limits the maximum number of plans that are created by
    /// the AQL query optimizer.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "maxPlans")]
    max_plans: Option<usize>,
    /// If set to true and the query contains a LIMIT clause, then the
    /// result will have an extra attribute with the sub-attributes
    /// `stats` and `full_count`, { ... , "extra": { "stats": { "full_count": 123 } } }.
    /// The `fullCount` attribute will contain the number of documents
    /// in the result before the last top-level LIMIT in the query
    /// was applied. It can be used to count the number of documents
    /// that match certain filter criteria, but only return a subset of them,
    /// in one go.
    ///
    /// It is thus similar to MySQL's `SQL_CALC_FOUND_ROWS` hint.
    ///
    /// Note that setting the option will disable a few LIMIT optimizations
    /// and may lead to more documents being processed, and thus make
    /// queries run longer.
    ///
    /// Note that the `fullCount` attribute may only be present in
    /// the result if the query has a top-level LIMIT clause and the LIMIT
    /// clause is actually used in the query.
    #[builder(setter(strip_option), default)]
    #[serde(rename = "fullCount")]
    full_count: Option<bool>,
}

/// Cursor creation optimizer rules
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Rules {
    /// A list of to-be-included or to-be-excluded optimizer rules can be
    /// put into this attribute, telling the optimizer to include or exclude
    /// specific rules. To disable a rule, prefix its name with a `-`,
    /// to enable a rule, prefix it with a `+`. There is also a pseudo-rule
    /// `all`, which matches all optimizer rules. `-all` disables all rules.
    rules: Option<Vec<String>>,
}

#[cfg(test)]
mod test {
    use super::{ConfigBuilder, BATCH_SIZE_ZERO_ERR};

    #[test]
    fn batch_size_zero_errors() {
        match ConfigBuilder::default().batch_size(0).build() {
            Ok(_) => panic!("The builder should fail!"),
            Err(e) => assert_eq!(BATCH_SIZE_ZERO_ERR, format!("{}", e)),
        }
    }
}
