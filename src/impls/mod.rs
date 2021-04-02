// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` trait impls for `[Connection](crate::Connection)`

mod coll;
mod db;

#[doc(hidden)]
#[macro_export]
macro_rules! api_get {
    ($self:ident, $url:ident, $suffix:expr) => {{
        let current_url = $self
            .$url()
            .join($suffix)
            .with_context(|| format!("Unable to build '{}' url", $suffix))?;
        Ok($self
            .client()
            .get(current_url)
            .send()
            .then(handle_response)
            .await?)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_post {
    ($self:ident, $url:ident, $suffix:expr, $json:expr) => {{
        let current_url = $self
            .$url()
            .join($suffix)
            .with_context(|| format!("Unable to build '{}' url", $suffix))?;
        Ok($self
            .client()
            .post(current_url)
            .json($json)
            .send()
            .then(handle_response)
            .await?)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_delete {
    ($self:ident, $url:ident, $suffix:expr) => {{
        let current_url = $self
            .$url()
            .join($suffix)
            .with_context(|| format!("Unable to build '{}' url", $suffix))?;
        Ok($self
            .client()
            .delete(current_url)
            .send()
            .then(handle_response)
            .await?)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_put {
    ($self:ident, $url:ident, $suffix:expr, $json:expr) => {{
        let current_url = $self
            .$url()
            .join($suffix)
            .with_context(|| format!("Unable to build '{}' url", $suffix))?;
        Ok($self
            .client()
            .put(current_url)
            .json($json)
            .send()
            .then(handle_response)
            .await?)
    }};
    ($self:ident, $url:ident, $suffix:expr) => {{
        let current_url = $self
            .$url()
            .join($suffix)
            .with_context(|| format!("Unable to build '{}' url", $suffix))?;
        Ok($self
            .client()
            .put(current_url)
            .send()
            .then(handle_response)
            .await?)
    }};
}
