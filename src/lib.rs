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
//! # use serde::{Deserialize, Serialize};
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
//! # use serde::{Deserialize, Serialize};
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
//! # use serde::{Deserialize, Serialize};
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
#![cfg_attr(
    all(feature = "unstable", nightly),
    feature(
        lint_reasons,
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns_lint,
        rustdoc_missing_doc_code_examples,
        strict_provenance,
        type_privacy_lints,
    )
)]
#![cfg_attr(nightly, allow(box_pointers, single_use_lifetimes))]
#![cfg_attr(
    nightly,
    deny(
        absolute_paths_not_starting_with_crate,
        ambiguous_glob_imports,
        ambiguous_glob_reexports,
        ambiguous_wide_pointer_comparisons,
        anonymous_parameters,
        array_into_iter,
        asm_sub_register,
        async_fn_in_trait,
        bad_asm_style,
        bare_trait_objects,
        break_with_label_and_loop,
        byte_slice_in_packed_struct_with_derive,
        clashing_extern_declarations,
        coherence_leak_check,
        confusable_idents,
        const_evaluatable_unchecked,
        const_item_mutation,
        dead_code,
        deprecated,
        deprecated_in_future,
        deprecated_where_clause_location,
        deref_into_dyn_supertrait,
        deref_nullptr,
        drop_bounds,
        dropping_copy_types,
        dropping_references,
        duplicate_macro_attributes,
        dyn_drop,
        elided_lifetimes_in_associated_constant,
        elided_lifetimes_in_paths,
        ellipsis_inclusive_range_patterns,
        explicit_outlives_requirements,
        exported_private_dependencies,
        ffi_unwind_calls,
        forbidden_lint_groups,
        forgetting_copy_types,
        forgetting_references,
        for_loops_over_fallibles,
        function_item_references,
        hidden_glob_reexports,
        improper_ctypes,
        improper_ctypes_definitions,
        incomplete_features,
        indirect_structural_match,
        inline_no_sanitize,
        internal_features,
        invalid_doc_attributes,
        invalid_from_utf8,
        invalid_macro_export_arguments,
        invalid_nan_comparisons,
        invalid_value,
        irrefutable_let_patterns,
        keyword_idents,
        large_assignments,
        late_bound_lifetime_arguments,
        legacy_derive_helpers,
        let_underscore_drop,
        macro_use_extern_crate,
        map_unit_fn,
        meta_variable_misuse,
        missing_abi,
        missing_copy_implementations,
        missing_debug_implementations,
        missing_docs,
        mixed_script_confusables,
        named_arguments_used_positionally,
        no_mangle_generic_items,
        non_ascii_idents,
        non_camel_case_types,
        non_fmt_panics,
        non_shorthand_field_patterns,
        non_snake_case,
        non_upper_case_globals,
        noop_method_call,
        opaque_hidden_inferred_bound,
        overlapping_range_endpoints,
        path_statements,
        pointer_structural_match,
        private_bounds,
        private_interfaces,
        redundant_semicolons,
        refining_impl_trait,
        renamed_and_removed_lints,
        repr_transparent_external_private_fields,
        rust_2021_incompatible_closure_captures,
        rust_2021_incompatible_or_patterns,
        rust_2021_prefixes_incompatible_syntax,
        rust_2021_prelude_collisions,
        semicolon_in_expressions_from_macros,
        special_module_name,
        stable_features,
        static_mut_refs,
        suspicious_double_ref_op,
        temporary_cstring_as_ptr,
        trivial_bounds,
        trivial_casts,
        trivial_numeric_casts,
        type_alias_bounds,
        tyvar_behind_raw_pointer,
        uncommon_codepoints,
        unconditional_recursion,
        undefined_naked_function_abi,
        unexpected_cfgs,
        ungated_async_fn_track_caller,
        uninhabited_static,
        unit_bindings,
        unknown_lints,
        unnameable_test_items,
        unreachable_code,
        unreachable_patterns,
        unreachable_pub,
        unsafe_code,
        unsafe_op_in_unsafe_fn,
        unstable_name_collisions,
        unstable_syntax_pre_expansion,
        unsupported_calling_conventions,
        unused_allocation,
        unused_assignments,
        unused_associated_type_bounds,
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
        unused_macro_rules,
        unused_macros,
        unused_must_use,
        unused_mut,
        unused_parens,
        unused_qualifications,
        unused_results,
        unused_unsafe,
        unused_variables,
        useless_ptr_null_checks,
        variant_size_differences,
        where_clauses_object_safety,
        while_true,
        writes_through_immutable_pointer,
    )
)]
// If nightly and unstable, allow `unstable_features`
#![cfg_attr(all(feature = "unstable", nightly), allow(unstable_features))]
// If nightly and not unstable, deny `unstable_features`
#![cfg_attr(all(not(feature = "unstable"), nightly), deny(unstable_features))]
// The unstable lints
#![cfg_attr(
    all(feature = "unstable", nightly),
    deny(
        fuzzy_provenance_casts,
        lossy_provenance_casts,
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns,
        unfulfilled_lint_expectations,
        // unknown_or_malformed_diagnostic_attributes,
        unnameable_types,
    )
)]
// clippy lints
#![cfg_attr(nightly, deny(clippy::all, clippy::pedantic))]
// rustdoc lints
#![cfg_attr(
    nightly,
    deny(
        rustdoc::bare_urls,
        rustdoc::broken_intra_doc_links,
        rustdoc::invalid_codeblock_attributes,
        rustdoc::invalid_html_tags,
        rustdoc::missing_crate_level_docs,
        rustdoc::private_doc_tests,
        rustdoc::private_intra_doc_links,
    )
)]
#![cfg_attr(
    all(nightly, feature = "unstable"),
    deny(rustdoc::missing_doc_code_examples)
)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]
#![cfg_attr(all(docsrs, nightly), feature(doc_cfg))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

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
pub use model::BaseErr;
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
