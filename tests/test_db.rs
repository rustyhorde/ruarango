// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` database operation integration tests

#[macro_use]
mod common;

use anyhow::{anyhow, Result};
use common::{conn_root_system, conn_ruarango, conn_ruarango_async, rand_name};
use lazy_static::lazy_static;
use ruarango::{
    common::output::Response,
    db::{
        input::{Create, CreateBuilder},
        output::Current,
    },
    Database, Job,
};

#[tokio::test]
async fn database_current_async() -> Result<()> {
    // Request async
    let conn = conn_ruarango_async().await?;
    let res = conn.current().await?;

    // Should get a 202 immediately with a 'job-id'
    assert!(res.is_left());
    let job_info = res.left_safe()?;
    assert_eq!(*job_info.code(), 202);
    let id = job_info.id().as_ref().ok_or_else(|| anyhow!("blah"))?;

    // Check status until we get 200
    let mut status = conn.status(id).await?;
    assert!(status == 200 || status == 204);

    while status != 200 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        status = conn.status(id).await?;
    }

    // Fetch the results
    let res: Response<Current> = conn.fetch(id).await?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert_eq!(res.result().name(), "ruarango");
    assert_eq!(res.result().id(), "415");
    assert!(!res.result().is_system());
    assert_eq!(res.result().path(), "none");
    assert!(res.result().sharding().is_none());
    assert!(res.result().replication_factor().is_none());
    assert!(res.result().write_concern().is_none());

    Ok(())
}

#[tokio::test]
async fn database_current() -> Result<()> {
    let conn = conn_ruarango().await?;
    let res = conn.current().await?;

    assert!(res.is_right());
    let stuff = res.right_safe()?;
    assert!(!stuff.error());
    assert_eq!(*stuff.code(), 200);
    assert_eq!(stuff.result().name(), "ruarango");
    assert_eq!(stuff.result().id(), "415");
    assert!(!stuff.result().is_system());
    assert_eq!(stuff.result().path(), "none");
    assert!(stuff.result().sharding().is_none());
    assert!(stuff.result().replication_factor().is_none());
    assert!(stuff.result().write_concern().is_none());

    Ok(())
}

// int_test!(res; database_user, conn_ruarango, user() => {
//     assert_eq!(res.result().len(), 1);
//     assert_eq!(res.result()[0], "ruarango");
// });

int_test!(res; database_list, conn_root_system, list() => {
    assert!(res.result().len() > 0);
    assert!(res.result().contains(&"ruarango".to_string()));
});

lazy_static! {
    static ref DB_NAME: String = rand_name();
}

fn create_config() -> Result<Create> {
    Ok(CreateBuilder::default().name(&*DB_NAME).build()?)
}

int_test!(res; conn; 201; database_create_drop, conn_root_system, create(&create_config()?) => {
    assert!(res.result());

    let res = conn.drop(&*DB_NAME).await?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(res.result());
});
