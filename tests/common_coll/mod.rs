// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Common functionality for Integration Tests

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
