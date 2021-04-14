// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` document operation integration tests

mod common;

use anyhow::Result;
use common::{conn_ruarango, conn_ruarango_async, process_async_result_300};
use getset::Getters;
use ruarango::{
    doc::input::{ReadConfig, ReadConfigBuilder},
    Document, Either,
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

enum IfNoneMatchKind {
    Match,
    NoneMatch,
}

fn if_none_match_config(kind: IfNoneMatchKind) -> Result<ReadConfig> {
    Ok(match kind {
        IfNoneMatchKind::Match => ReadConfigBuilder::default()
            .if_none_match("\"_cJG9TzO---\"")
            .build()?,
        IfNoneMatchKind::NoneMatch => ReadConfigBuilder::default()
            .if_none_match("\"blah\"")
            .build()?,
    })
}

#[tokio::test]
async fn doc_read_async() -> Result<()> {
    let conn = conn_ruarango_async().await?;
    let res: Either<libeither::Either<(), OutputDoc>> = conn
        .read(
            "test_coll",
            "51210",
            if_none_match_config(IfNoneMatchKind::Match)?,
        )
        .await?;
    let none_match: libeither::Either<(), OutputDoc> = process_async_result_300(res, &conn).await?;
    assert!(none_match.is_left());
    Ok(())
}

#[tokio::test]
async fn doc_read() -> Result<()> {
    let conn = conn_ruarango().await?;
    let outer_either: Either<libeither::Either<(), OutputDoc>> = conn
        .read(
            "test_coll",
            "51210",
            if_none_match_config(IfNoneMatchKind::Match)?,
        )
        .await?;
    assert!(outer_either.is_right());
    let none_match = outer_either.right_safe()?;
    assert!(none_match.is_left());
    Ok(())
}

#[tokio::test]
async fn doc_read_if_match() -> Result<()> {
    let conn = conn_ruarango().await?;
    let outer_either: Either<libeither::Either<(), OutputDoc>> = conn
        .read(
            "test_coll",
            "51210",
            if_none_match_config(IfNoneMatchKind::NoneMatch)?,
        )
        .await?;
    assert!(outer_either.is_right());
    let none_match = outer_either.right_safe()?;
    assert!(none_match.is_right());
    let doc = none_match.right_safe()?;
    assert_eq!(doc.test(), "test");
    Ok(())
}
