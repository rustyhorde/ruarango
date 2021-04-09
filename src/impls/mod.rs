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
        Ok($self.client().get($url).headers($headers).send().then(handle_response).await?)
    };
    ($self:ident, $url:ident, GET) => {
        Ok($self.client().get($url).send().then(handle_response).await?)
    };
    ($self:ident, $url:ident, DELETE) => {
        Ok($self.client().delete($url).send().then(handle_response).await?)
    };
    ($self:ident, $url:ident, PUT) => {
        Ok($self.client().put($url).send().then(handle_response).await?)
    };
    ($self:ident, $url:ident, PUT, $json:expr) => {
        Ok($self.client().put($url).json($json).send().then(handle_response).await?)
    };
    ($self:ident, $url:ident, DELETE) => {
        Ok($self.client().delete($url).send().then(handle_response).await?)
    };
    ($self:ident, $url:ident, POST, $json:expr) => {
        Ok($self.client().post($url).json($json).send().then(handle_response).await?)
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

#[doc(hidden)]
#[macro_export]
macro_rules! api_request_async {
    () => {};
    ($self:ident, $url:ident, GET, $headers:expr) => {
        $self.async_client().get($url).headers($headers).send().await?
    };
    ($self:ident, $url:ident, GET) => {
        $self.async_client().get($url).send().await?
    };
    ($self:ident, $url:ident, DELETE) => {
        $self.async_client().delete($url).send().await?
    };
    ($self:ident, $url:ident, PUT) => {
        $self.async_client().put($url).send().await?
    };
    ($self:ident, $url:ident, PUT, $json:expr) => {
        $self.async_client().put($url).json($json).send().await?
    };
    ($self:ident, $url:ident, DELETE) => {
        $self.async_client().delete($url).send().await?
    };
    ($self:ident, $url:ident, POST, $json:expr) => {
        $self.async_client().post($url).json($json).send().await?
    };
    ($self:ident, $url:ident, $suffix:expr, $($tail:tt)*) => {
        {
            let current_url = $self
                .$url()
                .join($suffix)
                .with_context(|| format!("Unable to build '{}' url", $suffix))?;

            let res = $crate::api_request_async!($self, current_url, $($tail)*);

            let status = res.status().as_u16();
            let job_id = res
                .headers()
                .get("x-arango-async-id")
                .map(|x| String::from_utf8_lossy(x.as_bytes()).to_string());

            Ok(libeither::Either::new_left(JobInfo::new(status, job_id)))
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_get_async {
    ($self:ident, $url:ident, $suffix:expr) => {
        $crate::api_request_async!($self, $url, $suffix, GET)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_request_right {
    () => {};
    ($self:ident, $url:ident, GET, $headers:expr) => {
        $self.client().get($url).headers($headers).send().then(handle_response).await
    };
    ($self:ident, $url:ident, GET) => {
        $self.client().get($url).send().then(handle_response).await
    };
    ($self:ident, $url:ident, DELETE) => {
        $self.client().delete($url).send().then(handle_response).await
    };
    ($self:ident, $url:ident, PUT) => {
        $self.client().put($url).send().then(handle_response).await
    };
    ($self:ident, $url:ident, PUT, $json:expr) => {
        $self.client().put($url).json($json).send().then(handle_response).await
    };
    ($self:ident, $url:ident, DELETE) => {
        $self.client().delete($url).send().then(handle_response).await
    };
    ($self:ident, $url:ident, POST, $json:expr) => {
        $self.client().post($url).json($json).send().then(handle_response).await
    };
    ($self:ident, $url:ident, $suffix:expr, $kind:ty, $($tail:tt)*) => {
        {
            let current_url = $self
                .$url()
                .join($suffix)
                .with_context(|| format!("Unable to build '{}' url", $suffix))?;

            let res: Result<$kind> = $crate::api_request_right!($self, current_url, $($tail)*);
            Ok(libeither::Either::new_right(res?))
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! api_get_right {
    ($self:ident, $url:ident, $suffix:expr, $kind:ty) => {
        $crate::api_request_right!($self, $url, $suffix, $kind, GET)
    };
}
