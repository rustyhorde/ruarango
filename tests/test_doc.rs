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
use common::{conn_ruarango, conn_ruarango_async, process_async_result, process_async_result_300};
use getset::Getters;
use reqwest::{Error, StatusCode};
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

#[ignore]
#[tokio::test]
async fn doc_read_async() -> Result<()> {
    let conn = conn_ruarango_async().await?;
    let res: Either<libeither::Either<(), OutputDoc>> = conn
        .read("test_coll", "51210", ReadConfigBuilder::default().build()?)
        .await?;
    assert!(res.is_left());
    println!("Got back async result");
    let either: libeither::Either<(), OutputDoc> = process_async_result_300(res, &conn).await?;
    assert!(either.is_right());
    let doc = either.right_safe()?;
    assert_eq!(doc.test(), "tester");
    Ok(())
}

#[tokio::test]
async fn doc_read() -> Result<()> {
    let conn = conn_ruarango().await?;
    let res: Either<libeither::Either<(), OutputDoc>> = conn
        .read("test_coll", "51210", ReadConfigBuilder::default().build()?)
        .await?;
    assert!(res.is_right());
    let none_match = res.right_safe()?;
    assert!(none_match.is_right());
    let doc = none_match.right_safe()?;
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
async fn doc_read_if_none_match_matches() -> Result<()> {
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

#[ignore = "upstream call is flaky for some reason"]
#[tokio::test]
async fn doc_read_if_none_match_doesnt_match_async() -> Result<()> {
    let conn = conn_ruarango_async().await?;
    let res: Either<libeither::Either<(), OutputDoc>> = conn
        .read(
            "test_coll",
            "51210",
            if_none_match_config(IfNoneMatchKind::NoneMatch)?,
        )
        .await?;
    let if_match: libeither::Either<(), OutputDoc> = process_async_result_300(res, &conn).await?;
    assert!(if_match.is_right());
    let doc = if_match.right_safe()?;
    assert_eq!(doc.test(), "tester");
    Ok(())
}

#[tokio::test]
async fn doc_read_if_none_match_doesnt_match() -> Result<()> {
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
    let outer_either: Either<libeither::Either<(), OutputDoc>> = conn
        .read("test_coll", "51210", if_match_config(IfMatchKind::Match)?)
        .await?;
    assert!(outer_either.is_right());
    let none_match = outer_either.right_safe()?;
    assert!(none_match.is_right());
    let doc = none_match.right_safe()?;
    assert_eq!(doc.test(), "tester");
    Ok(())
}

#[tokio::test]
async fn doc_read_if_match_doesnt_match() -> Result<()> {
    let conn = conn_ruarango().await?;
    let res: Result<Either<libeither::Either<(), OutputDoc>>> = conn
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
            assert_eq!(err.status(), Some(StatusCode::PRECONDITION_FAILED));
        }
    }
    Ok(())
}
