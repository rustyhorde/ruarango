// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` integration tests

#[macro_use]
mod common;

mod coll {
    use super::common::{
        conn_root_system, conn_root_system_async, conn_ruarango, conn_ruarango_async,
        process_async_result, process_sync_result, rand_name,
    };
    use anyhow::Result;
    use lazy_static::lazy_static;
    use ruarango::{
        coll::{
            input::{Config, ConfigBuilder, Props, PropsBuilder},
            output::{
                Checksum, Collection as Coll, Collections, Count, Create, Figures, Load,
                LoadIndexes, ModifyProps, RecalculateCount, Revision,
            },
            CollectionKind, Status,
        },
        common::output::Response,
        Collection,
    };

    const TEST_COLL: &str = "test_coll";

    int_test_async!(res; Response<Vec<Collections>>; collection_collections_ruarango_no_system_async, conn_ruarango_async, collections(true) => {
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert!(res.result().len() > 0);
    });

    int_test_sync!(res; collection_collections_ruarango_no_system, conn_ruarango, collections(true) => {
        assert!(res.result().len() > 0);
    });

    int_test_async!(res; Response<Vec<Collections>>; collection_collections_root_no_system_async, conn_root_system_async, collections(true) => {
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert_eq!(res.result().len(), 0);
    });

    int_test_sync!(res; collection_collections_root_no_system, conn_root_system, collections(true) => {
        assert_eq!(res.result().len(), 0);
    });

    int_test_async!(res; Response<Vec<Collections>>; collection_collections_ruarango_async, conn_ruarango_async, collections(false) => {
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert!(res.result().len() > 0);
    });

    int_test_sync!(res; collection_collections_ruarango, conn_ruarango, collections(false) => {
        assert!(res.result().len() > 0);
    });

    int_test_async!(res; Response<Vec<Collections>>; collection_collections_root_async, conn_root_system_async, collections(false) => {
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert!(res.result().len() > 0);
    });

    int_test_sync!(res; collection_collections, conn_root_system, collections(false) => {
        assert!(res.result().len() > 0);
    });

    int_test_async!(res; Coll; collection_collection_async, conn_ruarango_async, collection(TEST_COLL) => {
        assert_eq!(*res.kind(), CollectionKind::Document);
        assert_eq!(*res.status(), Status::Loaded);
        assert!(!res.is_system());
        assert_eq!(res.name(), TEST_COLL);
        assert_eq!(res.id(), "898");
        assert_eq!(res.globally_unique_id(), "h963E57B880A3/898");
    });

    int_test_sync!(res; collection_collection, conn_ruarango, collection(TEST_COLL) => {
        assert_eq!(*res.kind(), CollectionKind::Document);
        assert_eq!(*res.status(), Status::Loaded);
        assert!(!res.is_system());
        assert_eq!(res.name(), TEST_COLL);
        assert_eq!(res.id(), "898");
        assert_eq!(res.globally_unique_id(), "h963E57B880A3/898");
    });

    lazy_static! {
        static ref COLL_NAME: String = rand_name();
        static ref COLL_NAME_ASYNC: String = rand_name();
        static ref RENAME_NAME: String = rand_name();
        static ref RENAME_NAME_ASYNC: String = rand_name();
        static ref RENAME_NEW_NAME: String = rand_name();
        static ref RENAME_NEW_NAME_ASYNC: String = rand_name();
        static ref TRUNCATE_NAME: String = rand_name();
        static ref TRUNCATE_NAME_ASYNC: String = rand_name();
        static ref UNLOAD_NAME: String = rand_name();
        static ref UNLOAD_NAME_ASYNC: String = rand_name();
    }

    enum CreateKind {
        Coll,
        CollAsync,
        Rename,
        RenameAsync,
        Truncate,
        TruncateAsync,
        Unload,
        UnloadAsync,
    }

    fn create_config(kind: CreateKind) -> Result<Config> {
        Ok(match kind {
            CreateKind::Coll => ConfigBuilder::default().name(&*COLL_NAME).build()?,
            CreateKind::CollAsync => ConfigBuilder::default().name(&*COLL_NAME_ASYNC).build()?,
            CreateKind::Rename => ConfigBuilder::default().name(&*RENAME_NAME).build()?,
            CreateKind::RenameAsync => {
                ConfigBuilder::default().name(&*RENAME_NAME_ASYNC).build()?
            }
            CreateKind::Truncate => ConfigBuilder::default().name(&*TRUNCATE_NAME).build()?,
            CreateKind::TruncateAsync => ConfigBuilder::default()
                .name(&*TRUNCATE_NAME_ASYNC)
                .build()?,
            CreateKind::Unload => ConfigBuilder::default().name(&*UNLOAD_NAME).build()?,
            CreateKind::UnloadAsync => {
                ConfigBuilder::default().name(&*UNLOAD_NAME_ASYNC).build()?
            }
        })
    }

    int_test_async!(res; conn; Create; collection_create_drop_async, conn_ruarango_async, create(&create_config(CreateKind::CollAsync)?) => {
        assert_eq!(res.name(), &*COLL_NAME_ASYNC);

        let res = conn.drop(&*COLL_NAME_ASYNC, false).await?;
        let res = process_async_result(res, &conn).await?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
    });

    int_test_sync!(res; conn; collection_create_drop, conn_ruarango, create(&create_config(CreateKind::Coll)?) => {
        assert_eq!(res.name(), &*COLL_NAME);

        let either = conn.drop(&*COLL_NAME, false).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
    });

    int_test_async!(res; Checksum; collection_checksum_async, conn_ruarango_async, checksum(TEST_COLL, false, false) => {
        assert!(!res.checksum().is_empty());

    });

    int_test_sync!(res; collection_checksum, conn_ruarango, checksum(TEST_COLL, false, false) => {
        assert!(!res.checksum().is_empty());
    });

    int_test_async!(res; Checksum; collection_checksum_1_async, conn_ruarango_async, checksum(TEST_COLL, true, false) => {
        assert!(!res.checksum().is_empty());
    });

    int_test_sync!(res; collection_checksum_1, conn_ruarango, checksum(TEST_COLL, true, false) => {
        assert!(!res.checksum().is_empty());
    });

    int_test_async!(res; Checksum; collection_checksum_2_async, conn_ruarango_async, checksum(TEST_COLL, false, true) => {
        assert!(!res.checksum().is_empty());
    });

    int_test_sync!(res; collection_checksum_2, conn_ruarango, checksum(TEST_COLL, false, true) => {
        assert!(!res.checksum().is_empty());
    });

    int_test_async!(res; Checksum; collection_checksum_3_async, conn_ruarango_async, checksum(TEST_COLL, true, true) => {
        assert!(!res.checksum().is_empty());
    });

    int_test_sync!(res; collection_checksum_3, conn_ruarango, checksum(TEST_COLL, true, true) => {
        assert!(!res.checksum().is_empty());
    });

    int_test_async!(res; Count; collection_count_async, conn_ruarango_async, count(TEST_COLL) => {
        assert!(*res.count() >= 1);
    });

    int_test_sync!(res; collection_count, conn_ruarango, count(TEST_COLL) => {
        assert!(*res.count() >= 1);
    });

    int_test_async!(res; Figures; collection_figures_async, conn_ruarango_async, figures(TEST_COLL) => {
        assert!(*res.figures().indexes().count() >= 1);
        assert!(*res.figures().indexes().size() > 0);
        assert!(*res.figures().documents_size() > 0);
        assert!(!res.figures().cache_in_use());
        assert_eq!(*res.figures().cache_size(), 0);
        assert_eq!(*res.figures().cache_usage(), 0);
    });

    int_test_sync!(res; collection_figures, conn_ruarango, figures(TEST_COLL) => {
        assert!(*res.figures().indexes().count() >= 1);
        assert!(*res.figures().indexes().size() > 0);
        assert!(*res.figures().documents_size() > 0);
        assert!(!res.figures().cache_in_use());
        assert_eq!(*res.figures().cache_size(), 0);
        assert_eq!(*res.figures().cache_usage(), 0);
    });

    int_test_async!(res; Revision; collection_revision_async, conn_ruarango_async, revision(TEST_COLL) => {
        assert!(!res.revision().is_empty());
    });

    int_test_sync!(res; collection_revision, conn_ruarango, revision(TEST_COLL) => {
        assert!(!res.revision().is_empty());
    });

    int_test_async!(res; Load; collection_load_async, conn_ruarango_async, load(TEST_COLL, false) => {
        assert!(res.count().is_none());
    });

    int_test_sync!(res; collection_load, conn_ruarango, load(TEST_COLL, false) => {
        assert!(res.count().is_none());
    });

    int_test_async!(res; Load; collection_load_1_async, conn_ruarango_async, load(TEST_COLL, true) => {
        assert!(res.count().is_some());
        assert_eq!(res.count().unwrap(), 1);
    });

    int_test_sync!(res; collection_load_1, conn_ruarango, load(TEST_COLL, true) => {
        assert!(res.count().is_some());
        assert_eq!(res.count().unwrap(), 1);
    });

    int_test_async!(res; LoadIndexes; collection_load_indexes_async, conn_ruarango_async, load_indexes(TEST_COLL) => {
        assert!(res.result());
    });

    int_test_sync!(res; collection_load_indexes, conn_ruarango, load_indexes(TEST_COLL) => {
        assert!(res.result());
    });

    fn props_config(wait_for_sync: bool) -> Result<Props> {
        Ok(PropsBuilder::default()
            .wait_for_sync(wait_for_sync)
            .build()?)
    }

    int_test_async!(res; conn; ModifyProps; collection_modify_props_async, conn_ruarango_async, modify_props(TEST_COLL, props_config(true)?) => {
        assert!(res.wait_for_sync());
        let either = conn.modify_props(TEST_COLL, props_config(false)?).await?;
        let res = process_async_result(either, &conn).await?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert!(!res.wait_for_sync());
    });

    int_test_sync!(res; conn; collection_modify_props, conn_ruarango, modify_props(TEST_COLL, props_config(true)?) => {
        assert!(res.wait_for_sync());
        let either = conn.modify_props(TEST_COLL, props_config(false)?).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert!(!res.wait_for_sync());
    });

    int_test_async!(res; RecalculateCount; collection_recalculate_count_async, conn_ruarango_async, recalculate_count(TEST_COLL) => {
        assert!(res.result());
        assert_eq!(*res.count(), 1);
    });

    int_test_sync!(res; collection_recalculate_count, conn_ruarango, recalculate_count(TEST_COLL) => {
        assert!(res.result());
        assert_eq!(*res.count(), 1);
    });

    int_test_async!(res; conn; Create; collection_rename_async, conn_ruarango_async, create(&create_config(CreateKind::RenameAsync)?) => {
        assert_eq!(res.name(), &*RENAME_NAME_ASYNC);

        let either = conn.rename(&*RENAME_NAME_ASYNC, &RENAME_NEW_NAME_ASYNC).await?;
        let res = process_async_result(either, &conn).await?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert_eq!(res.name(), &*RENAME_NEW_NAME_ASYNC);

        let either = conn.drop(&*RENAME_NEW_NAME_ASYNC, false).await?;
        let res = process_async_result(either, &conn).await?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
    });

    int_test_sync!(res; conn; collection_rename, conn_ruarango, create(&create_config(CreateKind::Rename)?) => {
        assert_eq!(res.name(), &*RENAME_NAME);

        let either = conn.rename(&*RENAME_NAME, &RENAME_NEW_NAME).await?;
        let res = process_sync_result(either)?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert_eq!(res.name(), &*RENAME_NEW_NAME);

        let either = conn.drop(&*RENAME_NEW_NAME, false).await?;
        let res = process_sync_result(either)?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
    });

    int_test_async!(res; conn; Create; collection_truncate_async, conn_ruarango_async, create(&create_config(CreateKind::TruncateAsync)?) => {
        assert_eq!(res.name(), &*TRUNCATE_NAME_ASYNC);

        let either = conn.truncate(&*TRUNCATE_NAME_ASYNC).await?;
        let res = process_async_result(either, &conn).await?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);

        let either = conn.drop(&*TRUNCATE_NAME_ASYNC, false).await?;
        let res = process_async_result(either, &conn).await?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
    });

    int_test_sync!(res; conn; collection_truncate, conn_ruarango, create(&create_config(CreateKind::Truncate)?) => {
        assert_eq!(res.name(), &*TRUNCATE_NAME);

        let either = conn.truncate(&*TRUNCATE_NAME).await?;
        let res = process_sync_result(either)?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);

        let either = conn.drop(&*TRUNCATE_NAME, false).await?;
        let res = process_sync_result(either)?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
    });

    int_test_async!(res; conn; Create; collection_unload_async, conn_ruarango_async, create(&create_config(CreateKind::UnloadAsync)?) => {
        assert_eq!(res.name(), &*UNLOAD_NAME_ASYNC);

        let either = conn.unload(&*UNLOAD_NAME_ASYNC).await?;
        let _res = process_async_result(either, &conn).await?;

        let either = conn.drop(&*UNLOAD_NAME_ASYNC, false).await?;
        let res = process_async_result(either, &conn).await?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
    });

    int_test_sync!(res; conn; collection_unload, conn_ruarango, create(&create_config(CreateKind::Unload)?) => {
        assert_eq!(res.name(), &*UNLOAD_NAME);

        let either = conn.unload(&*UNLOAD_NAME).await?;
        let _res = process_sync_result(either)?;

        let either = conn.drop(&*UNLOAD_NAME, false).await?;
        let res = process_sync_result(either)?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
    });
}

mod db {
    use super::common::{
        conn_root_system, conn_root_system_async, conn_ruarango, conn_ruarango_async,
        process_async_result, process_sync_result, rand_name,
    };
    use anyhow::Result;
    use lazy_static::lazy_static;
    use ruarango::{
        common::output::Response,
        db::{
            input::{Create, CreateBuilder},
            output::Current,
        },
        Database,
    };

    int_test_async!(res; Response<Current>; database_current_async, conn_ruarango_async, current() => {
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert_eq!(res.result().name(), "ruarango");
        assert_eq!(res.result().id(), "415");
        assert!(!res.result().is_system());
        assert_eq!(res.result().path(), "none");
        assert!(res.result().sharding().is_none());
        assert!(res.result().replication_factor().is_none());
        assert!(res.result().write_concern().is_none());
    });

    int_test_sync!(res; database_current, conn_ruarango, current() => {
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert_eq!(res.result().name(), "ruarango");
        assert_eq!(res.result().id(), "415");
        assert!(!res.result().is_system());
        assert_eq!(res.result().path(), "none");
        assert!(res.result().sharding().is_none());
        assert!(res.result().replication_factor().is_none());
        assert!(res.result().write_concern().is_none());
    });

    int_test_async!(res; Response<Vec<String>>; database_user_async, conn_ruarango_async, user() => {
        assert_eq!(res.result().len(), 1);
        assert_eq!(res.result()[0], "ruarango");
    });

    int_test_sync!(res; database_user, conn_ruarango, user() => {
        assert_eq!(res.result().len(), 1);
        assert_eq!(res.result()[0], "ruarango");
    });

    int_test_async!(res; Response<Vec<String>>; database_list_async, conn_root_system_async, list() => {
        assert!(res.result().len() > 0);
        assert!(res.result().contains(&"ruarango".to_string()));
    });

    int_test_sync!(res; database_list, conn_root_system, list() => {
        assert!(res.result().len() > 0);
        assert!(res.result().contains(&"ruarango".to_string()));
    });

    lazy_static! {
        static ref DB_NAME: String = rand_name();
        static ref DB_NAME_ASYNC: String = rand_name();
    }

    enum CreateKind {
        Sync,
        Async,
    }

    fn create_config(kind: CreateKind) -> Result<Create> {
        match kind {
            CreateKind::Async => Ok(CreateBuilder::default().name(&*DB_NAME_ASYNC).build()?),
            CreateKind::Sync => Ok(CreateBuilder::default().name(&*DB_NAME).build()?),
        }
    }

    int_test_sync!(res; conn; 201; database_create_drop, conn_root_system, create(&create_config(CreateKind::Sync)?) => {
        assert!(res.result());

        let either = conn.drop(&*DB_NAME).await?;
        let res = process_sync_result(either)?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert!(res.result());
    });

    int_test_async!(res; conn; Response<bool>; database_create_drop_async, conn_root_system_async, create(&create_config(CreateKind::Async)?) => {
        assert!(res.result());

        let res = conn.drop(&*DB_NAME_ASYNC).await?;
        let res: Response<bool> = process_async_result(res, &conn).await?;
        assert!(!res.error());
        assert_eq!(*res.code(), 200);
        assert!(res.result());
    });
}

mod doc {
    use super::common::{conn_ruarango, conn_ruarango_async, process_async_doc_result};
    use anyhow::Result;
    use getset::Getters;
    use ruarango::{
        doc::{
            input::{ConfigBuilder, DeleteConfigBuilder, ReadConfig, ReadConfigBuilder},
            output::DocMeta,
        },
        Document, Either,
        Error::{self, DocumentNotFound, PreconditionFailed},
    };
    use serde_derive::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Getters, Serialize)]
    #[getset(get)]
    struct OutputDoc {
        #[serde(rename = "_key")]
        key: String,
        #[serde(rename = "_id")]
        id: String,
        #[serde(rename = "_rev")]
        rev: String,
        test: String,
    }

    #[derive(Clone, Debug, Deserialize, Getters, Serialize)]
    #[getset(get)]
    struct TestDoc {
        #[serde(rename = "_key", skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        test: String,
    }

    impl Default for TestDoc {
        fn default() -> Self {
            Self {
                key: None,
                test: "test".to_string(),
            }
        }
    }

    #[ignore = "This seems to give back a 304 Not Modified rather than the result"]
    #[tokio::test]
    async fn doc_read_async() -> Result<()> {
        let conn = conn_ruarango_async().await?;
        let res: Either<OutputDoc> = conn
            .read("test_coll", "51210", ReadConfigBuilder::default().build()?)
            .await?;
        assert!(res.is_left());
        let doc: OutputDoc = process_async_doc_result(res, &conn).await?;
        assert_eq!(doc.test(), "tester");
        Ok(())
    }

    #[tokio::test]
    async fn doc_read() -> Result<()> {
        let conn = conn_ruarango().await?;
        let res: Either<OutputDoc> = conn
            .read("test_coll", "51210", ReadConfigBuilder::default().build()?)
            .await?;
        assert!(res.is_right());
        let doc = res.right_safe()?;
        assert_eq!(doc.test(), "tester");
        Ok(())
    }

    enum IfNoneMatchKind {
        Match,
        NoneMatch,
    }

    fn if_none_match_config(kind: IfNoneMatchKind) -> Result<ReadConfig> {
        Ok(match kind {
            IfNoneMatchKind::Match => ReadConfigBuilder::default()
                .if_none_match(r#""_cLEYlhK---""#)
                .build()?,
            IfNoneMatchKind::NoneMatch => ReadConfigBuilder::default()
                .if_none_match(r#""_cJG9Tz1---""#)
                .build()?,
        })
    }

    #[ignore = "upstream call is flaky for some reason"]
    #[tokio::test]
    async fn doc_read_if_none_match_matches_async() -> Result<()> {
        let conn = conn_ruarango_async().await?;
        let res: Either<OutputDoc> = conn
            .read(
                "test_coll",
                "51210",
                if_none_match_config(IfNoneMatchKind::Match)?,
            )
            .await?;
        let none_match: Result<OutputDoc> = process_async_doc_result(res, &conn).await;
        assert!(none_match.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn doc_read_if_none_match_matches() -> Result<()> {
        let conn = conn_ruarango().await?;
        let res: Result<Either<OutputDoc>> = conn
            .read(
                "test_coll",
                "51210",
                if_none_match_config(IfNoneMatchKind::Match)?,
            )
            .await;
        assert!(res.is_err());
        Ok(())
    }

    #[ignore = "upstream call is flaky for some reason"]
    #[tokio::test]
    async fn doc_read_if_none_match_doesnt_match_async() -> Result<()> {
        let conn = conn_ruarango_async().await?;
        let res: Either<OutputDoc> = conn
            .read(
                "test_coll",
                "51210",
                if_none_match_config(IfNoneMatchKind::NoneMatch)?,
            )
            .await?;
        let doc: OutputDoc = process_async_doc_result(res, &conn).await?;
        assert_eq!(doc.test(), "tester");
        Ok(())
    }

    #[tokio::test]
    async fn doc_read_if_none_match_doesnt_match() -> Result<()> {
        let conn = conn_ruarango().await?;
        let either: Either<OutputDoc> = conn
            .read(
                "test_coll",
                "51210",
                if_none_match_config(IfNoneMatchKind::NoneMatch)?,
            )
            .await?;
        assert!(either.is_right());
        let doc = either.right_safe()?;
        assert_eq!(doc.test(), "tester");
        Ok(())
    }

    enum IfMatchKind {
        Match,
        NoneMatch,
    }

    fn if_match_config(kind: IfMatchKind) -> Result<ReadConfig> {
        Ok(match kind {
            IfMatchKind::Match => ReadConfigBuilder::default()
                .if_match(r#""_cLEYlhK---""#)
                .build()?,
            IfMatchKind::NoneMatch => ReadConfigBuilder::default()
                .if_match(r#""_cJG9Tz1---""#)
                .build()?,
        })
    }

    #[tokio::test]
    async fn doc_read_if_match_matches() -> Result<()> {
        let conn = conn_ruarango().await?;
        let either: Either<OutputDoc> = conn
            .read("test_coll", "51210", if_match_config(IfMatchKind::Match)?)
            .await?;
        assert!(either.is_right());
        let doc = either.right_safe()?;
        assert_eq!(doc.test(), "tester");
        Ok(())
    }

    #[tokio::test]
    async fn doc_read_if_match_doesnt_match() -> Result<()> {
        let conn = conn_ruarango().await?;
        let res: Result<Either<OutputDoc>> = conn
            .read(
                "test_coll",
                "51210",
                if_match_config(IfMatchKind::NoneMatch)?,
            )
            .await;
        match res {
            Ok(_) => panic!("This should be an error!"),
            Err(e) => {
                let err = e.downcast_ref::<Error>().expect("unanticipated error");
                match err {
                    PreconditionFailed { err } => {
                        assert!(err.is_some());
                        let pre_cond = err.as_ref().expect("this is bad!");
                        assert!(pre_cond.error());
                        assert_eq!(*pre_cond.code(), 412);
                        assert_eq!(*pre_cond.error_num(), 1200);
                        assert_eq!(pre_cond.error_message(), &Some("conflict".to_string()));
                    }
                    _ => panic!("Incorrect error!"),
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn doc_read_not_found() -> Result<()> {
        let conn = conn_ruarango().await?;
        let res: Result<Either<OutputDoc>> = conn
            .read("test_coll", "yoda", ReadConfigBuilder::default().build()?)
            .await;
        match res {
            Ok(_) => panic!("This should be an error!"),
            Err(e) => {
                let err = e.downcast_ref::<Error>().expect("unanticipated error");
                assert_eq!(err, &DocumentNotFound);
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn create_delete_basic() -> Result<()> {
        let conn = conn_ruarango().await?;

        // Create a document
        let create_config = ConfigBuilder::default().build()?;
        let create_res: Either<DocMeta<(), ()>> = conn
            .create("test_coll", create_config, &TestDoc::default())
            .await?;
        assert!(create_res.is_right());
        let doc_meta = create_res.right_safe()?;
        let key = doc_meta.key();

        // Delete that document
        let delete_config = DeleteConfigBuilder::default().return_old(true).build()?;
        let delete_res: Either<DocMeta<(), TestDoc>> =
            conn.delete("test_coll", &key, delete_config).await?;
        assert!(delete_res.is_right());
        let doc_meta = delete_res.right_safe()?;
        assert!(doc_meta.old_doc().is_some());
        assert_eq!(doc_meta.old_doc().as_ref().unwrap().test(), "test");

        Ok(())
    }

    #[tokio::test]
    async fn create_delete_overwrite_replace() -> Result<()> {
        let conn = conn_ruarango().await?;

        // Create a document
        let create_config = ConfigBuilder::default().build()?;
        let create_res: Either<DocMeta<(), ()>> = conn
            .create("test_coll", create_config, &TestDoc::default())
            .await?;
        assert!(create_res.is_right());
        let doc_meta = create_res.right_safe()?;
        let key = doc_meta.key();

        // Overwrite with replace
        let overwrite = ConfigBuilder::default().overwrite(true).build()?;
        let mut new_doc = TestDoc::default();
        new_doc.key = Some(key.clone());
        new_doc.test = "testing".to_string();
        let overwrite_res: Either<DocMeta<(), ()>> =
            conn.create("test_coll", overwrite, &new_doc).await?;
        assert!(overwrite_res.is_right());
        let doc_meta = overwrite_res.right_safe()?;
        let key = doc_meta.key();

        // Delete that document
        let delete_config = DeleteConfigBuilder::default().return_old(true).build()?;
        let delete_res: Either<DocMeta<(), TestDoc>> =
            conn.delete("test_coll", &key, delete_config).await?;
        assert!(delete_res.is_right());
        let doc_meta = delete_res.right_safe()?;
        assert!(doc_meta.old_doc().is_some());
        assert_eq!(doc_meta.old_doc().as_ref().unwrap().test(), "testing");

        Ok(())
    }
}
