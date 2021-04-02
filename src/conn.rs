// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! An `ArangoDB` connection implementing the database operation traits

use getset::Getters;
use reqwest::{Client, Url};

/// An `ArangoDB` connection implementing the database operation traits
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub(crate)")]
pub struct Connection {
    #[doc(hidden)]
    base_url: Url,
    #[doc(hidden)]
    db_url: Url,
    #[doc(hidden)]
    client: Client,
}

impl Connection {
    pub(crate) fn new(base_url: Url, db_url: Url, client: Client) -> Self {
        Self {
            base_url,
            db_url,
            client,
        }
    }
}
