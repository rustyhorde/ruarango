// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` integration tests
use crate::{
    common::{process_async_result, process_sync_result},
    conn::ConnKind,
    rand_util::rand_name,
};
use anyhow::Result;
use lazy_static::lazy_static;
use ruarango::{
    coll::{
        input::{Config, ConfigBuilder, Props, PropsBuilder},
        output::{
            Checksum, Collection as Coll, Collections, Count, Create, Figures, Load, LoadIndexes,
            ModifyProps, RecalculateCount, Revision,
        },
        CollectionKind, Status,
    },
    common::output::Response,
    Collection,
};

const TEST_COLL: &str = "test_coll";

int_test_async_new!(res; Response<Vec<Collections>>; collection_collections_ruarango_no_system_async, collections(true) => {
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(res.result().len() > 0);
});

int_test_sync_new!(res; collection_collections_ruarango_no_system, collections(true) => {
    assert!(res.result().len() > 0);
});

int_test_async_new!(res; Response<Vec<Collections>>; ConnKind::RootAsync; collection_collections_root_no_system_async, collections(true) => {
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert_eq!(res.result().len(), 0);
});

int_test_sync_new!(res; ConnKind::Root; collection_collections_root_no_system, collections(true) => {
    assert_eq!(res.result().len(), 0);
});

int_test_async_new!(res; Response<Vec<Collections>>; collection_collections_ruarango_async, collections(false) => {
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(res.result().len() > 0);
});

int_test_sync_new!(res; collection_collections_ruarango, collections(false) => {
    assert!(res.result().len() > 0);
});

int_test_async_new!(res; Response<Vec<Collections>>; ConnKind::RootAsync; collection_collections_root_async, collections(false) => {
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(res.result().len() > 0);
});

int_test_sync_new!(res; ConnKind::Root; collection_collections, collections(false) => {
    assert!(res.result().len() > 0);
});

int_test_async_new!(res; Coll; collection_collection_async, collection(TEST_COLL) => {
    assert_eq!(*res.kind(), CollectionKind::Document);
    assert_eq!(*res.status(), Status::Loaded);
    assert!(!res.is_system());
    assert_eq!(res.name(), TEST_COLL);
    assert_eq!(res.id(), "898");
    assert_eq!(res.globally_unique_id(), "h963E57B880A3/898");
});

int_test_sync_new!(res; collection_collection, collection(TEST_COLL) => {
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
        CreateKind::RenameAsync => ConfigBuilder::default().name(&*RENAME_NAME_ASYNC).build()?,
        CreateKind::Truncate => ConfigBuilder::default().name(&*TRUNCATE_NAME).build()?,
        CreateKind::TruncateAsync => ConfigBuilder::default()
            .name(&*TRUNCATE_NAME_ASYNC)
            .build()?,
        CreateKind::Unload => ConfigBuilder::default().name(&*UNLOAD_NAME).build()?,
        CreateKind::UnloadAsync => ConfigBuilder::default().name(&*UNLOAD_NAME_ASYNC).build()?,
    })
}

int_test_async_new!(res; conn; Create; ConnKind::RuarangoAsync; collection_create_drop_async, create(&create_config(CreateKind::CollAsync)?) => {
    assert_eq!(res.name(), &*COLL_NAME_ASYNC);

    let res = conn.drop(&*COLL_NAME_ASYNC, false).await?;
    let res = process_async_result(res, &conn).await?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
});

int_test_sync_new!(res; conn; collection_create_drop, create(&create_config(CreateKind::Coll)?) => {
    assert_eq!(res.name(), &*COLL_NAME);

    let either = conn.drop(&*COLL_NAME, false).await?;
    assert!(either.is_right());
    let res = either.right_safe()?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
});

int_test_async_new!(res; Checksum; collection_checksum_async, checksum(TEST_COLL, false, false) => {
    assert!(!res.checksum().is_empty());

});

int_test_sync_new!(res; collection_checksum, checksum(TEST_COLL, false, false) => {
    assert!(!res.checksum().is_empty());
});

int_test_async_new!(res; Checksum; collection_checksum_1_async, checksum(TEST_COLL, true, false) => {
    assert!(!res.checksum().is_empty());
});

int_test_sync_new!(res; collection_checksum_1, checksum(TEST_COLL, true, false) => {
    assert!(!res.checksum().is_empty());
});

int_test_async_new!(res; Checksum; collection_checksum_2_async, checksum(TEST_COLL, false, true) => {
    assert!(!res.checksum().is_empty());
});

int_test_sync_new!(res; collection_checksum_2, checksum(TEST_COLL, false, true) => {
    assert!(!res.checksum().is_empty());
});

int_test_async_new!(res; Checksum; collection_checksum_3_async, checksum(TEST_COLL, true, true) => {
    assert!(!res.checksum().is_empty());
});

int_test_sync_new!(res; collection_checksum_3, checksum(TEST_COLL, true, true) => {
    assert!(!res.checksum().is_empty());
});

int_test_async_new!(res; Count; collection_count_async, count(TEST_COLL) => {
    assert!(*res.count() >= 1);
});

int_test_sync_new!(res; collection_count, count(TEST_COLL) => {
    assert!(*res.count() >= 1);
});

int_test_async_new!(res; Figures; collection_figures_async, figures(TEST_COLL) => {
    assert!(*res.figures().indexes().count() >= 1);
    assert!(*res.figures().indexes().size() > 0);
    assert!(*res.figures().documents_size() > 0);
    assert!(!res.figures().cache_in_use());
    assert_eq!(*res.figures().cache_size(), 0);
    assert_eq!(*res.figures().cache_usage(), 0);
});

int_test_sync_new!(res; collection_figures, figures(TEST_COLL) => {
    assert!(*res.figures().indexes().count() >= 1);
    assert!(*res.figures().indexes().size() > 0);
    assert!(*res.figures().documents_size() > 0);
    assert!(!res.figures().cache_in_use());
    assert_eq!(*res.figures().cache_size(), 0);
    assert_eq!(*res.figures().cache_usage(), 0);
});

int_test_async_new!(res; Revision; collection_revision_async, revision(TEST_COLL) => {
    assert!(!res.revision().is_empty());
});

int_test_sync_new!(res; collection_revision, revision(TEST_COLL) => {
    assert!(!res.revision().is_empty());
});

int_test_async_new!(res; Load; collection_load_async, load(TEST_COLL, false) => {
    assert!(res.count().is_none());
});

int_test_sync_new!(res; collection_load, load(TEST_COLL, false) => {
    assert!(res.count().is_none());
});

int_test_async_new!(res; Load; collection_load_1_async, load(TEST_COLL, true) => {
    assert!(res.count().is_some());
    assert!(res.count().unwrap() >= 1);
});

int_test_sync_new!(res; collection_load_1, load(TEST_COLL, true) => {
    assert!(res.count().is_some());
    assert!(res.count().unwrap() >= 1);
});

int_test_async_new!(res; LoadIndexes; collection_load_indexes_async, load_indexes(TEST_COLL) => {
    assert!(res.result());
});

int_test_sync_new!(res; collection_load_indexes, load_indexes(TEST_COLL) => {
    assert!(res.result());
});

fn props_config(wait_for_sync: bool) -> Result<Props> {
    Ok(PropsBuilder::default()
        .wait_for_sync(wait_for_sync)
        .build()?)
}

int_test_async_new!(res; conn; ModifyProps; ConnKind::RuarangoAsync; collection_modify_props_async, modify_props(TEST_COLL, props_config(true)?) => {
    assert!(res.wait_for_sync());
    let either = conn.modify_props(TEST_COLL, props_config(false)?).await?;
    let res = process_async_result(either, &conn).await?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(!res.wait_for_sync());
});

int_test_sync_new!(res; conn; collection_modify_props, modify_props(TEST_COLL, props_config(true)?) => {
    assert!(res.wait_for_sync());
    let either = conn.modify_props(TEST_COLL, props_config(false)?).await?;
    assert!(either.is_right());
    let res = either.right_safe()?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(!res.wait_for_sync());
});

int_test_async_new!(res; RecalculateCount; collection_recalculate_count_async, recalculate_count(TEST_COLL) => {
    assert!(res.result());
    assert!(*res.count() >= 1);
});

int_test_sync_new!(res; collection_recalculate_count, recalculate_count(TEST_COLL) => {
    assert!(res.result());
    assert!(*res.count() >= 1);
});

int_test_async_new!(res; conn; Create; ConnKind::RuarangoAsync; collection_rename_async, create(&create_config(CreateKind::RenameAsync)?) => {
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

int_test_sync_new!(res; conn; collection_rename, create(&create_config(CreateKind::Rename)?) => {
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

int_test_async_new!(res; conn; Create; ConnKind::RuarangoAsync; collection_truncate_async, create(&create_config(CreateKind::TruncateAsync)?) => {
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

int_test_sync_new!(res; conn; collection_truncate, create(&create_config(CreateKind::Truncate)?) => {
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

int_test_async_new!(res; conn; Create; ConnKind::RuarangoAsync; collection_unload_async, create(&create_config(CreateKind::UnloadAsync)?) => {
    assert_eq!(res.name(), &*UNLOAD_NAME_ASYNC);

    let either = conn.unload(&*UNLOAD_NAME_ASYNC).await?;
    let _res = process_async_result(either, &conn).await?;

    let either = conn.drop(&*UNLOAD_NAME_ASYNC, false).await?;
    let res = process_async_result(either, &conn).await?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
});

int_test_sync_new!(res; conn; collection_unload, create(&create_config(CreateKind::Unload)?) => {
    assert_eq!(res.name(), &*UNLOAD_NAME);

    let either = conn.unload(&*UNLOAD_NAME).await?;
    let _res = process_sync_result(either)?;

    let either = conn.drop(&*UNLOAD_NAME, false).await?;
    let res = process_sync_result(either)?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
});
