// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document trait implementation

use crate::{
    api_post, api_request,
    doc::{
        input::{Config, OverwriteMode, ReadConfig},
        output::Create,
    },
    error::RuarangoError::Unreachable,
    traits::Document,
    utils::handle_response,
    Connection,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::FutureExt;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{de::DeserializeOwned, Serialize};

#[allow(dead_code)]
const BASE_SUFFIX: &str = "_api/document";

#[async_trait]
impl Document for Connection {
    async fn create<T, U, V>(
        &self,
        collection: &str,
        config: Config,
        document: &T,
    ) -> Result<Create<U, V>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        api_post!(
            self,
            db_url,
            &build_create_url(collection, config),
            document
        )
    }

    async fn read<T>(&self, collection: &str, key: &str, config: ReadConfig) -> Result<T>
    where
        T: DeserializeOwned + Send + Sync,
    {
        if config.has_header() {
            let mut headers = HeaderMap::new();
            if *config.if_match() {
                let _ = headers.append(
                    HeaderName::from_static("If-Match"),
                    HeaderValue::from_static("true"),
                );
                api_request!(
                    self,
                    db_url,
                    &format!("{}/{}/{}", BASE_SUFFIX, collection, key),
                    GET,
                    headers
                )
            } else if *config.if_none_match() {
                let _ = headers.append(
                    HeaderName::from_static("If-None-Match"),
                    HeaderValue::from_static("true"),
                );
                api_request!(
                    self,
                    db_url,
                    &format!("{}/{}/{}", BASE_SUFFIX, collection, key),
                    GET,
                    headers
                )
            } else {
                Err(Unreachable {
                    msg: "One of 'if_match' or 'if_none_match' should be true!".to_string(),
                }
                .into())
            }
        } else {
            api_request!(
                self,
                db_url,
                &format!("{}/{}/{}", BASE_SUFFIX, collection, key),
                GET
            )
        }
    }
}

macro_rules! add_qp {
    ($url:ident, $has_qp:ident, $val:expr;) => {
        let _ = prepend_sep(&mut $url, $has_qp);
        $url += $val;
    };
    ($url:ident, $has_qp:ident, $val:expr) => {
        let _ = prepend_sep(&mut $url, $has_qp);
        $url += $val;
        $has_qp = true;
    };
}

fn build_create_url(name: &str, config: Config) -> String {
    let mut url = format!("{}/{}", BASE_SUFFIX, name);
    let mut has_qp = false;

    // Add waitForSync if necessary
    if config.wait_for_sync().unwrap_or(false) {
        add_qp!(url, has_qp, "waitForSync=true");
    }

    // Setup the output related query parameters
    if config.silent().unwrap_or(false) {
        add_qp!(url, has_qp, "silent=true");
    } else {
        if config.return_new().unwrap_or(false) {
            add_qp!(url, has_qp, "returnNew=true");
        }
        if config.return_old().unwrap_or(false) {
            add_qp!(url, has_qp, "returnOld=true");
        }
    }

    // Setup the overwrite related query parameters
    if let Some(mode) = config.overwrite_mode() {
        add_qp!(url, has_qp, &format!("overwriteMode={}", mode));

        if *mode == OverwriteMode::Update {
            if config.keep_null().unwrap_or(false) {
                add_qp!(url, has_qp, "keepNull=true");
            }

            if config.merge_objects().unwrap_or(false) {
                add_qp!(url, has_qp, "mergeObjects=true";);
            }
        }
    } else if config.overwrite().unwrap_or(false) {
        add_qp!(url, has_qp, "overwrite=true";);
    }

    url
}

fn prepend_sep(url: &mut String, has_qp: bool) -> &mut String {
    if has_qp {
        *url += "&";
    } else {
        *url += "?";
    }

    url
}

#[cfg(test)]
mod test {
    use super::{build_create_url, prepend_sep};
    use crate::{
        doc::{
            input::{ConfigBuilder, OverwriteMode},
            output::{Create, OutputDoc},
        },
        error::RuarangoError,
        traits::Document,
        utils::{
            default_conn, mock_auth,
            mocks::doc::{
                mock_create, mock_create_1, mock_create_2, mock_return_new, mock_return_old,
            },
        },
    };
    use anyhow::Result;
    use getset::Setters;
    use serde_derive::{Deserialize, Serialize};
    use wiremock::MockServer;

    #[test]
    fn has_no_qp() {
        let mut result = String::new();
        assert_eq!("?", prepend_sep(&mut result, false));
    }

    #[test]
    fn has_qp() {
        let mut result = String::new();
        assert_eq!("&", prepend_sep(&mut result, true));
    }

    #[test]
    fn basic_url() -> Result<()> {
        let config = ConfigBuilder::default().build()?;
        let url = build_create_url("test", config);
        assert_eq!("_api/document/test", url);
        Ok(())
    }

    #[test]
    fn wait_for_sync_url() -> Result<()> {
        let config = ConfigBuilder::default().wait_for_sync(true).build()?;
        let url = build_create_url("test", config);
        assert_eq!("_api/document/test?waitForSync=true", url);
        Ok(())
    }

    #[test]
    fn silent_url() -> Result<()> {
        let config = ConfigBuilder::default().silent(true).build()?;
        let url = build_create_url("test", config);
        assert_eq!("_api/document/test?silent=true", url);
        Ok(())
    }

    #[test]
    fn silent_url_forces_no_return() -> Result<()> {
        let config = ConfigBuilder::default()
            .silent(true)
            .return_new(true)
            .return_old(true)
            .build()?;
        let url = build_create_url("test", config);
        assert_eq!("_api/document/test?silent=true", url);
        Ok(())
    }

    #[test]
    fn returns_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .return_new(true)
            .return_old(true)
            .build()?;
        let url = build_create_url("test", config);
        assert_eq!("_api/document/test?returnNew=true&returnOld=true", url);
        Ok(())
    }

    #[test]
    fn overwrite_url() -> Result<()> {
        let config = ConfigBuilder::default().overwrite(true).build()?;
        let url = build_create_url("test", config);
        assert_eq!("_api/document/test?overwrite=true", url);
        Ok(())
    }

    #[test]
    fn overwrite_mode_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .overwrite_mode(OverwriteMode::Update)
            .build()?;
        let url = build_create_url("test", config);
        assert_eq!("_api/document/test?overwriteMode=update", url);
        Ok(())
    }

    #[test]
    fn overwrite_mode_forces_no_overwrite() -> Result<()> {
        let config = ConfigBuilder::default()
            .overwrite(true)
            .overwrite_mode(OverwriteMode::Update)
            .build()?;
        let url = build_create_url("test", config);
        assert_eq!("_api/document/test?overwriteMode=update", url);
        Ok(())
    }

    #[test]
    fn overwrite_mode_update() -> Result<()> {
        let config = ConfigBuilder::default()
            .keep_null(true)
            .merge_objects(true)
            .overwrite_mode(OverwriteMode::Update)
            .build()?;
        let url = build_create_url("test", config);
        assert_eq!(
            "_api/document/test?overwriteMode=update&keepNull=true&mergeObjects=true",
            url
        );
        Ok(())
    }

    #[test]
    fn overwrite_mode_non_update_forces_no_keep_null_merge_objects() -> Result<()> {
        let config = ConfigBuilder::default()
            .keep_null(true)
            .merge_objects(true)
            .overwrite_mode(OverwriteMode::Conflict)
            .build()?;
        let url = build_create_url("test", config);
        assert_eq!("_api/document/test?overwriteMode=conflict", url);
        Ok(())
    }

    #[test]
    fn all_the_opts() -> Result<()> {
        let config = ConfigBuilder::default()
            .wait_for_sync(true)
            .return_new(true)
            .return_old(true)
            .keep_null(true)
            .merge_objects(true)
            .overwrite_mode(OverwriteMode::Update)
            .build()?;
        let url = build_create_url("test", config);
        assert_eq!(
            "_api/document/test?waitForSync=true&returnNew=true&returnOld=true&overwriteMode=update&keepNull=true&mergeObjects=true",
            url
        );
        Ok(())
    }

    #[derive(Deserialize, Serialize, Setters)]
    #[getset(set)]
    struct TestDoc {
        #[serde(rename = "_key", skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        test: String,
    }

    impl Default for TestDoc {
        fn default() -> Self {
            Self {
                key: None,
                test: "test".to_string(),
            }
        }
    }

    #[tokio::test]
    async fn basic_create() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_create(&mock_server).await?;

        let conn = default_conn(mock_server.uri()).await?;
        let config = ConfigBuilder::default().build()?;
        let doc = TestDoc::default();
        let res: Create<(), ()> = conn.create("test_coll", config, &doc).await?;

        assert_eq!(res.key(), "abc");
        assert_eq!(res.id(), "def");
        assert_eq!(res.rev(), "ghi");
        assert!(res.old_rev().is_none());
        assert!(res.new_doc().is_none());
        assert!(res.old_doc().is_none());

        Ok(())
    }

    #[tokio::test]
    async fn overwrite_create() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_create_1(&mock_server).await?;
        mock_create_2(&mock_server).await?;

        let conn = default_conn(mock_server.uri()).await?;
        let config = ConfigBuilder::default().build()?;
        let mut doc = TestDoc::default();
        let _ = doc.set_key(Some("test_key".to_string()));
        let res: Create<(), ()> = conn.create("test_coll", config, &doc).await?;

        assert_eq!(res.key(), "test_key");
        assert!(!res.id().is_empty());
        assert!(!res.rev().is_empty());
        assert!(res.old_rev().is_none());
        assert!(res.new_doc().is_none());
        assert!(res.old_doc().is_none());

        let overwrite_config = ConfigBuilder::default().overwrite(true).build()?;
        let res: Create<(), ()> = conn.create("test_coll", overwrite_config, &doc).await?;

        assert_eq!(res.key(), "test_key");
        assert!(!res.id().is_empty());
        assert!(!res.rev().is_empty());
        assert!(res.old_rev().is_some());
        assert!(res.new_doc().is_none());
        assert!(res.old_doc().is_none());

        Ok(())
    }

    #[tokio::test]
    async fn return_new() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_return_new(&mock_server).await?;

        let conn = default_conn(mock_server.uri()).await?;
        let config = ConfigBuilder::default().return_new(true).build()?;
        let doc = TestDoc::default();
        let res: Create<OutputDoc, ()> = conn.create("test_coll", config, &doc).await?;

        assert_eq!(res.key(), "abc");
        assert_eq!(res.id(), "def");
        assert_eq!(res.rev(), "ghi");
        assert!(res.old_rev().is_none());
        assert!(res.new_doc().is_some());
        assert_eq!(
            res.new_doc()
                .as_ref()
                .ok_or::<RuarangoError>("".into())?
                .key(),
            "abc"
        );
        assert_eq!(
            res.new_doc()
                .as_ref()
                .ok_or::<RuarangoError>("".into())?
                .id(),
            "def"
        );
        assert_eq!(
            res.new_doc()
                .as_ref()
                .ok_or::<RuarangoError>("".into())?
                .rev(),
            "ghi"
        );
        assert_eq!(
            res.new_doc()
                .as_ref()
                .ok_or::<RuarangoError>("".into())?
                .test(),
            "test"
        );
        assert!(res.old_doc().is_none());

        Ok(())
    }

    #[tokio::test]
    async fn return_old() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_create_1(&mock_server).await?;
        mock_return_old(&mock_server).await?;

        let conn = default_conn(mock_server.uri()).await?;
        // let conn = default_conn("http://localhost:8529").await?;
        let config = ConfigBuilder::default().build()?;
        let mut doc = TestDoc::default();
        let _ = doc.set_key(Some("test_key".to_string()));
        let res: Create<(), ()> = conn.create("test_coll", config, &doc).await?;

        assert_eq!(res.key(), "test_key");
        assert!(!res.id().is_empty());
        assert!(!res.rev().is_empty());
        assert!(res.old_rev().is_none());
        assert!(res.new_doc().is_none());
        assert!(res.old_doc().is_none());

        let overwrite_config = ConfigBuilder::default()
            .overwrite(true)
            .return_new(true)
            .return_old(true)
            .build()?;
        let res: Create<OutputDoc, OutputDoc> =
            conn.create("test_coll", overwrite_config, &doc).await?;

        assert_eq!(res.key(), "test_key");
        assert!(!res.id().is_empty());
        assert!(!res.rev().is_empty());
        assert!(res.old_rev().is_some());
        assert!(res.new_doc().is_some());
        assert!(res.old_doc().is_some());

        Ok(())
    }
}
