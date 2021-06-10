// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! An `ArangoDB` connection implementing the database operation traits

use anyhow::Result;
use futures::{Future, FutureExt};
use getset::Getters;
use libeither::Either;
use reqwest::{header::HeaderMap, Client, Error, Response, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::{utils::handle_job_response, ArangoResult};

pub(crate) enum HttpVerb {
    Delete,
    Get,
    Patch,
    Post,
    Put,
}

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
    #[doc(hidden)]
    async_client: Client,
    #[doc(hidden)]
    is_async: bool,
}

impl Connection {
    pub(crate) fn new(
        base_url: Url,
        db_url: Url,
        client: Client,
        async_client: Client,
        is_async: bool,
    ) -> Self {
        Self {
            base_url,
            db_url,
            client,
            async_client,
            is_async,
        }
    }

    pub(crate) async fn req<F, T, U, V>(
        &self,
        verb: &HttpVerb,
        url: Url,
        headers: Option<HeaderMap>,
        json: Option<U>,
        f: F,
    ) -> ArangoResult<T>
    where
        T: DeserializeOwned + Send + Sync,
        U: Serialize + Send + Sync,
        F: FnOnce(std::result::Result<Response, Error>) -> V,
        V: Future<Output = Result<T>> + Send + Sync,
    {
        if *self.is_async() {
            let client = self.async_client();
            Ok(Either::new_left(
                req(client, verb, url, headers, json)
                    .then(handle_job_response)
                    .await?,
            ))
        } else {
            let client = self.client();
            Ok(Either::new_right(
                req(client, verb, url, headers, json).then(f).await?,
            ))
        }
    }

    pub(crate) async fn delete<F, T, U, V>(
        &self,
        url: Url,
        headers: Option<HeaderMap>,
        json: U,
        f: F,
    ) -> ArangoResult<T>
    where
        T: DeserializeOwned + Send + Sync,
        U: Serialize + Send + Sync,
        F: FnOnce(std::result::Result<Response, Error>) -> V,
        V: Future<Output = Result<T>> + Send + Sync,
    {
        self.req(&HttpVerb::Delete, url, headers, Some(json), f)
            .await
    }

    pub(crate) async fn get<F, T, U, V>(
        &self,
        url: Url,
        headers: Option<HeaderMap>,
        json: Option<U>,
        f: F,
    ) -> ArangoResult<T>
    where
        T: DeserializeOwned + Send + Sync,
        U: Serialize + Send + Sync,
        F: FnOnce(std::result::Result<Response, Error>) -> V,
        V: Future<Output = Result<T>> + Send + Sync,
    {
        self.req(&HttpVerb::Get, url, headers, json, f).await
    }

    pub(crate) async fn patch<F, T, U, V>(
        &self,
        url: Url,
        headers: Option<HeaderMap>,
        json: U,
        f: F,
    ) -> ArangoResult<T>
    where
        T: DeserializeOwned + Send + Sync,
        U: Serialize + Send + Sync,
        F: FnOnce(std::result::Result<Response, Error>) -> V,
        V: Future<Output = Result<T>> + Send + Sync,
    {
        self.req(&HttpVerb::Patch, url, headers, Some(json), f)
            .await
    }

    pub(crate) async fn post<F, T, U, V>(
        &self,
        url: Url,
        headers: Option<HeaderMap>,
        json: U,
        f: F,
    ) -> ArangoResult<T>
    where
        T: DeserializeOwned + Send + Sync,
        U: Serialize + Send + Sync,
        F: FnOnce(std::result::Result<Response, Error>) -> V,
        V: Future<Output = Result<T>> + Send + Sync,
    {
        self.req(&HttpVerb::Post, url, headers, Some(json), f).await
    }

    pub(crate) async fn put<F, T, U, V>(
        &self,
        url: Url,
        headers: Option<HeaderMap>,
        json: U,
        f: F,
    ) -> ArangoResult<T>
    where
        T: DeserializeOwned + Send + Sync,
        U: Serialize + Send + Sync,
        F: FnOnce(std::result::Result<Response, Error>) -> V,
        V: Future<Output = Result<T>> + Send + Sync,
    {
        self.req(&HttpVerb::Put, url, headers, Some(json), f).await
    }
}

fn req<T>(
    client: &Client,
    verb: &HttpVerb,
    url: Url,
    headers: Option<HeaderMap>,
    json: Option<T>,
) -> impl Future<Output = std::result::Result<Response, Error>>
where
    T: Serialize + Send + Sync,
{
    let mut rb = match verb {
        HttpVerb::Delete => client.delete(url),
        HttpVerb::Get => client.get(url),
        HttpVerb::Patch => client.patch(url),
        HttpVerb::Post => client.post(url),
        HttpVerb::Put => client.put(url),
    };

    if let Some(headers) = headers {
        rb = rb.headers(headers);
    }

    if let Some(json) = json {
        rb = rb.json(&json);
    }

    rb.send()
}
