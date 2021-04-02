// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` database trait implementation

use crate::{
    api_delete, api_get, api_post,
    common::output::Response,
    conn::Connection,
    db::{input::Create, output::Current},
    traits::Database,
    utils::handle_response,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use const_format::concatcp;
use futures::FutureExt;

const BASE_SUFFIX: &str = "_api/database";
const USER_SUFFIX: &str = concatcp!(BASE_SUFFIX, "/user");
const CURRENT_SUFFIX: &str = concatcp!(BASE_SUFFIX, "/current");

#[async_trait]
impl Database for Connection {
    async fn current(&self) -> Result<Response<Current>> {
        api_get!(self, db_url, CURRENT_SUFFIX)
    }

    async fn user(&self) -> Result<Response<Vec<String>>> {
        api_get!(self, db_url, USER_SUFFIX)
    }

    async fn list(&self) -> Result<Response<Vec<String>>> {
        api_get!(self, base_url, BASE_SUFFIX)
    }

    async fn create(&self, create: &Create) -> Result<Response<bool>> {
        api_post!(self, base_url, BASE_SUFFIX, create)
    }

    async fn drop(&self, name: &str) -> Result<Response<bool>> {
        api_delete!(self, base_url, &format!("{}/{}", BASE_SUFFIX, name))
    }
}

#[cfg(test)]
mod test {
    use super::Database;
    use crate::{
        db::input::{CreateBuilder, OptionsBuilder, UserBuilder},
        mock_test,
        utils::{
            default_conn, mock_auth,
            mocks::db::{mock_create, mock_current, mock_drop, mock_list, mock_user},
            no_db_conn,
        },
    };
    use anyhow::Result;
    use wiremock::MockServer;

    mock_test!(test_current, res; current(); mock_current => {
        assert_eq!(res.result().name(), "test");
        assert_eq!(res.result().id(), "123");
        assert!(!res.result().is_system());
        assert_eq!(res.result().path(), "abcdef");
        assert!(res.result().sharding().is_none());
        assert!(res.result().replication_factor().is_none());
        assert!(res.result().write_concern().is_none());
    });

    mock_test!(test_user, res; user(); mock_user => {
        assert!(res.result().len() > 0);
    });

    mock_test!(no_db_conn, 200, test_list, res; list(); mock_list => {
        assert!(res.result().len() > 0);
        assert!(res.result().contains(&"_system".to_string()));
    });

    #[tokio::test]
    async fn test_create_drop() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_create(&mock_server).await;
        mock_drop(&&mock_server).await;

        let conn = no_db_conn(mock_server.uri()).await?;
        let options = OptionsBuilder::default().build()?;
        let users = UserBuilder::default()
            .username("test")
            .password("test")
            .active(true)
            .build()?;
        let create = CreateBuilder::default()
            .name("test_db")
            .options(options)
            .users(vec![users])
            .build()?;

        let res = conn.create(&create).await?;
        assert_eq!(*res.code(), 201);
        assert!(!res.error());
        assert!(res.result());

        let res = conn.drop("test_db").await?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        assert!(res.result());
        Ok(())
    }
}
