// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` collection trait implementation

use crate::{
    api_delete, api_get, api_post, api_put,
    conn::Connection,
    model::{
        coll::{
            ChecksumResponse, CollectionCreate, CountResponse, CreateCollResponse,
            DropCollResponse, FiguresResponse, GetCollResponse, GetCollsResponse,
            LoadIndexesResponse, LoadResponse, NewNameBuilder, Props, PutPropertiesResponse,
            RecalculateCountResponse, RenameResponse, RevisionResponse, ShouldCountBuilder,
            TruncateResponse, UnloadResponse,
        },
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
    async fn collections(&self, exclude_system: bool) -> Result<Response<Vec<GetCollsResponse>>> {
        if exclude_system {
            api_get!(self, db_url, EXCLUDE_SUFFIX)
        } else {
            api_get!(self, db_url, BASE_SUFFIX)
        }
    }

    async fn collection(&self, name: &str) -> Result<GetCollResponse> {
        api_get!(self, db_url, &format!("{}/{}", BASE_SUFFIX, name))
    }

    async fn create(&self, create: &CollectionCreate) -> Result<CreateCollResponse> {
        api_post!(self, db_url, BASE_SUFFIX, create)
    }

    async fn drop(&self, name: &str, is_system: bool) -> Result<DropCollResponse> {
        if is_system {
            api_delete!(
                self,
                db_url,
                &format!("{}/{}?isSystem=true", BASE_SUFFIX, name)
            )
        } else {
            api_delete!(self, db_url, &format!("{}/{}", BASE_SUFFIX, name))
        }
    }

    async fn checksum(
        &self,
        name: &str,
        with_revisions: bool,
        with_data: bool,
    ) -> Result<ChecksumResponse> {
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
        api_get!(self, db_url, &url)
    }

    async fn count(&self, name: &str) -> Result<CountResponse> {
        api_get!(self, db_url, &format!("{}/{}/count", BASE_SUFFIX, name))
    }

    async fn figures(&self, name: &str) -> Result<FiguresResponse> {
        api_get!(self, db_url, &format!("{}/{}/figures", BASE_SUFFIX, name))
    }

    async fn revision(&self, name: &str) -> Result<RevisionResponse> {
        api_get!(self, db_url, &format!("{}/{}/revision", BASE_SUFFIX, name))
    }

    async fn load(&self, name: &str, count: bool) -> Result<LoadResponse> {
        api_put!(
            self,
            db_url,
            &format!("{}/{}/load", BASE_SUFFIX, name),
            &ShouldCountBuilder::default().count(count).build()?
        )
    }

    async fn load_indexes(&self, name: &str) -> Result<LoadIndexesResponse> {
        api_put!(
            self,
            db_url,
            &format!("{}/{}/loadIndexesIntoMemory", BASE_SUFFIX, name)
        )
    }

    async fn modify_props(&self, name: &str, props: Props) -> Result<PutPropertiesResponse> {
        api_put!(
            self,
            db_url,
            &format!("{}/{}/properties", BASE_SUFFIX, name),
            &props
        )
    }

    async fn recalculate_count(&self, name: &str) -> Result<RecalculateCountResponse> {
        api_put!(
            self,
            db_url,
            &format!("{}/{}/recalculateCount", BASE_SUFFIX, name)
        )
    }

    async fn rename(&self, name: &str, new_name: &str) -> Result<RenameResponse> {
        api_put!(
            self,
            db_url,
            &format!("{}/{}/rename", BASE_SUFFIX, name),
            &NewNameBuilder::default().name(new_name).build()?
        )
    }

    async fn truncate(&self, name: &str) -> Result<TruncateResponse> {
        api_put!(self, db_url, &format!("{}/{}/truncate", BASE_SUFFIX, name))
    }

    async fn unload(&self, name: &str) -> Result<UnloadResponse> {
        api_put!(self, db_url, &format!("{}/{}/unload", BASE_SUFFIX, name))
    }
}

#[cfg(test)]
mod test {
    use super::Collection;
    use crate::{
        mock_test,
        model::coll::{CollectionCreateBuilder, PropsBuilder},
        utils::{
            default_conn, mock_auth,
            mocks::collection::{
                mock_checksum, mock_collection, mock_collections, mock_collections_exclude,
                mock_count, mock_create, mock_drop, mock_figures, mock_load, mock_load_indexes,
                mock_modify_props, mock_recalculate, mock_rename, mock_revision, mock_truncate,
                mock_unload,
            },
        },
    };
    use anyhow::Result;
    use wiremock::MockServer;

    mock_test!(get_collections, res; collections(true); mock_collections_exclude => {
        assert!(res.result().len() > 0);
    });

    mock_test!(get_collections_with_sys_works, res; collections(false); mock_collections => {
        assert!(res.result().len() > 0);
    });

    mock_test!(get_collection, res; collection("keti"); mock_collection => {
        assert_eq!(*res.kind(), 2);
        assert_eq!(*res.status(), 3);
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
        let create = CollectionCreateBuilder::default()
            .name("test_coll")
            .build()?;

        let res = conn.create(&create).await?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        assert_eq!(res.name(), "test_coll");

        let res = conn.drop("test_coll", false).await?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        Ok(())
    }

    mock_test!(get_checksum, res; checksum("test_coll", false, false); mock_checksum => {
        assert_eq!(*res.kind(), 2);
        assert_eq!(*res.status(), 3);
        assert!(!res.is_system());
        assert_eq!(res.name(), "test_coll");
        assert_eq!(res.id(), "5847");
        assert_eq!(res.globally_unique_id(), "hD4537D142F4C/5847");
        assert_eq!(res.revision(), "_cF8MSCu---");
        assert_eq!(res.checksum(), "0");
    });

    mock_test!(get_count, res; count("test_coll"); mock_count => {
        assert_eq!(*res.count(), 10);
    });

    mock_test!(get_figures, res; figures("test_coll"); mock_figures => {
        assert_eq!(*res.figures().indexes().count(), 1);
        assert_eq!(*res.figures().indexes().size(), 0);
        assert_eq!(*res.figures().documents_size(), 0);
        assert!(!res.figures().cache_in_use());
        assert_eq!(*res.figures().cache_size(), 0);
        assert_eq!(*res.figures().cache_usage(), 0);
    });

    mock_test!(get_revision, res; revision("test_coll"); mock_revision => {});

    mock_test!(put_load, res; load("test_coll", true); mock_load => {
        assert_eq!(*res.count(), 10);
    });

    mock_test!(put_load_indexes, res; load_indexes("test_coll"); mock_load_indexes => {
        assert!(res.result());
    });

    #[tokio::test]
    async fn put_props() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_modify_props(&mock_server).await;

        let props = PropsBuilder::default().wait_for_sync(true).build()?;
        let conn = default_conn(mock_server.uri()).await?;
        let res = conn.modify_props("test_coll", props).await?;
        assert_eq!(*res.code(), 200);
        assert!(!res.error());
        assert!(res.wait_for_sync());

        Ok(())
    }

    mock_test!(put_recalculate, res; recalculate_count("test_coll"); mock_recalculate => {
        assert!(res.result());
        assert_eq!(*res.count(), 10);
    });

    mock_test!(put_rename, res; rename("test_coll", "test_boll"); mock_rename => {
        assert_eq!(res.name(), "test_boll");
    });

    mock_test!(put_truncate, res; truncate("test_coll"); mock_truncate => {});

    mock_test!(put_unload, res; unload("test_coll"); mock_unload => {});
}
