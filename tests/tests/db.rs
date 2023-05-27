use crate::{
    common::{process_async_result, process_sync_result},
    rand_util::rand_name,
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

int_test_async_new!(res; Response<Current>; database_current_async, current() => {
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert_eq!(res.result().name(), "ruarango");
    assert_eq!(res.result().id(), "4310866");
    assert!(!res.result().is_system());
    assert_eq!(res.result().path(), "none");
    assert!(res.result().sharding().is_none());
    assert!(res.result().replication_factor().is_none());
    assert!(res.result().write_concern().is_none());
});

int_test_sync_new!(res; database_current, current() => {
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert_eq!(res.result().name(), "ruarango");
    assert_eq!(res.result().id(), "4310866");
    assert!(!res.result().is_system());
    assert_eq!(res.result().path(), "none");
    assert!(res.result().sharding().is_none());
    assert!(res.result().replication_factor().is_none());
    assert!(res.result().write_concern().is_none());
});

int_test_async_new!(res; Response<Vec<String>>; database_user_async, user() => {
    assert_eq!(res.result().len(), 1);
    assert_eq!(res.result()[0], "ruarango");
});

int_test_sync_new!(res; database_user, user() => {
    assert_eq!(res.result().len(), 1);
    assert_eq!(res.result()[0], "ruarango");
});

int_test_async_new!(res; Response<Vec<String>>; crate::pool::ROOT_ASYNC_POOL; database_list_async, list() => {
    assert!(!res.result().is_empty());
    assert!(res.result().contains(&"ruarango".to_string()));
});

int_test_sync_new!(res; crate::pool::ROOT_POOL; database_list, list() => {
    assert!(!res.result().is_empty());
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

int_test_sync_new!(res; conn; 201; crate::pool::ROOT_POOL; database_create_drop, create(&create_config(CreateKind::Sync)?) => {
    assert!(res.result());

    let either = conn.drop(&DB_NAME).await?;
    let res = process_sync_result(either)?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(res.result());
});

int_test_async_new!(res; conn; Response<bool>; crate::pool::ROOT_ASYNC_POOL; database_create_drop_async, create(&create_config(CreateKind::Async)?) => {
    assert!(res.result());

    let res = conn.drop(&DB_NAME_ASYNC).await?;
    let res: Response<bool> = process_async_result(res, conn).await?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(res.result());
});
