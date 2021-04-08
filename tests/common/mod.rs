// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Common functionality for Integration Tests

use anyhow::Result;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use ruarango::{Connection, ConnectionBuilder};
use std::iter;

macro_rules! int_test {
    () => {};
    ($res:ident; $conn:ident; $code:literal; $name:ident, $conn_ty:ident, $api:ident($($args:expr),*) => $asserts: block) => {
        #[tokio::test]
        async fn $name() -> Result<()> {
            let $conn = $conn_ty().await?;
            let $res = $conn.$api($($args),*).await?;

            assert!(!$res.error());
            assert_eq!(*$res.code(), $code);
            $asserts

            Ok(())
        }
    };
    ($res:ident; $conn:ident; $code:literal; $($tail:tt)*) => {
        int_test!($res; $conn; $code; $($tail)*);
    };
    ($res:ident; $conn:ident; $($tail:tt)*) => {
        int_test!($res; $conn; 200; $($tail)*);
    };
    ($res:ident; $($tail:tt)*) => {
        int_test!($res; conn; 200; $($tail)*);
    };
}

pub(crate) async fn conn_ruarango() -> Result<Connection> {
    ConnectionBuilder::default()
        .url(env!("ARANGODB_URL"))
        .username("ruarango")
        .password(env!("ARANGODB_RUARANGO_PASSWORD"))
        .database("ruarango")
        .build()
        .await
}

pub(crate) async fn conn_root_system() -> Result<Connection> {
    ConnectionBuilder::default()
        .url(env!("ARANGODB_URL"))
        .username("root")
        .password(env!("ARANGODB_ROOT_PASSWORD"))
        .build()
        .await
}

#[allow(dead_code)]
pub(crate) async fn conn_root_ruarango() -> Result<Connection> {
    ConnectionBuilder::default()
        .url(env!("ARANGODB_URL"))
        .username("root")
        .password(env!("ARANGODB_ROOT_PASSWORD"))
        .database("ruarango")
        .build()
        .await
}

pub(crate) fn rand_name() -> String {
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
