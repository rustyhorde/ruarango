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
use serde_derive::Serialize;

pub mod db;

#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct Response<T> {
    error: bool,
    code: usize,
    result: T,
}
