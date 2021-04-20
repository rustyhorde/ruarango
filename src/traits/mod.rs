// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Database operation traits

use getset::Getters;
use serde_derive::{Deserialize, Serialize};

mod coll;
mod db;
mod doc;
mod job;

pub use coll::Collection;
pub use db::Database;
pub use doc::Document;
pub use job::Job;

/// Job Information from an asynchronous invocation
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct JobInfo {
    /// The response code
    code: u16,
    /// The id if valid
    id: Option<String>,
}

impl JobInfo {
    #[doc(hidden)]
    #[must_use]
    pub fn new(code: u16, id: Option<String>) -> Self {
        Self { code, id }
    }
}
