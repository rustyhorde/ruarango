// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Collection trait implementation

use crate::{
    api_delete_async, api_delete_right, api_get_async, api_get_right, api_post_async,
    api_post_right, api_put_async, api_put_right,
    coll::{
        input::{Config, NewNameBuilder, Props, ShouldCountBuilder},
        output::{
            Checksum, Collection as Coll, Collections, Count, Create, Drop, Figures, Load,
            LoadIndexes, ModifyProps, RecalculateCount, Rename, Revision, Truncate, Unload,
        },
    },
    common::output::Response,
    conn::Connection,
    traits::{Collection, JobInfo},
    types::ArangoResult,
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
    async fn collections(&self, exclude_system: bool) -> ArangoResult<Response<Vec<Collections>>> {
        if *self.is_async() {
            if exclude_system {
                api_get_async!(self, db_url, EXCLUDE_SUFFIX)
            } else {
                api_get_async!(self, db_url, BASE_SUFFIX)
            }
        } else if exclude_system {
            api_get_right!(self, db_url, EXCLUDE_SUFFIX, Response<Vec<Collections>>)
        } else {
            api_get_right!(self, db_url, BASE_SUFFIX, Response<Vec<Collections>>)
        }
    }

    async fn collection(&self, name: &str) -> ArangoResult<Coll> {
        let url = &format!("{}/{}", BASE_SUFFIX, name);
        if *self.is_async() {
            api_get_async!(self, db_url, url)
        } else {
            api_get_right!(self, db_url, url, Coll)
        }
    }

    async fn create(&self, config: &Config) -> ArangoResult<Create> {
        if *self.is_async() {
            api_post_async!(self, db_url, BASE_SUFFIX, config)
        } else {
            api_post_right!(self, db_url, BASE_SUFFIX, Create, config)
        }
    }

    async fn drop(&self, name: &str, is_system: bool) -> ArangoResult<Drop> {
        let url = &format!("{}/{}", BASE_SUFFIX, name);
        let is_system_url = &format!("{}/{}?isSystem=true", BASE_SUFFIX, name);
        if *self.is_async() {
            if is_system {
                api_delete_async!(self, db_url, is_system_url)
            } else {
                api_delete_async!(self, db_url, url)
            }
        } else if is_system {
            api_delete_right!(self, db_url, is_system_url, Drop)
        } else {
            api_delete_right!(self, db_url, url, Drop)
        }
    }

    async fn checksum(
        &self,
        name: &str,
        with_revisions: bool,
        with_data: bool,
    ) -> ArangoResult<Checksum> {
        let mut url = format!("{}/{}/checksum", BASE_SUFFIX, name);
        let mut has_qp = false;
        if with_revisions {
            url += "?withRevisions=true";
            has_qp = true;
        }
        if with_data {
            if has_qp {
                url += "&";
            } else {
                url += "?";
            }
            url += "withData=true";
        }

        if *self.is_async() {
            api_get_async!(self, db_url, &url)
        } else {
            api_get_right!(self, db_url, &url, Checksum)
        }
    }

    async fn count(&self, name: &str) -> ArangoResult<Count> {
        let url = &format!("{}/{}/count", BASE_SUFFIX, name);
        if *self.is_async() {
            api_get_async!(self, db_url, url)
        } else {
            api_get_right!(self, db_url, url, Count)
        }
    }

    async fn figures(&self, name: &str) -> ArangoResult<Figures> {
        let url = &format!("{}/{}/figures", BASE_SUFFIX, name);
        if *self.is_async() {
            api_get_async!(self, db_url, url)
        } else {
            api_get_right!(self, db_url, url, Figures)
        }
    }

    async fn revision(&self, name: &str) -> ArangoResult<Revision> {
        let url = &format!("{}/{}/revision", BASE_SUFFIX, name);
        if *self.is_async() {
            api_get_async!(self, db_url, url)
        } else {
            api_get_right!(self, db_url, url, Revision)
        }
    }

    async fn load(&self, name: &str, include_count: bool) -> ArangoResult<Load> {
        let url = &format!("{}/{}/load", BASE_SUFFIX, name);
        let should_count = &ShouldCountBuilder::default().count(include_count).build()?;
        if *self.is_async() {
            api_put_async!(self, db_url, url, should_count)
        } else {
            api_put_right!(self, db_url, url, Load, should_count)
        }
    }

    async fn load_indexes(&self, name: &str) -> ArangoResult<LoadIndexes> {
        let url = &format!("{}/{}/loadIndexesIntoMemory", BASE_SUFFIX, name);
        if *self.is_async() {
            api_put_async!(self, db_url, url)
        } else {
            api_put_right!(self, db_url, url, LoadIndexes)
        }
    }

    async fn modify_props(&self, name: &str, props: Props) -> ArangoResult<ModifyProps> {
        let url = &format!("{}/{}/properties", BASE_SUFFIX, name);
        if *self.is_async() {
            api_put_async!(self, db_url, url, &props)
        } else {
            api_put_right!(self, db_url, url, ModifyProps, &props)
        }
    }

    async fn recalculate_count(&self, name: &str) -> ArangoResult<RecalculateCount> {
        let url = &format!("{}/{}/recalculateCount", BASE_SUFFIX, name);

        if *self.is_async() {
            api_put_async!(self, db_url, url)
        } else {
            api_put_right!(self, db_url, url, RecalculateCount)
        }
    }

    async fn rename(&self, name: &str, new_name: &str) -> ArangoResult<Rename> {
        let url = &format!("{}/{}/rename", BASE_SUFFIX, name);
        let body = &NewNameBuilder::default().name(new_name).build()?;

        if *self.is_async() {
            api_put_async!(self, db_url, url, body)
        } else {
            api_put_right!(self, db_url, url, Rename, body)
        }
    }

    async fn truncate(&self, name: &str) -> ArangoResult<Truncate> {
        let url = &format!("{}/{}/truncate", BASE_SUFFIX, name);

        if *self.is_async() {
            api_put_async!(self, db_url, url)
        } else {
            api_put_right!(self, db_url, url, Truncate)
        }
    }

    async fn unload(&self, name: &str) -> ArangoResult<Unload> {
        let url = &format!("{}/{}/unload", BASE_SUFFIX, name);

        if *self.is_async() {
            api_put_async!(self, db_url, url)
        } else {
            api_put_right!(self, db_url, url, Unload)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Collection;
    use crate::{
        coll::{CollectionKind, Status},
        mock_test_async, mock_test_right,
        model::coll::input::{ConfigBuilder, PropsBuilder},
        utils::{
            default_conn, default_conn_async, mock_auth,
            mocks::collection::{
                mock_checksum, mock_collection, mock_collection_async, mock_collections,
                mock_collections_async, mock_collections_exclude, mock_collections_exclude_async,
                mock_count, mock_create, mock_drop, mock_figures, mock_load, mock_load_indexes,
                mock_modify_props, mock_recalculate, mock_rename, mock_revision, mock_truncate,
                mock_unload,
            },
        },
    };
    use anyhow::{anyhow, Result};
    use wiremock::MockServer;

    mock_test_async!(get_collections_async, res; collections(true); mock_collections_exclude_async => {
        let left = res.left_safe()?;
        assert_eq!(*left.code(), 202);
        assert!(left.id().is_some());
        let job_id = left.id().as_ref().ok_or_else(|| anyhow!("invalid job_id"))?;
        assert_eq!(job_id, "123456");
    });

    mock_test_right!(get_collections, res; collections(true); mock_collections_exclude => {
        assert!(!res.result().is_empty());
    });

    mock_test_async!(get_collections_with_sys_async, res; collections(true); mock_collections_async => {
        let left = res.left_safe()?;
        assert_eq!(*left.code(), 202);
        assert!(left.id().is_some());
        let job_id = left.id().as_ref().ok_or_else(|| anyhow!("invalid job_id"))?;
        assert_eq!(job_id, "123456");
    });

    mock_test_right!(get_collections_with_sys_works, res; collections(false); mock_collections => {
        assert!(!res.result().is_empty());
    });

    mock_test_async!(get_collection_async, res; collection("keti"); mock_collection_async => {
        let left = res.left_safe()?;
        assert_eq!(*left.code(), 202);
        assert!(left.id().is_some());
        let job_id = left.id().as_ref().ok_or_else(|| anyhow!("invalid job_id"))?;
        assert_eq!(job_id, "123456");
    });

    mock_test_right!(get_collection, res; collection("keti"); mock_collection => {
        assert_eq!(*res.kind(), CollectionKind::Document);
        assert_eq!(*res.status(), Status::Loaded);
        assert!(!res.is_system());
        assert_eq!(res.name(), "keti");
        assert_eq!(res.id(), "5847");
        assert_eq!(res.globally_unique_id(), "hD4537D142F4C/5847");
    });

    #[tokio::test]
    async fn create_then_drop() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_create(&mock_server).await;
        mock_drop(&mock_server).await;

        let conn = default_conn(mock_server.uri()).await?;
        let create = ConfigBuilder::default().name("test_coll").build()?;

        let either = conn.create(&create).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        assert_eq!(res.name(), "test_coll");

        let either = conn.drop("test_coll", false).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        Ok(())
    }

    mock_test_right!(get_checksum, res; checksum("test_coll", false, false); mock_checksum => {
        assert_eq!(res.checksum(), "0");
    });

    mock_test_right!(get_count, res; count("test_coll"); mock_count => {
        assert_eq!(*res.count(), 10);
    });

    mock_test_right!(get_figures, res; figures("test_coll"); mock_figures => {
        assert_eq!(*res.figures().indexes().count(), 1);
        assert_eq!(*res.figures().indexes().size(), 0);
        assert_eq!(*res.figures().documents_size(), 0);
        assert!(!res.figures().cache_in_use());
        assert_eq!(*res.figures().cache_size(), 0);
        assert_eq!(*res.figures().cache_usage(), 0);
    });

    mock_test_right!(get_revision, res; revision("test_coll"); mock_revision => {});

    mock_test_right!(put_load, res; load("test_coll", true); mock_load => {
        assert!(res.count().is_some());
        assert_eq!(res.count().unwrap(), 10);
    });

    mock_test_right!(put_load_indexes, res; load_indexes("test_coll"); mock_load_indexes => {
        assert!(res.result());
    });

    #[tokio::test]
    async fn put_props() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_modify_props(&mock_server).await;

        let props = PropsBuilder::default().wait_for_sync(true).build()?;
        let conn = default_conn(mock_server.uri()).await?;
        let either = conn.modify_props("test_coll", props).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        assert!(res.wait_for_sync());

        Ok(())
    }

    mock_test_right!(put_recalculate, res; recalculate_count("test_coll"); mock_recalculate => {
        assert!(res.result());
        assert_eq!(*res.count(), 10);
    });

    mock_test_right!(put_rename, res; rename("test_coll", "test_boll"); mock_rename => {
        assert_eq!(res.name(), "test_boll");
    });

    mock_test_right!(put_truncate, res; truncate("test_coll"); mock_truncate => {});

    mock_test_right!(put_unload, res; unload("test_coll"); mock_unload => {});
}
