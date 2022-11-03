// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango`
//!
//! A database driver written in Rust for the [`ArangoDB`](https://www.arangodb.com) database.
//!
//! While the API for `ruarango` is async, `ArangoDB` supports 3 modes of operation.  They are blocking, store, and fire & forget.
//! The latter two modes are asynchronus, while the first is synchronous.  More details can be found
//! [here](https://www.arangodb.com/docs/stable/http/async-results-management.html).  See below for examples of using
//! the driver in these various modes.
//!
//! # Synchronous Blocking Connection
//! Use the driver in [`Blocking`](https://www.arangodb.com/docs/stable/http/async-results-management.html#blocking-execution) mode
//!
//! ```
//! # use anyhow::Result;
//! // Use `ConnectionBuilder` to build a connection and pull in the
//! // traits for operations you wish to use
//! use ruarango::{ConnectionBuilder, Database};
//! # use libeither::Either;
//! # use ruarango::{JobInfo, common::output::Response, db::output::Current, mock_auth, mock_database_create, start_mock_server};
//! # use serde_derive::{Deserialize, Serialize};
//! # use wiremock::{
//! #    matchers::{method, path, body_string_contains},
//! #    Mock, MockServer, ResponseTemplate,
//! # };
//! #
//! # async fn blah() -> Result<()> {
//! # let mock_server = start_mock_server().await;
//! # mock_auth(&mock_server).await;
//! # mock_database_create(&mock_server).await;
//! # let url = mock_server.uri();
//!
//! // Setup a synchronous connection to the database
//! let conn = ConnectionBuilder::default()
//!     .url(url) // The normal url for ArangoDB running locally is http://localhost:8529
//!     .username("root")
//!     .password("")
//!     .database("test_db")
//!     .build()
//!     .await?;
//!
//! // Use the connection to query information about the current database
//! let res = conn.current().await?;
//!
//! // Get the sync results out of the right side of the `Either`
//! assert!(res.is_right());
//! let contents = res.right_safe()?;
//! assert!(!contents.error());
//! assert_eq!(*contents.code(), 200);
//! assert_eq!(contents.result().name(), "test");
//! assert_eq!(contents.result().id(), "123");
//! assert!(!contents.result().is_system());
//! #     Ok(())
//! # }
//! # tokio_test::block_on(blah());
//! ```
//!
//! # Asynchronous Store Connection
//! Use the driver in [`Store`](https://www.arangodb.com/docs/stable/http/async-results-management.html#async-execution-and-later-result-retrieval) mode
//!
//! ```
//! # use anyhow::{anyhow, Result};
//! // Use `ConnectionBuilder` to build a connection and pull in the
//! // traits for operations you wish to use
//! use ruarango::{ConnectionBuilder, Database, Job};
//! # use libeither::Either;
//! # use ruarango::{
//! #     mock_auth, mock_async_database_create, mock_get_job, mock_put_job, start_mock_server, AsyncKind, JobInfo,
//! #     common::output::Response,
//! #     db::output::Current,
//! # };
//! # use serde_derive::{Deserialize, Serialize};
//! # use wiremock::{
//! #     matchers::{method, path, body_string_contains},
//! #     Mock, MockServer, ResponseTemplate,
//! # };
//! #
//! # async fn blah() -> Result<()> {
//! # let mock_server = start_mock_server().await;
//! # mock_auth(&mock_server).await;
//! # mock_async_database_create(&mock_server).await;
//! # mock_get_job(&mock_server).await;
//! # mock_put_job(&mock_server).await;
//! # let url = mock_server.uri();
//!
//! // Setup a asynchronous store connection to the database
//! let conn = ConnectionBuilder::default()
//!     .url(url) // The normal url for ArangoDB running locally is http://localhost:8529
//!     .username("root")
//!     .password("")
//!     .database("test_db")
//!     .async_kind(AsyncKind::Store)
//!     .build()
//!     .await?;
//!
//! // Use the connection to spawn a job for information about the current database
//! // This will return immediately with a 202 code and job information if the job
//! // was accepted into the queue.
//! let res = conn.current().await?;
//!
//! // Get the async job info results out of the left side of the `Either`
//! assert!(res.is_left());
//! let contents = res.left_safe()?;
//! assert_eq!(*contents.code(), 202);
//! assert!(contents.id().is_some());
//! let job_id = contents.id().as_ref().ok_or_else(|| anyhow!("invalid job_id"))?;
//! assert_eq!(job_id, "123456");
//!
//! // Check status until we get 200 (or error out on 404)
//! let mut status = conn.status(job_id).await?;
//! assert!(status == 200 || status == 204);
//!
//! while status != 200 {
//!     std::thread::sleep(std::time::Duration::from_millis(500));
//!     status = conn.status(job_id).await?;
//! }
//!
//! // Fetch the results (this has the side effect of removing the job off of the server)
//! let res: Response<Current> = conn.fetch(job_id).await?;
//! assert!(!res.error());
//! assert_eq!(*res.code(), 200);
//! assert_eq!(res.result().name(), "test");
//! assert_eq!(res.result().id(), "123");
//! assert!(!res.result().is_system());
//! #     Ok(())
//! # }
//! # tokio_test::block_on(blah());
//! ```
//!
//! # Asynchronous Fire & Forget Connection
//! Use the driver in [`Fire & Forget`](https://www.arangodb.com/docs/stable/http/async-results-management.html#fire-and-forget) mode
//!
//! ```
//! # use anyhow::{anyhow, Result};
//! // Use `ConnectionBuilder` to build a connection and pull in the
//! // traits for operations you wish to use
//! use ruarango::{ConnectionBuilder, Database, Job};
//! # use libeither::Either;
//! # use ruarango::{
//! #     mock_auth, mock_async_ff_database_create, start_mock_server, AsyncKind, JobInfo,
//! #     common::output::Response,
//! #     db::output::Current
//! # };
//! # use serde_derive::{Deserialize, Serialize};
//! # use wiremock::{
//! #   matchers::{method, path, body_string_contains},
//! #   Mock, MockServer, ResponseTemplate,
//! # };
//! #
//! # async fn blah() -> Result<()> {
//! # let mock_server = start_mock_server().await;
//! # mock_auth(&mock_server).await;
//! # mock_async_ff_database_create(&mock_server).await;
//! # let url = mock_server.uri();
//!
//! // Setup a asynchronous store connection to the database
//! let conn = ConnectionBuilder::default()
//!     .url(url) // The normal url for ArangoDB running locally is http://localhost:8529
//!     .username("root")
//!     .password("")
//!     .database("test_db")
//!     .async_kind(AsyncKind::FireAndForget)
//!     .build()
//!     .await?;
//!
//! // Use the connection to spawn a job for information about the current database
//! // In this case, fire and forget isn't useful, but for other operations it
//! // may be.  Fire and Forget jobs run on the server and do not store results.
//! let res = conn.current().await?;
//!
//! // Check that the job was accepted into the queue.
//! assert!(res.is_left());
//! let contents = res.left_safe()?;
//! assert_eq!(*contents.code(), 202);
//! assert!(contents.id().is_none());
//! #     Ok(())
//! # }
//! # tokio_test::block_on(blah());
//! ```
//!
// rustc lints
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    array_into_iter,
    asm_sub_register,
    bare_trait_objects,
    bindings_with_variant_name,
    // box_pointers,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    confusable_idents,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_in_future,
    drop_bounds,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    forbidden_lint_groups,
    function_item_references,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    indirect_structural_match,
    inline_no_sanitize,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    mixed_script_confusables,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    noop_method_call,
    overlapping_range_endpoints,
    path_statements,
    pointer_structural_match,
    private_in_public,
    proc_macro_back_compat,
    redundant_semicolons,
    renamed_and_removed_lints,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    stable_features,
    temporary_cstring_as_ptr,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_recursion,
    uninhabited_static,
    unknown_lints,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_results,
    unused_unsafe,
    unused_variables,
    variant_size_differences,
    where_clauses_object_safety,
    while_true
)]
// nightly only lints
#![cfg_attr(
    nightly_lints,
    deny(
        non_fmt_panics,
        rust_2021_incompatible_closure_captures,
        rust_2021_incompatible_or_patterns,
        rust_2021_prefixes_incompatible_syntax,
        rust_2021_prelude_collisions,
        unsupported_calling_conventions,
    )
)]
// nightly or beta only lints
#![cfg_attr(any(beta_lints, nightly_lints), deny(invalid_doc_attributes))]
// beta only lints
#![cfg_attr(beta_lints, deny(disjoint_capture_migration))]
// beta or stable only lints
#![cfg_attr(
    any(beta_lints, stable_lints),
    deny(non_fmt_panic, or_patterns_back_compat)
)]
// stable only lints
#![cfg_attr(stable_lints, deny(disjoint_capture_drop_reorder))]
// clippy lints
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::default_trait_access,
    clippy::semicolon_if_nothing_returned,
    clippy::uninlined_format_args
)]
#![cfg_attr(
    nightly_lints,
    allow(clippy::nonstandard_macro_braces, clippy::no_effect_underscore_binding)
)]
// rustdoc lints
#![deny(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::missing_crate_level_docs,
    // rustdoc::missing_doc_code_examples,
    // rustdoc::private_doc_tests,
    rustdoc::private_intra_doc_links,
)]

#[cfg(test)]
use {lazy_static as _, r2d2 as _, rand as _, tokio_test as _};

#[macro_use]
mod impls;
#[macro_use]
mod utils;

mod builder;
mod conn;
mod error;
#[doc(hidden)]
mod mocks;
mod model;
mod traits;
mod types;

pub use builder::AsyncKind;
pub use builder::Connection as BaseConnection;
pub use builder::ConnectionBuilder;
pub use conn::Connection;
pub use error::RuarangoErr as Error;
#[doc(hidden)]
pub use mocks::mock_async_database_create;
#[doc(hidden)]
pub use mocks::mock_async_ff_database_create;
#[doc(hidden)]
pub use mocks::mock_auth;
#[doc(hidden)]
pub use mocks::mock_database_create;
#[doc(hidden)]
pub use mocks::mock_get_job;
#[doc(hidden)]
pub use mocks::mock_put_job;
#[doc(hidden)]
pub use mocks::start_mock_server;
pub use model::coll;
pub use model::common;
pub use model::cursor;
pub use model::db;
pub use model::doc;
pub use model::graph;
pub use traits::Collection;
pub use traits::Cursor;
pub use traits::Database;
pub use traits::Document;
pub use traits::Graph;
pub use traits::Job;
pub use traits::JobInfo;
pub use types::ArangoEither;
pub use types::ArangoResult;
pub use types::ArangoVec;
pub use types::ArangoVecResult;
pub use types::DocMetaResult;
pub use types::DocMetaVecResult;
