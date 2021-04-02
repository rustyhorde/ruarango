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

#[cfg(test)]
use {
    crate::{
        builder::ConnectionBuilder,
        conn::Connection,
        error::RuarangoError::{self, TestError},
        model::AuthResponse,
    },
    anyhow::Result,
    wiremock::{
        matchers::{body_string_contains, method, path},
        Mock, MockServer, ResponseTemplate,
    },
};

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
    ConnectionBuilder::new()
        .url(uri)
        .username("root")
        .password("")
        .database("keti")
        .build()
        .await
}

#[cfg(test)]
pub(crate) async fn no_db_conn<T>(uri: T) -> Result<Connection>
where
    T: Into<String>,
{
    ConnectionBuilder::new()
        .url(uri)
        .username("root")
        .password("")
        .build()
        .await
}

#[allow(dead_code)]
#[cfg(test)]
pub(crate) fn to_test_error(val: String) -> RuarangoError {
    TestError { val }
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
macro_rules! mock_x {
    ($fn_name:ident, $resp:ty, $code:expr, $method:literal, $($matcher:expr),*) => {
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
        pub(crate) async fn $fn_name(mock_server: &MockServer) {
            let body = <$resp>::default();
            let mock_response = ResponseTemplate::new(200).set_body_json(body);

            let mut mock_builder = Mock::given(method($method));

            $(
                mock_builder = mock_builder.and($matcher);
            )*

            mock_builder.respond_with(mock_response)
                .mount(&mock_server)
                .await;
        }
    };
}

#[cfg(test)]
pub(crate) mod mocks {
    pub(crate) mod collection {
        use crate::{
            coll::{
                ChecksumResponse, CountResponse, CreateCollResponse, DropCollResponse,
                FiguresResponse, GetCollResponse, GetCollsResponse, LoadIndexesResponse,
                LoadResponse, PutPropertiesResponse, RecalculateCountResponse, RenameResponse,
                RevisionResponse, TruncateResponse, UnloadResponse,
            },
            model::Response,
        };
        use wiremock::{
            matchers::{body_string_contains, method, path, query_param},
            Mock, MockServer, ResponseTemplate,
        };

        mock_x!(
            mock_unload,
            UnloadResponse,
            "PUT",
            path("_db/keti/_api/collection/test_coll/unload")
        );

        mock_x!(
            mock_collection,
            GetCollResponse,
            "GET",
            path("_db/keti/_api/collection/keti")
        );

        mock_x!(
            mock_drop,
            DropCollResponse,
            "DELETE",
            path("_db/keti/_api/collection/test_coll")
        );

        mock_x!(
            mock_create,
            CreateCollResponse,
            "POST",
            path("_db/keti/_api/collection"),
            body_string_contains("test_coll")
        );

        mock_x!(
            mock_checksum,
            ChecksumResponse,
            "GET",
            path("_db/keti/_api/collection/test_coll/checksum")
        );

        mock_x!(
            mock_count,
            CountResponse,
            "GET",
            path("_db/keti/_api/collection/test_coll/count")
        );

        mock_x!(
            mock_figures,
            FiguresResponse,
            "GET",
            path("_db/keti/_api/collection/test_coll/figures")
        );

        mock_x!(
            mock_revision,
            RevisionResponse,
            "GET",
            path("_db/keti/_api/collection/test_coll/revision")
        );

        mock_x!(
            mock_load,
            LoadResponse,
            "PUT",
            path("_db/keti/_api/collection/test_coll/load"),
            body_string_contains("count")
        );

        mock_x!(
            mock_load_indexes,
            LoadIndexesResponse,
            "PUT",
            path("_db/keti/_api/collection/test_coll/loadIndexesIntoMemory")
        );

        mock_x!(
            mock_modify_props,
            PutPropertiesResponse,
            "PUT",
            path("_db/keti/_api/collection/test_coll/properties"),
            body_string_contains("waitForSync")
        );

        mock_x!(
            mock_recalculate,
            RecalculateCountResponse,
            "PUT",
            path("_db/keti/_api/collection/test_coll/recalculateCount")
        );

        mock_x!(
            mock_rename,
            RenameResponse,
            "PUT",
            path("_db/keti/_api/collection/test_coll/rename"),
            body_string_contains("test_boll")
        );

        mock_x!(
            mock_truncate,
            TruncateResponse,
            "PUT",
            path("_db/keti/_api/collection/test_coll/truncate")
        );

        mock_x!(
            mock_collections,
            Response<Vec<GetCollsResponse>>,
            "GET",
            path("_db/keti/_api/collection")
        );

        mock_x!(
            mock_collections_exclude,
            Response<Vec<GetCollsResponse>>,
            "GET",
            path("_db/keti/_api/collection"),
            query_param("excludeSystem", "true")
        );
    }

    pub(crate) mod db {
        use crate::{db::Current, model::Response};
        use wiremock::{
            matchers::{body_string_contains, method, path},
            Mock, MockServer, ResponseTemplate,
        };

        mock_x!(
            mock_current,
            Response::<Current>,
            "GET",
            path("_db/keti/_api/database/current")
        );

        mock_x!(
            mock_user,
            Response::<Vec<String>>,
            "GET",
            path("_db/keti/_api/database/user")
        );

        mock_x!(
            mock_list,
            Response::<Vec<String>>,
            "GET",
            path("_api/database")
        );

        mock_x!(
            mock_create,
            Response::<bool>,
            201,
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
}
