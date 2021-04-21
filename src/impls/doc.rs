// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document trait implementation

use crate::{
    conn::HttpVerb,
    doc::input::{
        CreateConfig, CreatesConfig, DeleteConfig, ReadConfig, ReadsConfig, ReplaceConfig,
        UpdateConfig, UpdatesConfig,
    },
    model::{AddHeaders, BuildUrl},
    traits::Document,
    types::{ArangoResult, ArangoVecResult, DocMetaResult, DocMetaVecResult},
    utils::{doc_resp, doc_vec_resp},
    Connection,
};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

const BASE_SUFFIX: &str = "_api/document";
const EMPTY_BODY: Option<String> = None;

#[async_trait]
impl Document for Connection {
    async fn create<T, U, V>(&self, config: CreateConfig<T>) -> DocMetaResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_SUFFIX, self)?;
        self.post(url, None, config.document(), doc_resp).await
    }

    async fn creates<T, U, V>(&self, config: CreatesConfig<T>) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_SUFFIX, self)?;
        self.post(url, None, config.document(), doc_vec_resp).await
    }

    async fn read<T>(&self, config: ReadConfig) -> ArangoResult<T>
    where
        T: DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.get(url, headers, EMPTY_BODY, doc_resp).await
    }

    async fn reads<T, U>(&self, config: ReadsConfig<T>) -> ArangoVecResult<U>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_SUFFIX, self)?;
        self.put(url, None, config.documents(), doc_vec_resp).await
    }

    async fn replace<T, U, V>(&self, config: ReplaceConfig<T>) -> DocMetaResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.put(url, headers, config.document(), doc_resp).await
    }

    async fn replaces<T, U, V>() -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        Err(anyhow!("not implemented"))
    }

    async fn update<T, U, V>(&self, config: UpdateConfig<T>) -> DocMetaResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.patch(url, headers, config.document(), doc_resp).await
    }

    async fn updates<T, U, V>(&self, config: UpdatesConfig<T>) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_SUFFIX, self)?;
        self.patch(url, None, config.documents(), doc_vec_resp)
            .await
    }

    async fn delete<U, V>(&self, config: DeleteConfig) -> DocMetaResult<U, V>
    where
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.delete(url, headers, EMPTY_BODY, doc_resp).await
    }

    async fn deletes<T, U, V>(
        &self,
        collection: &str,
        config: DeleteConfig,
        documents: &[T],
    ) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let suffix = &build_deletes_url(collection, &config);
        let url = self
            .db_url()
            .join(suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))?;

        self.req(&HttpVerb::Delete, url, None, Some(documents), doc_vec_resp)
            .await
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

fn build_deletes_url(collection: &str, config: &DeleteConfig) -> String {
    let mut url = format!("{}/{}", BASE_SUFFIX, collection);
    let mut has_qp = false;

    // Add waitForSync if necessary
    if config.wait_for_sync().unwrap_or(false) {
        add_qp!(url, has_qp, "waitForSync=true");
    }

    // Setup the output related query parameters
    if config.return_old().unwrap_or(false) {
        add_qp!(url, has_qp, "returnOld=true");
    }

    // Setup ignore revs
    if config.ignore_revs().unwrap_or(false) {
        add_qp!(url, has_qp, "ignoreRevs=true";);
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
    use super::prepend_sep;
    use crate::{
        doc::{
            input::{CreateConfigBuilder, ReadConfigBuilder},
            output::{DocMeta, OutputDoc},
        },
        error::RuarangoErr,
        traits::Document,
        types::{ArangoEither, ArangoResult},
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
    use libeither::Either;
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

    // #[tokio::test]
    // async fn basic_create_url() -> Result<()> {
    //     let config = CreateConfigBuilder::default().build()?;
    //     let conn = default_conn("http://localhost:8529").await?;
    //     let url = build_create_url(&conn, "test", config)?;
    //     assert_eq!(
    //         "http://localhost:8529/_db/keti/_api/document/test",
    //         url.into_string()
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn basic_delete_url() -> Result<()> {
    //     let config = DeleteConfigBuilder::default().build()?;
    //     let url = build_delete_url("test_coll", "test_key", &config);
    //     assert_eq!("_api/document/test_coll/test_key", url);
    //     Ok(())
    // }

    // #[test]
    // fn create_wait_for_sync_url() -> Result<()> {
    //     let config = CreateConfigBuilder::default().wait_for_sync(true).build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!("_api/document/test?waitForSync=true", url);
    //     Ok(())
    // }

    // #[test]
    // fn delete_wait_for_sync_url() -> Result<()> {
    //     let config = DeleteConfigBuilder::default().wait_for_sync(true).build()?;
    //     let url = build_delete_url("test_coll", "test_key", &config);
    //     assert_eq!("_api/document/test_coll/test_key?waitForSync=true", url);
    //     Ok(())
    // }

    // #[test]
    // fn create_silent_url() -> Result<()> {
    //     let config = CreateConfigBuilder::default().silent(true).build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!("_api/document/test?silent=true", url);
    //     Ok(())
    // }

    // #[test]
    // fn delete_silent_url() -> Result<()> {
    //     let config = DeleteConfigBuilder::default().silent(true).build()?;
    //     let url = build_delete_url("test_coll", "test_key", &config);
    //     assert_eq!("_api/document/test_coll/test_key?silent=true", url);
    //     Ok(())
    // }

    // #[test]
    // fn create_silent_url_forces_no_return() -> Result<()> {
    //     let config = CreateConfigBuilder::default()
    //         .silent(true)
    //         .return_new(true)
    //         .return_old(true)
    //         .build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!("_api/document/test?silent=true", url);
    //     Ok(())
    // }

    // #[test]
    // fn delete_silent_url_forces_no_return() -> Result<()> {
    //     let config = DeleteConfigBuilder::default()
    //         .silent(true)
    //         .return_old(true)
    //         .build()?;
    //     let url = build_delete_url("test_coll", "test_key", &config);
    //     assert_eq!("_api/document/test_coll/test_key?silent=true", url);
    //     Ok(())
    // }

    // #[test]
    // fn create_returns_url() -> Result<()> {
    //     let config = CreateConfigBuilder::default()
    //         .return_new(true)
    //         .return_old(true)
    //         .build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!("_api/document/test?returnNew=true&returnOld=true", url);
    //     Ok(())
    // }

    // #[test]
    // fn delete_returns_url() -> Result<()> {
    //     let config = DeleteConfigBuilder::default().return_old(true).build()?;
    //     let url = build_delete_url("test_coll", "test_key", &config);
    //     assert_eq!("_api/document/test_coll/test_key?returnOld=true", url);
    //     Ok(())
    // }

    // #[test]
    // fn overwrite_url() -> Result<()> {
    //     let config = CreateConfigBuilder::default().overwrite(true).build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!("_api/document/test?overwrite=true", url);
    //     Ok(())
    // }

    // #[test]
    // fn overwrite_mode_url() -> Result<()> {
    //     let config = CreateConfigBuilder::default()
    //         .overwrite_mode(OverwriteMode::Update)
    //         .build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!("_api/document/test?overwriteMode=update", url);
    //     Ok(())
    // }

    // #[test]
    // fn overwrite_mode_forces_no_overwrite() -> Result<()> {
    //     let config = CreateConfigBuilder::default()
    //         .overwrite(true)
    //         .overwrite_mode(OverwriteMode::Update)
    //         .build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!("_api/document/test?overwriteMode=update", url);
    //     Ok(())
    // }

    // #[test]
    // fn overwrite_mode_update() -> Result<()> {
    //     let config = CreateConfigBuilder::default()
    //         .keep_null(true)
    //         .merge_objects(true)
    //         .overwrite_mode(OverwriteMode::Update)
    //         .build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!(
    //         "_api/document/test?overwriteMode=update&keepNull=true&mergeObjects=true",
    //         url
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn overwrite_mode_non_update_forces_no_keep_null_merge_objects() -> Result<()> {
    //     let config = CreateConfigBuilder::default()
    //         .keep_null(true)
    //         .merge_objects(true)
    //         .overwrite_mode(OverwriteMode::Conflict)
    //         .build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!("_api/document/test?overwriteMode=conflict", url);
    //     Ok(())
    // }

    // #[test]
    // fn create_all_the_opts() -> Result<()> {
    //     let config = CreateConfigBuilder::default()
    //         .wait_for_sync(true)
    //         .return_new(true)
    //         .return_old(true)
    //         .keep_null(true)
    //         .merge_objects(true)
    //         .overwrite_mode(OverwriteMode::Update)
    //         .build()?;
    //     let url = build_create_url("test", config);
    //     assert_eq!(
    //         "_api/document/test?waitForSync=true&returnNew=true&returnOld=true&overwriteMode=update&keepNull=true&mergeObjects=true",
    //         url
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn delete_all_the_opts() -> Result<()> {
    //     let config = DeleteConfigBuilder::default()
    //         .wait_for_sync(true)
    //         .return_old(true)
    //         .build()?;
    //     let url = build_delete_url("test_coll", "test_key", &config);
    //     assert_eq!(
    //         "_api/document/test_coll/test_key?waitForSync=true&returnOld=true",
    //         url
    //     );
    //     Ok(())
    // }

    #[derive(Clone, Deserialize, Getters, Serialize, Setters)]
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
        let config = CreateConfigBuilder::default()
            .collection("test_coll")
            .document(TestDoc::default())
            .build()?;
        let either: ArangoEither<DocMeta<(), ()>> = conn.create(config).await?;
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
        let mut doc = TestDoc::default();
        let _ = doc.set_key(Some("test_key".to_string()));
        let config = CreateConfigBuilder::default()
            .collection("test_coll")
            .document(doc.clone())
            .build()?;
        let either: ArangoEither<DocMeta<(), ()>> = conn.create(config).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert_eq!(res.key(), "test_key");
        assert!(!res.id().is_empty());
        assert!(!res.rev().is_empty());
        assert!(res.old_rev().is_none());
        assert!(res.new_doc().is_none());
        assert!(res.old_doc().is_none());

        let overwrite_config = CreateConfigBuilder::default()
            .collection("test_coll")
            .document(doc)
            .overwrite(true)
            .build()?;
        let either: ArangoEither<DocMeta<(), ()>> = conn.create(overwrite_config).await?;
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
        let doc = TestDoc::default();
        let config = CreateConfigBuilder::default()
            .collection("test_coll")
            .document(doc)
            .return_new(true)
            .build()?;
        let either: ArangoEither<DocMeta<OutputDoc, ()>> = conn.create(config).await?;
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
                .ok_or::<RuarangoErr>("".into())?
                .key(),
            "abc"
        );
        assert_eq!(
            res.new_doc().as_ref().ok_or::<RuarangoErr>("".into())?.id(),
            "def"
        );
        assert_eq!(
            res.new_doc()
                .as_ref()
                .ok_or::<RuarangoErr>("".into())?
                .rev(),
            "ghi"
        );
        assert_eq!(
            res.new_doc()
                .as_ref()
                .ok_or::<RuarangoErr>("".into())?
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
        let mut doc = TestDoc::default();
        let _ = doc.set_key(Some("test_key".to_string()));
        let config = CreateConfigBuilder::default()
            .collection("test_coll")
            .document(doc.clone())
            .build()?;
        let either: ArangoEither<DocMeta<(), ()>> = conn.create(config).await?;
        assert!(either.is_right());
        let res = either.right_safe()?;
        assert_eq!(res.key(), "test_key");
        assert!(!res.id().is_empty());
        assert!(!res.rev().is_empty());
        assert!(res.old_rev().is_none());
        assert!(res.new_doc().is_none());
        assert!(res.old_doc().is_none());

        let overwrite_config = CreateConfigBuilder::default()
            .collection("test_coll")
            .document(doc)
            .overwrite(true)
            .return_new(true)
            .return_old(true)
            .build()?;
        let either: ArangoEither<DocMeta<OutputDoc, OutputDoc>> =
            conn.create(overwrite_config).await?;
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
        let config = ReadConfigBuilder::default()
            .collection("test_coll")
            .key("test_doc")
            .build()?;
        let outer_either: ArangoEither<OutputDoc> = conn.read(config).await?;
        assert!(outer_either.is_right());
        let doc = outer_either.right_safe()?;
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
        let config = ReadConfigBuilder::default()
            .collection("test_coll")
            .key("test_doc")
            .if_none_match("_cIw-YT6---")
            .build()?;
        let res: ArangoResult<OutputDoc> = conn.read(config).await;
        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn read_if_match() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_read_if_match(&mock_server).await?;

        let conn = default_conn(mock_server.uri()).await?;
        let config = ReadConfigBuilder::default()
            .collection("test_coll")
            .key("test_doc")
            .if_match("_cIw-YT6---")
            .build()?;
        let outer_either: ArangoEither<OutputDoc> = conn.read(config).await?;
        assert!(outer_either.is_right());
        let doc = outer_either.right_safe()?;
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
        let config = ReadConfigBuilder::default()
            .collection("test_coll")
            .key("test_doc")
            .if_match("this_wont_match")
            .build()?;
        let outer_either: ArangoResult<Either<(), TestDoc>> = conn.read(config).await;
        assert!(outer_either.is_err());

        Ok(())
    }
}
