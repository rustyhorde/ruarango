// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango`
//!
//! ```
//! # use anyhow::Result;
//! # use ruarango::{ConnectionBuilder, Database, Response, db::Current};
//! # use serde_derive::{Deserialize, Serialize};
//! # use wiremock::{
//! #    matchers::{method, path, body_string_contains},
//! #    Mock, MockServer, ResponseTemplate,
//! # };
//! # #[derive(Deserialize, Serialize)]
//! # pub(crate) struct AuthResponse {
//! #     jwt: String,
//! # }
//! #
//! # impl From<&str> for AuthResponse {
//! #     fn from(val: &str) -> AuthResponse {
//! #         Self {
//! #             jwt: val.to_string(),
//! #         }
//! #     }
//! # }
//! #
//! # #[tokio::main]
//! # async fn blah() -> Result<()> {
//! # let mock_server = MockServer::start().await;
//! # let body: AuthResponse = "not a real jwt".into();
//! # let mock_response = ResponseTemplate::new(200).set_body_json(body);
//! #
//! # Mock::given(method("POST"))
//! #     .and(path("/_open/auth"))
//! #     .and(body_string_contains("username"))
//! #     .and(body_string_contains("password"))
//! #     .respond_with(mock_response)
//! #     .mount(&mock_server)
//! #     .await;
//! #
//! # let body = Response::<Current>::default();
//! # let mock_response = ResponseTemplate::new(200).set_body_json(body);
//! #
//! # Mock::given(method("GET"))
//! #     .and(path("/_db/keti/_api/database/current"))
//! #     .respond_with(mock_response)
//! #     .mount(&mock_server)
//! #     .await;
//! #
//! # let url = mock_server.uri();
//! // Setup a connection to the database
//! // The normal url for ArangoDB running locally is http://localhost:8529
//! let conn = ConnectionBuilder::new()
//!     .url(url)
//!     .username("root")
//!     .password("")
//!     .database("keti")
//!     .build()
//!     .await?;
//!
//! // Use the connection to query information about the
//! // current database
//! let res = conn.current().await?;
//!
//! // Assert on the results
//! assert!(!res.error());
//! assert_eq!(*res.code(), 200);
//! assert_eq!(res.result().name(), "test");
//! assert_eq!(res.result().id(), "123");
//! assert!(!res.result().is_system());
//! #     Ok(())
//! # }
//! ```
// Common lints
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
    // disjoint_capture_drop_reorder,
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
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_fmt_panic,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    overlapping_range_endpoints,
    path_statements,
    pointer_structural_match,
    private_in_public,
    proc_macro_derive_resolution_fallback,
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
    unstable_features,
    unstable_name_collisions,
    unsupported_naked_functions,
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
#![cfg_attr(nightly_lints, deny(unaligned_references))]
// nightly or beta only lints
#![cfg_attr(
    any(beta_lints, nightly_lints),
    deny(
        legacy_derive_helpers,
        noop_method_call,
        proc_macro_back_compat,
        unsafe_op_in_unsafe_fn,
    )
)]
// beta or stable only lints
#![cfg_attr(any(beta_lints, stable_lints), deny(safe_packed_borrows))]
// stable only lints
#![cfg_attr(
    stable_lints,
    deny(
        broken_intra_doc_links,
        invalid_codeblock_attributes,
        invalid_html_tags,
        missing_crate_level_docs,
        missing_doc_code_examples,
        non_autolinks,
        private_doc_tests,
        private_intra_doc_links,
    )
)]
// clippy lints
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::clippy::default_trait_access)]
// rustdoc lints
#![cfg_attr(
    any(nightly_lints, beta_lints),
    deny(
        rustdoc::broken_intra_doc_links,
        rustdoc::invalid_codeblock_attributes,
        rustdoc::invalid_html_tags,
        rustdoc::missing_crate_level_docs,
        rustdoc::missing_doc_code_examples,
        rustdoc::non_autolinks,
        rustdoc::private_doc_tests,
        rustdoc::private_intra_doc_links,
    )
)]

#[macro_use]
mod impls;
#[macro_use]
mod utils;

mod builder;
mod conn;
mod error;
mod model;
mod traits;

pub use builder::ConnectionBuilder;
pub use conn::Connection;
pub use model::coll;
pub use model::db;
pub use model::Response;
pub use traits::Collection;
pub use traits::Database;
