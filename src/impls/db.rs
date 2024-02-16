// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` database trait implementation

use crate::{
    api_delete_async, api_delete_right, api_get_async, api_get_right, api_post_async,
    api_post_right,
    common::output::Response,
    conn::Connection,
    db::{input::Create, output::Current},
    traits::{Database, JobInfo},
    types::ArangoResult,
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
    async fn current(&self) -> ArangoResult<Response<Current>> {
        if *self.is_async() {
            api_get_async!(self, db_url, CURRENT_SUFFIX)
        } else {
            api_get_right!(self, db_url, CURRENT_SUFFIX, Response<Current>)
        }
    }

    async fn user(&self) -> ArangoResult<Response<Vec<String>>> {
        if *self.is_async() {
            api_get_async!(self, db_url, USER_SUFFIX)
        } else {
            api_get_right!(self, db_url, USER_SUFFIX, Response<Vec<String>>)
        }
    }

    async fn list(&self) -> ArangoResult<Response<Vec<String>>> {
        if *self.is_async() {
            api_get_async!(self, base_url, BASE_SUFFIX)
        } else {
            api_get_right!(self, base_url, BASE_SUFFIX, Response<Vec<String>>)
        }
    }

    async fn create(&self, create: &Create) -> ArangoResult<Response<bool>> {
        if *self.is_async() {
            api_post_async!(self, base_url, BASE_SUFFIX, create)
        } else {
            api_post_right!(self, base_url, BASE_SUFFIX, Response<bool>, create)
        }
    }

    async fn drop(&self, name: &str) -> ArangoResult<Response<bool>> {
        if *self.is_async() {
            api_delete_async!(self, base_url, &format!("{BASE_SUFFIX}/{name}"))
        } else {
            api_delete_right!(
                self,
                base_url,
                &format!("{BASE_SUFFIX}/{name}"),
                Response<bool>
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::Database;
    use crate::{
        db::input::{CreateBuilder, OptionsBuilder, UserBuilder},
        mock_test_async, mock_test_right,
        utils::{
            default_conn, default_conn_async, mock_auth,
            mocks::db::{
                mock_create, mock_current, mock_current_async, mock_drop, mock_list,
                mock_list_async, mock_user, mock_user_async,
            },
            no_db_conn, no_db_conn_async,
        },
    };
    use anyhow::{anyhow, Result};
    use wiremock::MockServer;

    mock_test_async!(test_current_async, res; current(); mock_current_async => {
        let left = res.left_safe()?;
        assert_eq!(*left.code(), 202);
        assert!(left.id().is_some());
        let job_id = left.id().as_ref().ok_or_else(|| anyhow!("invalid job_id"))?;
        assert_eq!(job_id, "123456");
    });

    mock_test_right!(test_current, res; current(); mock_current => {
        assert_eq!(res.result().name(), "test");
        assert_eq!(res.result().id(), "123");
        assert!(!res.result().is_system());
        assert_eq!(res.result().path(), "abcdef");
        assert!(res.result().sharding().is_none());
        assert!(res.result().replication_factor().is_none());
        assert!(res.result().write_concern().is_none());
    });

    mock_test_async!(test_user_async, res; user(); mock_user_async => {
        let left = res.left_safe()?;
        assert_eq!(*left.code(), 202);
        assert!(left.id().is_some());
        let job_id = left.id().as_ref().ok_or_else(|| anyhow!("invalid job_id"))?;
        assert_eq!(job_id, "123456");
    });

    mock_test_right!(test_user, res; user(); mock_user => {
        assert!(!res.result().is_empty());
    });

    mock_test_async!(no_db_conn_async, test_list_async, res; list(); mock_list_async => {
        let left = res.left_safe()?;
        assert_eq!(*left.code(), 202);
        assert!(left.id().is_some());
        let job_id = left.id().as_ref().ok_or_else(|| anyhow!("invalid job_id"))?;
        assert_eq!(job_id, "123456");
    });

    mock_test_right!(no_db_conn, 200, test_list, res; list(); mock_list => {
        assert!(!res.result().is_empty());
        assert!(res.result().contains(&"_system".to_string()));
    });

    #[tokio::test]
    async fn test_create_drop() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_create(&mock_server).await;
        mock_drop(&mock_server).await;

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

        let either = conn.create(&create).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert_eq!(*res.code(), 201);
        assert!(!res.error());
        assert!(res.result());

        let either = conn.drop("test_db").await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        assert!(res.result());
        Ok(())
    }
}
