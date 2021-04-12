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
use common::{conn_ruarango, conn_ruarango_async};
use ruarango::{
    doc::input::{ReadConfig, ReadConfigBuilder},
    Document, Either,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct OutputDoc {
    #[serde(rename = "_key")]
    key: String,
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_rev")]
    rev: String,
    test: String,
}

fn if_none_match_config() -> Result<ReadConfig> {
    Ok(ReadConfigBuilder::default()
        .if_none_match("\"_cJG9TzO---\"")
        .build()?)
}

#[tokio::test]
async fn doc_read_async() -> Result<()> {
    let conn = conn_ruarango_async().await?;
    let res: libeither::Either<(), Either<OutputDoc>> = conn
        .read("test_coll", "51210", if_none_match_config()?)
        .await?;
    assert!(res.is_left());
    Ok(())
}

#[tokio::test]
async fn doc_read() -> Result<()> {
    let conn = conn_ruarango().await?;
    let res: libeither::Either<(), Either<OutputDoc>> = conn
        .read("test_coll", "51210", if_none_match_config()?)
        .await?;
    assert!(res.is_left());
    Ok(())
}
