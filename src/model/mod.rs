// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` model

use getset::Getters;
use serde_derive::Deserialize;
#[cfg(test)]
use {
    self::{coll::Collection, db::Current},
    getset::Setters,
    serde_derive::Serialize,
};

mod auth;
pub(crate) use auth::{AuthBody, AuthResponse};
pub mod coll;
pub mod db;

/// A base response
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize, Setters))]
#[getset(get = "pub")]
#[cfg_attr(test, getset(set = "pub(crate)"))]
pub struct Response<T> {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// The response content
    result: T,
}

#[cfg(test)]
impl Default for Response<Current> {
    fn default() -> Self {
        Response {
            error: false,
            code: 200,
            result: Current::default(),
        }
    }
}

#[cfg(test)]
impl Default for Response<Vec<String>> {
    fn default() -> Self {
        Response {
            error: false,
            code: 200,
            result: vec!["_system".to_string(), "test".to_string()],
        }
    }
}

#[cfg(test)]
impl Default for Response<bool> {
    fn default() -> Self {
        Response {
            error: false,
            code: 200,
            result: true,
        }
    }
}

#[cfg(test)]
impl Default for Response<Vec<Collection>> {
    fn default() -> Self {
        Response {
            error: false,
            code: 200,
            result: vec![Collection::default()],
        }
    }
}
