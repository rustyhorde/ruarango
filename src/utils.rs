// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` utils

use reqwest::Error;
use serde::de::DeserializeOwned;

async fn to_json<T>(res: reqwest::Response) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    res.error_for_status()
        .map(|res| async move { res.json::<T>().await })?
        .await
}

pub(crate) async fn handle_response<T>(res: Result<reqwest::Response, Error>) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    res.map(to_json)?.await
}
