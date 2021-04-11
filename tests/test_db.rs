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

use anyhow::Result;
use common::{
    conn_root_system, conn_root_system_async, conn_ruarango, conn_ruarango_async,
    process_async_result, process_sync_result, rand_name,
};
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
