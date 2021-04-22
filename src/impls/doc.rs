// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document trait implementation

use crate::{
    doc::{
        input::{
            CreateConfig, CreatesConfig, DeleteConfig, DeletesConfig, ReadConfig, ReadsConfig,
            ReplaceConfig, ReplacesConfig, UpdateConfig, UpdatesConfig,
        },
        BASE_DOC_SUFFIX,
    },
    model::{AddHeaders, BuildUrl},
    traits::Document,
    types::{ArangoResult, ArangoVecResult, DocMetaResult, DocMetaVecResult},
    utils::{doc_resp, doc_vec_resp},
    Connection,
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

const EMPTY_BODY: Option<String> = None;

#[async_trait]
impl Document for Connection {
    async fn create<T, U, V>(&self, config: CreateConfig<T>) -> DocMetaResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        self.post(url, None, config.document(), doc_resp).await
    }

    async fn creates<T, U, V>(&self, config: CreatesConfig<T>) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        self.post(url, None, config.document(), doc_vec_resp).await
    }

    async fn read<T>(&self, config: ReadConfig) -> ArangoResult<T>
    where
        T: DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.get(url, headers, EMPTY_BODY, doc_resp).await
    }

    async fn reads<T, U>(&self, config: ReadsConfig<T>) -> ArangoVecResult<U>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        self.put(url, None, config.documents(), doc_vec_resp).await
    }

    async fn replace<T, U, V>(&self, config: ReplaceConfig<T>) -> DocMetaResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.put(url, headers, config.document(), doc_resp).await
    }

    async fn replaces<T, U, V>(&self, config: ReplacesConfig<T>) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        self.put(url, None, config.documents(), doc_vec_resp).await
    }

    async fn update<T, U, V>(&self, config: UpdateConfig<T>) -> DocMetaResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.patch(url, headers, config.document(), doc_resp).await
    }

    async fn updates<T, U, V>(&self, config: UpdatesConfig<T>) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        self.patch(url, None, config.documents(), doc_vec_resp)
            .await
    }

    async fn delete<U, V>(&self, config: DeleteConfig) -> DocMetaResult<U, V>
    where
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.delete(url, headers, EMPTY_BODY, doc_resp).await
    }

    async fn deletes<T, U, V>(&self, config: DeletesConfig<T>) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_DOC_SUFFIX, self)?;
        self.delete(url, None, config.documents(), doc_vec_resp)
            .await
    }
}

#[cfg(test)]
mod test {
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
