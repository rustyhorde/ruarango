// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` collection trait implementation

use crate::{
    api_get,
    conn::Connection,
    model::{
        coll::{Collection as Coll, CollectionInfo},
        Response,
    },
    traits::Collection,
    utils::handle_response,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use const_format::concatcp;
use futures::FutureExt;

const BASE_SUFFIX: &str = "_api/collection";
const EXCLUDE_SUFFIX: &str = concatcp!(BASE_SUFFIX, "?excludeSystem=true");

#[async_trait]
impl Collection for Connection {
    async fn collections(&self, exclude_system: bool) -> Result<Response<Vec<Coll>>> {
        if exclude_system {
            api_get!(self, db_url, EXCLUDE_SUFFIX)
        } else {
            api_get!(self, db_url, BASE_SUFFIX)
        }
    }

    async fn collection(&self, name: &str) -> Result<CollectionInfo> {
        api_get!(self, db_url, &format!("{}/{}", BASE_SUFFIX, name))
    }
}

#[cfg(test)]
mod test {
    use super::Collection;
    use crate::{
        model::{
            coll::{Collection as Coll, CollectionInfo},
            Response,
        },
        utils::{default_conn, mock_auth},
    };
    use anyhow::Result;
    use wiremock::{
        matchers::{method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    };

    async fn mock_collections(mock_server: &MockServer, exclude: bool) {
        let body = Response::<Vec<Coll>>::default();
        let mock_response = ResponseTemplate::new(200).set_body_json(body);

        let mut mock = Mock::given(method("GET")).and(path("/_db/keti/_api/collection"));

        if exclude {
            mock = mock.and(query_param("excludeSystem", "true"));
        }

        mock.respond_with(mock_response).mount(&mock_server).await;
    }

    async fn mock_collection(mock_server: &MockServer) {
        let body = CollectionInfo::default();
        let mock_response = ResponseTemplate::new(200).set_body_json(body);

        Mock::given(method("GET"))
            .and(path("/_db/keti/_api/collection/keti"))
            .respond_with(mock_response)
            .mount(&mock_server)
            .await;
    }

    #[tokio::test]
    async fn get_collections_works() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_collections(&mock_server, true).await;

        let conn = default_conn(mock_server.uri()).await?;
        let res = conn.collections(true).await?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        assert!(res.result().len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn get_collections_with_sys_works() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_collections(&mock_server, false).await;

        let conn = default_conn(mock_server.uri()).await?;
        let res = conn.collections(false).await?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        assert!(res.result().len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn get_collection() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_collection(&mock_server).await;

        let conn = default_conn(mock_server.uri()).await?;
        let res = conn.collection("keti").await?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        assert_eq!(*res.kind(), 2);
        assert_eq!(*res.status(), 3);
        assert!(!res.is_system());
        assert_eq!(res.name(), "keti");
        assert_eq!(res.id(), "5847");
        assert_eq!(res.globally_unique_id(), "hD4537D142F4C/5847");
        Ok(())
    }
}
