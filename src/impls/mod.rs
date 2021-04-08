// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Trait impls for `[Connection](crate::Connection)`

mod coll;
mod db;
mod doc;
mod job;

#[doc(hidden)]
#[macro_export]
macro_rules! api_request {
    () => {};
    ($self:ident, $url:ident, GET, $headers:expr) => {
        Ok($self
            .client()
            .get($url)
            .headers($headers)
            .send()
            .then(handle_response)
            .await?)
    };
    ($self:ident, $url:ident, GET) => {
        Ok($self
            .client()
            .get($url)
            .send()
            .then(handle_response)
            .await?)
    };
    ($self:ident, $url:ident, DELETE) => {
        Ok($self
            .client()
            .delete($url)
            .send()
            .then(handle_response)
            .await?)
    };
    ($self:ident, $url:ident, PUT) => {
        Ok($self
            .client()
            .put($url)
            .send()
            .then(handle_response)
            .await?)
    };
    ($self:ident, $url:ident, PUT, $json:expr) => {
        Ok($self
            .client()
            .put($url)
            .json($json)
            .send()
            .then(handle_response)
            .await?)
    };
    ($self:ident, $url:ident, DELETE) => {
        Ok($self
            .client()
            .delete($url)
            .send()
            .then(handle_response)
            .await?)
    };
    ($self:ident, $url:ident, POST, $json:expr) => {
        Ok($self
            .client()
            .post($url)
            .json($json)
            .send()
            .then(handle_response)
            .await?)
    };
    ($self:ident, $url:ident, $suffix:expr, $($tail:tt)*) => {
        {
            let current_url = $self
                .$url()
                .join($suffix)
                .with_context(|| format!("Unable to build '{}' url", $suffix))?;

            $crate::api_request!($self, current_url, $($tail)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_get {
    ($self:ident, $url:ident, $suffix:expr) => {
        $crate::api_request!($self, $url, $suffix, GET)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_post {
    ($self:ident, $url:ident, $suffix:expr, $json:expr) => {
        $crate::api_request!($self, $url, $suffix, POST, $json)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_delete {
    ($self:ident, $url:ident, $suffix:expr) => {
        $crate::api_request!($self, $url, $suffix, DELETE)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_put {
    ($self:ident, $url:ident, $suffix:expr, $json:expr) => {
        $crate::api_request!($self, $url, $suffix, PUT, $json)
    };
    ($self:ident, $url:ident, $suffix:expr) => {
        $crate::api_request!($self, $url, $suffix, PUT)
    };
}
