// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Common functionality for Integration Tests

use anyhow::{anyhow, Result};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use ruarango::{ArangoEither, Connection, Job};
use serde::{de::DeserializeOwned, Serialize};
use std::iter;

pub fn rand_name() -> String {
    // Setup a random name so CI testing won't cause collisions
    let mut rng = thread_rng();
    let mut name = String::from("ruarango-");
    let name_ext: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect();
    name.push_str(&name_ext);
    name
}

pub fn process_sync_result<T>(res: ArangoEither<T>) -> Result<T>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    assert!(res.is_right());
    Ok(res.right_safe()?)
}

pub async fn process_async_result<T>(res: ArangoEither<T>, conn: &Connection) -> Result<T>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    assert!(res.is_left());
    let job_info = res.left_safe()?;
    assert_eq!(*job_info.code(), 202);
    let id = job_info
        .id()
        .as_ref()
        .ok_or_else(|| anyhow!("invalid job id"))?;

    let mut status = conn.status(id).await?;
    assert!(status == 200 || status == 204);

    while status != 200 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        status = conn.status(id).await?;
    }

    Ok(conn.fetch(id).await?)
}

pub async fn process_async_doc_result<T>(res: ArangoEither<T>, conn: &Connection) -> Result<T>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    assert!(res.is_left());
    let job_info = res.left_safe()?;
    assert_eq!(*job_info.code(), 202);
    let id = job_info
        .id()
        .as_ref()
        .ok_or_else(|| anyhow!("invalid job id"))?;

    let mut status = conn.status(id).await?;
    assert!(status == 200 || status == 204);

    while status != 200 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        status = conn.status(id).await?;
    }

    Ok(conn.fetch_doc_job(id).await?)
}
