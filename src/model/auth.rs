// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` auth model

use getset::Getters;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct AuthBody {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub(crate)")]
pub(crate) struct AuthResponse {
    jwt: String,
}

#[cfg(test)]
impl From<&str> for AuthResponse {
    fn from(val: &str) -> AuthResponse {
        Self {
            jwt: val.to_string(),
        }
    }
}
