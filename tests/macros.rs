macro_rules! int_test_sync_new {
    () => {};
    ($res:ident; $conn:ident; $code:literal; $conn_kind:expr; $name:ident, $api:ident($($args:expr),*) => $asserts: block) => {
        #[tokio::test]
        async fn $name() -> Result<()> {
            let $conn = $crate::conn::conn($conn_kind).await?;
            let res = $conn.$api($($args),*).await?;
            let $res = $crate::common::process_sync_result(res)?;
            $asserts

            Ok(())
        }
    };
    ($res:ident; $conn:ident; $code:literal; $conn_kind:expr; $($tail:tt)*) => {
        int_test_sync_new!($res; $conn; $code; $conn_kind; $($tail)*);
    };
    ($res:ident; $conn:ident; $code:literal; $($tail:tt)*) => {
        int_test_sync_new!($res; $conn; $code; $crate::conn::ConnKind::Ruarango; $($tail)*);
    };
    ($res:ident; $conn:ident; $($tail:tt)*) => {
        int_test_sync_new!($res; $conn; 200; $($tail)*);
    };
    ($res:ident; $($tail:tt)*) => {
        int_test_sync_new!($res; conn; $($tail)*);
    };
}

macro_rules! int_test_async_new {
    () => {};
    ($res:ident; $conn:ident; $kind:ty; $conn_kind:expr; $name:ident, $api:ident($($args:expr),*) => $asserts: block) => {
        #[tokio::test]
        async fn $name() -> Result<()> {
            let $conn = $crate::conn::conn($conn_kind).await?;
            let res = $conn.$api($($args),*).await?;
            let $res: $kind = $crate::common::process_async_result(res, &$conn).await?;
            $asserts

            Ok(())
        }
    };
    ($res:ident; $conn:ident, $kind:ty; $conn_kind:expr; $($tail:tt)*) => {
        int_test_async_new!($res; $conn; $kind; $conn_kind; $($tail)*);
    };
    ($res:ident; $kind:ty; $conn_kind:expr; $($tail:tt)*) => {
        int_test_async_new!($res; conn; $kind; $conn_kind; $($tail)*);
    };
    ($res:ident; $kind:ty; $($tail:tt)*) => {
        int_test_async_new!($res; conn; $kind; $crate::conn::ConnKind::RuarangoAsync; $($tail)*);
    };
}
