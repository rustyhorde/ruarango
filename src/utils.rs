// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` utils

use crate::{
    error::RuarangoErr::{
        Conflict, Cursor, DocumentNotFound, InvalidBody, InvalidCursorResponse, InvalidDocResponse,
        NotModified, PreconditionFailed,
    },
    model::{common::output::ArangoErr, doc::output::DocErr, BaseErr},
    JobInfo,
};
use anyhow::{anyhow, Result};
use libeither::Either;
use reqwest::{Error, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[cfg(test)]
use {
    crate::{
        builder::{AsyncKind, ConnectionBuilder},
        conn::Connection,
        model::auth::output::AuthResponse,
    },
    wiremock::{
        matchers::{body_string_contains, method, path},
        Mock, MockServer, ResponseTemplate,
    },
};

pub(crate) fn prepend_sep(url: &mut String, has_qp: bool) -> &mut String {
    if has_qp {
        *url += "&";
    } else {
        *url += "?";
    }

    url
}

#[doc(hidden)]
#[macro_export]
macro_rules! add_qps {
    ($field:expr, $url:ident, $has_qp:ident, $val:expr => last) => {
        if $field.unwrap_or(false) {
            let _ = crate::utils::prepend_sep(&mut $url, $has_qp);
            $url += $val;
        }
    };
    ($field:expr, $url:ident, $has_qp:ident, $val:expr) => {
        if $field.unwrap_or(false) {
            let _ = crate::utils::prepend_sep(&mut $url, $has_qp);
            $url += $val;
            $has_qp = true;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! add_qp {
    ($url:ident, $has_qp:ident, $val:expr;) => {
        let _ = crate::utils::prepend_sep(&mut $url, $has_qp);
        $url += $val;
    };
    ($url:ident, $has_qp:ident, $val:expr) => {
        let _ = crate::utils::prepend_sep(&mut $url, $has_qp);
        $url += $val;
        $has_qp = true;
    };
}

fn invalid_body(e: &serde_json::Error, text: &str) -> anyhow::Error {
    InvalidBody {
        err: format!("{}", e),
        body: text.to_string(),
    }
    .into()
}

async fn handle_text<T>(res: reqwest::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    match res.text().await {
        Ok(text) => {
            let invalid_body = |e: serde_json::Error| -> anyhow::Error { invalid_body(&e, &text) };
            serde_json::from_str::<T>(&text).map_err(invalid_body)
        }
        Err(e) => Err(e.into()),
    }
}

async fn handle_text_vec<T>(res: reqwest::Response) -> Result<Vec<Either<ArangoErr, T>>>
where
    T: DeserializeOwned,
{
    match res.text().await {
        Ok(text) => {
            let invalid_body = |e: serde_json::Error| -> anyhow::Error { invalid_body(&e, &text) };
            let body: Value = serde_json::from_str(&text).map_err(invalid_body)?;
            let mut result: Vec<Either<ArangoErr, T>> = vec![];
            match body {
                Value::Array(v) => {
                    for val in v {
                        let doc_val = val.clone();
                        let err_val = val.clone();
                        match serde_json::from_value::<T>(doc_val) {
                            Ok(doc) => result.push(Either::new_right(doc)),
                            Err(_e) => match serde_json::from_value::<ArangoErr>(err_val) {
                                Ok(doc_err) => result.push(Either::new_left(doc_err)),
                                Err(_e) => {}
                            },
                        }
                    }
                }
                _ => return Err(anyhow!("result was not an array!")),
            }
            Ok(result)
        }
        Err(e) => Err(e.into()),
    }
}

async fn to_json<T>(res: reqwest::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    res.error_for_status()
        .map(|res| async move { handle_text(res).await })?
        .await
}

pub(crate) async fn handle_response<T>(res: Result<reqwest::Response, Error>) -> Result<T>
where
    T: DeserializeOwned,
{
    res.map(to_json)?.await
}

fn to_empty(res: reqwest::Response) -> Result<()> {
    res.error_for_status().map(|_| ()).map_err(Error::into)
}

pub(crate) async fn empty(res: Result<reqwest::Response, Error>) -> Result<()> {
    res.map(to_empty)?
}

pub(crate) async fn handle_job_response(res: Result<reqwest::Response, Error>) -> Result<JobInfo> {
    res.map(|res| {
        let status = res.status().as_u16();
        let job_id = res
            .headers()
            .get("x-arango-async-id")
            .map(|x| x.to_str().unwrap_or_default().to_string());
        JobInfo::new(status, job_id)
    })
    .map_err(Error::into)
}

async fn to_docmeta_json<T>(res: reqwest::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    match res.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => Ok(handle_text(res).await?),
        StatusCode::NOT_FOUND => Err(DocumentNotFound.into()),
        StatusCode::NOT_MODIFIED => Err(NotModified.into()),
        StatusCode::CONFLICT => {
            let err: Option<DocErr> = handle_text(res).await.ok();
            Err(Conflict { err }.into())
        }
        StatusCode::PRECONDITION_FAILED => {
            let err: Option<DocErr> = handle_text(res).await.ok();
            Err(PreconditionFailed { err }.into())
        }
        _ => {
            let status = res.status().as_u16();
            Err(InvalidDocResponse { status }.into())
        }
    }
}

async fn to_docmeta_vec_json<T>(res: reqwest::Response) -> Result<Vec<Either<ArangoErr, T>>>
where
    T: DeserializeOwned,
{
    match res.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
            Ok(handle_text_vec(res).await?)
        }
        StatusCode::NOT_FOUND => Err(DocumentNotFound.into()),
        StatusCode::NOT_MODIFIED => Err(NotModified.into()),
        StatusCode::CONFLICT => {
            let err: Option<DocErr> = handle_text(res).await.ok();
            Err(Conflict { err }.into())
        }
        StatusCode::PRECONDITION_FAILED => {
            let err: Option<DocErr> = handle_text(res).await.ok();
            Err(PreconditionFailed { err }.into())
        }
        _ => {
            let status = res.status().as_u16();
            Err(InvalidDocResponse { status }.into())
        }
    }
}

pub(crate) async fn doc_resp<T>(res: std::result::Result<reqwest::Response, Error>) -> Result<T>
where
    T: DeserializeOwned,
{
    res.map(to_docmeta_json)?.await
}

pub(crate) async fn doc_vec_resp<T>(
    res: std::result::Result<reqwest::Response, Error>,
) -> Result<Vec<Either<ArangoErr, T>>>
where
    T: DeserializeOwned,
{
    res.map(to_docmeta_vec_json)?.await
}

async fn to_cursor_json<T>(res: reqwest::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    match res.status() {
        StatusCode::CREATED | StatusCode::ACCEPTED => Ok(handle_text(res).await?),
        StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND => {
            let err: Option<BaseErr> = handle_text(res).await.ok();
            Err(Cursor { err }.into())
        }
        _ => {
            let status = res.status().as_u16();
            Err(InvalidCursorResponse { status }.into())
        }
    }
}

pub(crate) async fn cursor_resp<T>(res: std::result::Result<reqwest::Response, Error>) -> Result<T>
where
    T: DeserializeOwned,
{
    res.map(to_cursor_json)?.await
}

#[cfg(test)]
pub(crate) async fn mock_auth(mock_server: &MockServer) {
    let body: AuthResponse = "not a real jwt".into();
    let mock_response = ResponseTemplate::new(200).set_body_json(body);

    Mock::given(method("POST"))
        .and(path("/_open/auth"))
        .and(body_string_contains("username"))
        .and(body_string_contains("password"))
        .respond_with(mock_response)
        .mount(&mock_server)
        .await;
}

#[cfg(test)]
pub(crate) async fn default_conn<T>(uri: T) -> Result<Connection>
where
    T: Into<String>,
{
    ConnectionBuilder::default()
        .url(uri)
        .username("root")
        .password("")
        .database("keti")
        .build()
        .await
}

#[cfg(test)]
pub(crate) async fn default_conn_async<T>(uri: T) -> Result<Connection>
where
    T: Into<String>,
{
    ConnectionBuilder::default()
        .url(uri)
        .username("root")
        .password("")
        .database("keti")
        .async_kind(AsyncKind::Store)
        .build()
        .await
}

#[cfg(test)]
pub(crate) async fn no_db_conn<T>(uri: T) -> Result<Connection>
where
    T: Into<String>,
{
    ConnectionBuilder::default()
        .url(uri)
        .username("root")
        .password("")
        .build()
        .await
}

#[cfg(test)]
pub(crate) async fn no_db_conn_async<T>(uri: T) -> Result<Connection>
where
    T: Into<String>,
{
    ConnectionBuilder::default()
        .url(uri)
        .username("root")
        .password("")
        .async_kind(AsyncKind::Store)
        .build()
        .await
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! mock_test {
    ($conn_ty:ident, $code:literal, $name:ident, $res:ident; $api:ident($($args:expr),*); $($mock:ident),+ => $asserts: block) => {
        #[tokio::test]
        async fn $name() -> Result<()> {
            let mock_server = MockServer::start().await;
            mock_auth(&mock_server).await;
            $(
                $mock(&mock_server).await;
            )+

            let conn = $conn_ty(mock_server.uri()).await?;
            let $res = conn.$api($($args),*).await?;

            assert_eq!(*$res.code(), $code);
            assert!(!$res.error());
            $asserts

            Ok(())
        }
    };
    ($code:literal, $($tail:tt)*) => {
        mock_test!(default_conn, $code, $($tail)*);
    };
    ($($tail:tt)*) => {
        mock_test!(200, $($tail)*);
    };
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! mock_test_async {
    ($conn_ty:ident, $name:ident, $res:ident; $api:ident($($args:expr),*); $($mock:ident),+ => $asserts: block) => {
        #[tokio::test]
        async fn $name() -> Result<()> {
            let mock_server = MockServer::start().await;
            mock_auth(&mock_server).await;
            $(
                $mock(&mock_server).await;
            )+

            let conn = $conn_ty(mock_server.uri()).await?;
            let $res = conn.$api($($args),*).await?;

            assert!($res.is_left());
            $asserts

            Ok(())
        }
    };
    ($($tail:tt)*) => {
        mock_test_async!(default_conn_async, $($tail)*);
    };
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! mock_test_right {
    ($conn_ty:ident, $code:literal, $name:ident, $res:ident; $api:ident($($args:expr),*); $($mock:ident),+ => $asserts: block) => {
        #[tokio::test]
        async fn $name() -> Result<()> {
            let mock_server = MockServer::start().await;
            mock_auth(&mock_server).await;
            $(
                $mock(&mock_server).await;
            )+

            let conn = $conn_ty(mock_server.uri()).await?;
            let res = conn.$api($($args),*).await?;

            assert!(res.is_right());
            let $res = res.right_safe()?;
            assert!(!$res.error());
            assert_eq!(*$res.code(), $code);
            $asserts

            Ok(())
        }
    };
    ($code:literal, $($tail:tt)*) => {
        mock_test_right!(default_conn, $code, $($tail)*);
    };
    ($($tail:tt)*) => {
        mock_test_right!(200, $($tail)*);
    };
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! mock_async {
    ($fn_name:ident, $code:expr, $method:literal, $($matcher:expr),*) => {
        pub(crate) async fn $fn_name(mock_server: &MockServer) {
            let mock_response = ResponseTemplate::new($code).insert_header("x-arango-async-id", "123456");

            let mut mock_builder = Mock::given(method($method));

            $(
                mock_builder = mock_builder.and($matcher);
            )*

            mock_builder.respond_with(mock_response)
                .mount(&mock_server)
                .await;
        }
    };
    ($fn_name:ident, $method:literal, $($matcher:expr),*) => {
        mock_async!($fn_name, 202, $method, $($matcher),*);
    };
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! mock_x {
    ($fn_name:ident, $resp:ty, $code:expr => with_set, $method:literal, $($matcher:expr),*) => {
        pub(crate) async fn $fn_name(mock_server: &MockServer) {
            let mut body = <$resp>::default();
            let _ = body.set_code($code);
            let mock_response = ResponseTemplate::new($code).set_body_json(body);

            let mut mock_builder = Mock::given(method($method));

            $(
                mock_builder = mock_builder.and($matcher);
            )*

            mock_builder.respond_with(mock_response)
                .mount(&mock_server)
                .await;
        }
    };
    ($fn_name:ident, $resp:ty, $method:literal, $($matcher:expr),*) => {
        mock_x!($fn_name, $resp, 200 => with_set, $method, $($matcher),*);
    };
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! mock_res {
    ($fn_name:ident, $resp:expr, $code:expr, $method:literal, $($matcher:expr),*) => {
        pub(crate) async fn $fn_name(mock_server: &MockServer) -> Result<()> {
            let body = $resp;
            let mock_response = ResponseTemplate::new($code).set_body_json(body);

            let mut mock_builder = Mock::given(method($method));

            $(
                mock_builder = mock_builder.and($matcher);
            )*

            mock_builder.respond_with(mock_response)
                .up_to_n_times(1)
                .mount(&mock_server)
                .await;
            Ok(())
        }
    };
    ($fn_name:ident, $resp:expr, $method:literal, $($matcher:expr),*) => {
        mock_res!($fn_name, $resp, 200, $method, $($matcher),*);
    };
}

#[cfg(test)]
pub(crate) mod mocks {
    use anyhow::Result;

    pub(crate) trait Mock<T>
    where
        T: PartialEq,
    {
        fn try_mock(name: T) -> Result<Self>
        where
            Self: Sized;
    }

    pub(crate) mod collection {
        use crate::{
            coll::output::{
                Checksum, Collection, Collections, Count, Create, Drop, Figures, Load, LoadIndexes,
                ModifyProps, RecalculateCount, Rename, Revision, Truncate, Unload,
            },
            common::output::Response,
        };
        use wiremock::{
            matchers::{body_string_contains, method, path, query_param},
            Mock, MockServer, ResponseTemplate,
        };

        mock_x!(
            mock_unload,
            Unload,
            "PUT",
            path("_db/keti/_api/collection/test_coll/unload")
        );

        mock_async!(
            mock_collection_async,
            "GET",
            path("_db/keti/_api/collection/keti")
        );

        mock_x!(
            mock_collection,
            Collection,
            "GET",
            path("_db/keti/_api/collection/keti")
        );

        mock_x!(
            mock_drop,
            Drop,
            "DELETE",
            path("_db/keti/_api/collection/test_coll")
        );

        mock_x!(
            mock_create,
            Create,
            "POST",
            path("_db/keti/_api/collection"),
            body_string_contains("test_coll")
        );

        mock_x!(
            mock_checksum,
            Checksum,
            "GET",
            path("_db/keti/_api/collection/test_coll/checksum")
        );

        mock_x!(
            mock_count,
            Count,
            "GET",
            path("_db/keti/_api/collection/test_coll/count")
        );

        mock_x!(
            mock_figures,
            Figures,
            "GET",
            path("_db/keti/_api/collection/test_coll/figures")
        );

        mock_x!(
            mock_revision,
            Revision,
            "GET",
            path("_db/keti/_api/collection/test_coll/revision")
        );

        mock_x!(
            mock_load,
            Load,
            "PUT",
            path("_db/keti/_api/collection/test_coll/load"),
            body_string_contains("count")
        );

        mock_x!(
            mock_load_indexes,
            LoadIndexes,
            "PUT",
            path("_db/keti/_api/collection/test_coll/loadIndexesIntoMemory")
        );

        mock_x!(
            mock_modify_props,
            ModifyProps,
            "PUT",
            path("_db/keti/_api/collection/test_coll/properties"),
            body_string_contains("waitForSync")
        );

        mock_x!(
            mock_recalculate,
            RecalculateCount,
            "PUT",
            path("_db/keti/_api/collection/test_coll/recalculateCount")
        );

        mock_x!(
            mock_rename,
            Rename,
            "PUT",
            path("_db/keti/_api/collection/test_coll/rename"),
            body_string_contains("test_boll")
        );

        mock_x!(
            mock_truncate,
            Truncate,
            "PUT",
            path("_db/keti/_api/collection/test_coll/truncate")
        );

        mock_async!(
            mock_collections_async,
            "GET",
            path("_db/keti/_api/collection")
        );

        mock_x!(
            mock_collections,
            Response<Vec<Collections>>,
            "GET",
            path("_db/keti/_api/collection")
        );

        mock_async!(
            mock_collections_exclude_async,
            "GET",
            path("_db/keti/_api/collection"),
            query_param("excludeSystem", "true")
        );

        mock_x!(
            mock_collections_exclude,
            Response<Vec<Collections>>,
            "GET",
            path("_db/keti/_api/collection"),
            query_param("excludeSystem", "true")
        );
    }

    pub(crate) mod db {
        use crate::{common::output::Response, db::output::Current};
        use wiremock::{
            matchers::{body_string_contains, method, path},
            Mock, MockServer, ResponseTemplate,
        };

        mock_async!(
            mock_current_async,
            "GET",
            path("_db/keti/_api/database/current")
        );

        mock_x!(
            mock_current,
            Response::<Current>,
            "GET",
            path("_db/keti/_api/database/current")
        );

        mock_async!(mock_user_async, "GET", path("_db/keti/_api/database/user"));

        mock_x!(
            mock_user,
            Response::<Vec<String>>,
            "GET",
            path("_db/keti/_api/database/user")
        );

        mock_async!(mock_list_async, "GET", path("_api/database"));

        mock_x!(
            mock_list,
            Response::<Vec<String>>,
            "GET",
            path("_api/database")
        );

        mock_x!(
            mock_create,
            Response::<bool>,
            201 => with_set,
            "POST",
            path("_api/database"),
            body_string_contains("test_db")
        );

        mock_x!(
            mock_drop,
            Response::<bool>,
            "DELETE",
            path("_api/database/test_db")
        );
    }

    pub(crate) mod doc {
        use super::Mock as RuarangoMock;
        use crate::doc::output::{CreateMockKind, DocMeta, OutputDoc, ReadMockKind};
        use anyhow::Result;
        use wiremock::{
            matchers::{body_string_contains, header_exists, method, path, query_param},
            Mock, MockServer, ResponseTemplate,
        };

        mock_res!(
            mock_create,
            DocMeta::<(), ()>::default(),
            201,
            "POST",
            path("_db/keti/_api/document/test_coll"),
            body_string_contains("test")
        );

        mock_res!(
            mock_create_1,
            DocMeta::<(), ()>::try_mock(CreateMockKind::FirstCreate)?,
            201,
            "POST",
            path("_db/keti/_api/document/test_coll"),
            body_string_contains("test_key")
        );

        mock_res!(
            mock_create_2,
            DocMeta::<(), ()>::try_mock(CreateMockKind::SecondCreate)?,
            201,
            "POST",
            path("_db/keti/_api/document/test_coll"),
            body_string_contains("test_key")
        );
        mock_res!(
            mock_return_new,
            DocMeta::<OutputDoc, ()>::try_mock(CreateMockKind::NewDoc)?,
            201,
            "POST",
            path("_db/keti/_api/document/test_coll"),
            query_param("returnNew", "true")
        );
        mock_res!(
            mock_return_old,
            DocMeta::<OutputDoc, OutputDoc>::try_mock(CreateMockKind::NewOldDoc)?,
            201,
            "POST",
            path("_db/keti/_api/document/test_coll"),
            body_string_contains("test_key"),
            query_param("returnNew", "true"),
            query_param("returnOld", "true")
        );
        mock_res!(
            mock_read,
            OutputDoc::try_mock(ReadMockKind::Found)?,
            "GET",
            path("_db/keti/_api/document/test_coll/test_doc")
        );
        mock_res!(
            mock_read_if_match,
            OutputDoc::try_mock(ReadMockKind::Found)?,
            "GET",
            path("_db/keti/_api/document/test_coll/test_doc"),
            header_exists("if-match")
        );
    }
}

#[cfg(test)]
mod test {
    use super::prepend_sep;

    #[test]
    fn has_no_qp() {
        let mut result = String::new();
        assert_eq!("?", prepend_sep(&mut result, false));
    }

    #[test]
    fn has_qp() {
        let mut result = String::new();
        assert_eq!("&", prepend_sep(&mut result, true));
    }
}
