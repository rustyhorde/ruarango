// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Types

use crate::{
    model::{common::output::ArangoErr, doc::output::DocMeta},
    traits::JobInfo,
};
use anyhow::Result;
use libeither::Either;

/// Either [`JobInfo`](crate::traits::JobInfo) from an asynchronous invocation on the left
/// or the result `T` from a synchronous invocation on the right
pub type ArangoEither<T> = Either<JobInfo, T>;

/// A result that on success is either [`JobInfo`](crate::traits::JobInfo)
/// from an asynchronous invocation on the left or the result `T` from
/// a synchronous invocation on the right
pub type ArangoResult<T> = Result<ArangoEither<T>>;

/// An [`ArangoResult`] that has [`DocMeta`](crate::model::doc::output::DocMeta)
/// on the right.
///
/// * The type `N` is the type of the [`new_doc`](crate::model::doc::output::DocMeta::new_doc) output if enabled.
/// * The type `O` is the type of the [`old_doc`](crate::model::doc::output::DocMeta::old_doc) output if enabled.
pub type DocMetaResult<N, O> = ArangoResult<DocMeta<N, O>>;

///
pub type ArangoVec<T> = Vec<Either<ArangoErr, T>>;

///
pub type ArangoVecResult<T> = ArangoResult<ArangoVec<T>>;

///
pub type DocMetaVecResult<N, O> = ArangoResult<ArangoVec<DocMeta<N, O>>>;
