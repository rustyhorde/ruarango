// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document trait implementation

use crate::{
    api_post_async, api_post_right,
    doc::{
        input::{Config, OverwriteMode, ReadConfig},
        output::DocMeta,
    },
    error::RuarangoError::Unreachable,
    traits::{Document, Either, JobInfo},
    utils::{handle_response, handle_response_300},
    Connection,
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::{Future, FutureExt};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Response, Url,
};
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
    ) -> Result<Either<DocMeta<U, V>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        if *self.is_async() {
            api_post_async!(
                self,
                db_url,
                &build_create_url(collection, config),
                document
            )
        } else {
            api_post_right!(self, db_url, &build_create_url(collection, config), DocMeta<U, V>, document)
        }
    }

    async fn creates<T, U, V>(
        &self,
        _collection: &str,
        _config: Config,
        _documents: &Vec<T>,
    ) -> Result<Either<Vec<DocMeta<U, V>>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        Err(anyhow!("not implemented"))
    }

    async fn read<T>(
        &self,
        collection: &str,
        key: &str,
        config: ReadConfig,
    ) -> Result<Either<libeither::Either<(), T>>>
    where
        T: DeserializeOwned + Send + Sync,
    {
        let suffix = &format!("{}/{}/{}", BASE_SUFFIX, collection, key);
        let current_url = self
            .db_url()
            .join(suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))?;
        let mut headers = None;

        if config.has_header() {
            let mut headers_map = HeaderMap::new();

            if let Some(rev) = config.if_match() {
                let _ = headers_map.append(
                    HeaderName::from_static("if-match"),
                    HeaderValue::from_str(rev)?,
                );
                headers = Some(headers_map);
            } else if let Some(rev) = config.if_none_match() {
                let _ = headers_map.append(
                    HeaderName::from_static("if-none-match"),
                    HeaderValue::from_str(rev)?,
                );
                headers = Some(headers_map);
            } else {
                return Err(Unreachable {
                    msg: "One of 'if_match' or 'if_none_match' should be true!".to_string(),
                }
                .into());
            }
        }

        if *self.is_async() {
            async_req(self.async_client(), current_url, headers).await
        } else {
            sync_req(self.client(), current_url, headers).await
        }
    }

    async fn reads<T>() -> Result<Either<Vec<T>>>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
    {
        Err(anyhow!("not implemented"))
    }

    async fn replace<T, U, V>() -> Result<Either<DocMeta<U, V>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        Err(anyhow!("not implemented"))
    }

    async fn replaces<T, U, V>() -> Result<Either<Vec<DocMeta<U, V>>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        Err(anyhow!("not implemented"))
    }

    async fn update<T, U, V>() -> Result<Either<DocMeta<U, V>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        Err(anyhow!("not implemented"))
    }

    async fn delete<T, U, V>() -> Result<Either<DocMeta<U, V>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        Err(anyhow!("not implemented"))
    }
}

fn req(
    client: &Client,
    url: Url,
    headers: Option<HeaderMap>,
) -> impl Future<Output = std::result::Result<Response, reqwest::Error>> {
    let mut rb = client.get(url);

    if let Some(headers) = headers {
        rb = rb.headers(headers);
    }

    rb.send()
}

async fn async_req<T>(
    client: &Client,
    url: Url,
    headers: Option<HeaderMap>,
) -> Result<Either<libeither::Either<(), T>>>
where
    T: DeserializeOwned + Send + Sync,
{
    let res = req(client, url, headers).await?;

    let status = res.status().as_u16();
    let job_id = res
        .headers()
        .get("x-arango-async-id")
        .map(|x| x.to_str().unwrap_or_default().to_string());

    Ok(libeither::Either::new_left(JobInfo::new(status, job_id)))
}

async fn sync_req<T>(
    client: &Client,
    url: Url,
    headers: Option<HeaderMap>,
) -> Result<Either<libeither::Either<(), T>>>
where
    T: DeserializeOwned + Send + Sync,
{
    let res = req(client, url, headers).then(handle_response_300).await?;
    Ok(libeither::Either::new_right(res))
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
            input::{ConfigBuilder, OverwriteMode, ReadConfigBuilder},
            output::{DocMeta, OutputDoc},
        },
        error::RuarangoError,
        traits::{Document, Either},
        utils::{
            default_conn, mock_auth,
            mocks::doc::{
                mock_create, mock_create_1, mock_create_2, mock_read, mock_read_if_match,
                mock_return_new, mock_return_old,
            },
        },
    };
    use anyhow::Result;
    use getset::{Getters, Setters};
    use serde_derive::{Deserialize, Serialize};
    use wiremock::{
        matchers::{header_exists, method, path},
        Mock, MockServer, ResponseTemplate,
    };

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

    #[derive(Deserialize, Getters, Serialize, Setters)]
    #[getset(get, set)]
    struct TestDoc {
        #[serde(rename = "_key", skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "_rev", skip_serializing_if = "Option::is_none")]
        rev: Option<String>,
        test: String,
    }

    impl Default for TestDoc {
        fn default() -> Self {
            Self {
                key: None,
                id: None,
                rev: None,
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
        let either: Either<DocMeta<(), ()>> = conn.create("test_coll", config, &doc).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
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
        let either: Either<DocMeta<(), ()>> = conn.create("test_coll", config, &doc).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert_eq!(res.key(), "test_key");
        assert!(!res.id().is_empty());
        assert!(!res.rev().is_empty());
        assert!(res.old_rev().is_none());
        assert!(res.new_doc().is_none());
        assert!(res.old_doc().is_none());

        let overwrite_config = ConfigBuilder::default().overwrite(true).build()?;
        let either: Either<DocMeta<(), ()>> =
            conn.create("test_coll", overwrite_config, &doc).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
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
        let either: Either<DocMeta<OutputDoc, ()>> = conn.create("test_coll", config, &doc).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
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
        let either: Either<DocMeta<(), ()>> = conn.create("test_coll", config, &doc).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
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
        let either: Either<DocMeta<OutputDoc, OutputDoc>> =
            conn.create("test_coll", overwrite_config, &doc).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert_eq!(res.key(), "test_key");
        assert!(!res.id().is_empty());
        assert!(!res.rev().is_empty());
        assert!(res.old_rev().is_some());
        assert!(res.new_doc().is_some());
        assert!(res.old_doc().is_some());

        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_read(&mock_server).await?;

        let conn = default_conn(mock_server.uri()).await?;
        let config = ReadConfigBuilder::default().build()?;
        let outer_either: Either<libeither::Either<(), OutputDoc>> =
            conn.read("test_coll", "test_doc", config).await?;
        assert!(outer_either.is_right());
        let either = outer_either.right_safe()?;
        assert!(either.is_right());
        let doc = either.right_safe()?;
        assert_eq!(doc.key(), "abc");
        assert!(!doc.id().is_empty());
        assert!(!doc.rev().is_empty());
        assert_eq!(doc.test(), "test");

        Ok(())
    }

    async fn mock_read_if_none_match(mock_server: &MockServer) -> Result<()> {
        let mock_response = ResponseTemplate::new(304);

        let mock_builder = Mock::given(method("GET"))
            .and(path("_db/keti/_api/document/test_coll/test_doc"))
            .and(header_exists("if-none-match"));

        mock_builder
            .respond_with(mock_response)
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;
        Ok(())
    }

    #[tokio::test]
    async fn read_if_none_match() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_read_if_none_match(&mock_server).await?;

        let conn = default_conn(mock_server.uri()).await?;
        // let conn = default_conn("http://localhost:8529").await?;
        let config = ReadConfigBuilder::default()
            .if_none_match("_cIw-YT6---")
            .build()?;
        let outer_either: Either<libeither::Either<(), OutputDoc>> =
            conn.read("test_coll", "test_doc", config).await?;
        assert!(outer_either.is_right());
        let either = outer_either.right_safe()?;
        assert!(either.is_left());

        Ok(())
    }

    #[tokio::test]
    async fn read_if_match() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_read_if_match(&mock_server).await?;

        let conn = default_conn(mock_server.uri()).await?;
        let config = ReadConfigBuilder::default()
            .if_match("_cIw-YT6---")
            .build()?;
        let outer_either: Either<libeither::Either<(), OutputDoc>> =
            conn.read("test_coll", "test_doc", config).await?;
        assert!(outer_either.is_right());
        let either = outer_either.right_safe()?;
        assert!(either.is_right());
        let doc = either.right_safe()?;
        assert_eq!(doc.key(), "abc");
        assert!(!doc.id().is_empty());
        assert!(!doc.rev().is_empty());
        assert_eq!(doc.test(), "test");

        Ok(())
    }

    async fn mock_read_if_match_fail(mock_server: &MockServer) -> Result<()> {
        let mock_response = ResponseTemplate::new(412);

        let mock_builder = Mock::given(method("GET"))
            .and(path("_db/keti/_api/document/test_coll/test_doc"))
            .and(header_exists("if-match"));

        mock_builder
            .respond_with(mock_response)
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;
        Ok(())
    }

    #[tokio::test]
    async fn read_if_match_fail() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_read_if_match_fail(&mock_server).await?;

        let conn = default_conn(mock_server.uri()).await?;
        // let conn = default_conn("http://localhost:8529").await?;
        let config = ReadConfigBuilder::default()
            .if_match("this_wont_match")
            .build()?;
        let outer_either: Result<Either<libeither::Either<(), TestDoc>>> =
            conn.read("test_coll", "test_doc", config).await;
        assert!(outer_either.is_err());

        Ok(())
    }
}
