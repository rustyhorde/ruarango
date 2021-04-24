mod common;
mod conn;
mod model;

pub use common::process_async_result as _;
pub use common::process_sync_result as _;
pub use common::rand_name as _;
pub use conn::ConnKind::Root as _;

use anyhow::Result;
use common::process_async_doc_result;
use conn::{conn, ConnKind};
use getset::Getters;
use model::{unwrap_doc, OutputDoc, TestDoc};
use ruarango::{
    doc::{
        input::{
            CreateConfigBuilder, CreatesConfigBuilder, DeleteConfigBuilder, DeletesConfigBuilder,
            ReadConfig, ReadConfigBuilder, ReadsConfigBuilder, ReplaceConfigBuilder,
            UpdateConfigBuilder, UpdatesConfigBuilder,
        },
        output::DocMeta,
    },
    ArangoEither, ArangoResult, ArangoVec, Document,
    Error::{self, DocumentNotFound, PreconditionFailed},
};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
struct SearchDoc {
    #[serde(rename = "_key")]
    key: String,
}

#[ignore = "This seems to give back a 304 Not Modified rather than the result"]
#[tokio::test]
async fn doc_read_async() -> Result<()> {
    let conn = conn(ConnKind::RuarangoAsync).await?;
    let config = ReadConfigBuilder::default()
        .collection("test_coll")
        .key("51210")
        .build()?;
    let res: ArangoEither<OutputDoc> = conn.read(config).await?;
    assert!(res.is_left());
    let doc: OutputDoc = process_async_doc_result(res, &conn).await?;
    assert_eq!(doc.test(), "tester");
    Ok(())
}

#[tokio::test]
async fn doc_read() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;
    let config = ReadConfigBuilder::default()
        .collection("test_coll")
        .key("51210")
        .build()?;
    let res: ArangoEither<OutputDoc> = conn.read(config).await?;
    assert!(res.is_right());
    let doc = res.right_safe()?;
    assert_eq!(doc.test(), "tester");
    Ok(())
}

#[tokio::test]
async fn doc_reads() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;
    let mut search_docs = vec![];
    search_docs.push(SearchDoc {
        key: "51210".to_string(),
    });
    search_docs.push(SearchDoc {
        key: "abcd".to_string(),
    });
    let config = ReadsConfigBuilder::default()
        .collection("test_coll")
        .documents(search_docs)
        .build()?;
    let res: ArangoEither<ArangoVec<OutputDoc>> = conn.reads(config).await?;
    assert!(res.is_right());
    let docs = res.right_safe()?;
    assert_eq!(docs.len(), 2);
    let output_doc = docs.get(0).unwrap().clone();
    assert!(output_doc.is_right());
    let doc = output_doc.right_safe()?;
    assert_eq!(doc.key(), "51210");
    assert_eq!(doc.test(), "tester");
    let err_doc = docs.get(1).unwrap().clone();
    assert!(err_doc.is_left());
    let err = err_doc.left_safe()?;
    assert!(err.error());
    assert_eq!(*err.error_num(), 1202);
    Ok(())
}

enum IfNoneMatchKind {
    Match,
    NoneMatch,
}

fn if_none_match_config(kind: IfNoneMatchKind) -> Result<ReadConfig> {
    Ok(match kind {
        IfNoneMatchKind::Match => ReadConfigBuilder::default()
            .collection("test_coll")
            .key("51210")
            .if_none_match(r#""_cM7mafK---""#)
            .build()?,
        IfNoneMatchKind::NoneMatch => ReadConfigBuilder::default()
            .collection("test_coll")
            .key("51210")
            .if_none_match(r#""_cJG9Tz1---""#)
            .build()?,
    })
}

#[ignore = "upstream call is flaky for some reason"]
#[tokio::test]
async fn doc_read_if_none_match_matches_async() -> Result<()> {
    let conn = conn(ConnKind::RuarangoAsync).await?;
    let res: ArangoEither<OutputDoc> = conn
        .read(if_none_match_config(IfNoneMatchKind::Match)?)
        .await?;
    let none_match: Result<OutputDoc> = process_async_doc_result(res, &conn).await;
    assert!(none_match.is_err());
    Ok(())
}

#[tokio::test]
async fn doc_read_if_none_match_matches() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;
    let res: ArangoResult<OutputDoc> = conn
        .read(if_none_match_config(IfNoneMatchKind::Match)?)
        .await;
    assert!(res.is_err());
    Ok(())
}

#[ignore = "upstream call is flaky for some reason"]
#[tokio::test]
async fn doc_read_if_none_match_doesnt_match_async() -> Result<()> {
    let conn = conn(ConnKind::RuarangoAsync).await?;
    let res: ArangoEither<OutputDoc> = conn
        .read(if_none_match_config(IfNoneMatchKind::NoneMatch)?)
        .await?;
    let doc: OutputDoc = process_async_doc_result(res, &conn).await?;
    assert_eq!(doc.test(), "tester");
    Ok(())
}

#[tokio::test]
async fn doc_read_if_none_match_doesnt_match() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;
    let either: ArangoEither<OutputDoc> = conn
        .read(if_none_match_config(IfNoneMatchKind::NoneMatch)?)
        .await?;
    assert!(either.is_right());
    let doc = either.right_safe()?;
    assert_eq!(doc.test(), "tester");
    Ok(())
}

enum IfMatchKind {
    Match,
    NoneMatch,
}

fn if_match_config(kind: IfMatchKind) -> Result<ReadConfig> {
    Ok(match kind {
        IfMatchKind::Match => ReadConfigBuilder::default()
            .collection("test_coll")
            .key("51210")
            .if_match(r#""_cM7mafK---""#)
            .build()?,
        IfMatchKind::NoneMatch => ReadConfigBuilder::default()
            .collection("test_coll")
            .key("51210")
            .if_match(r#""_cJG9Tz1---""#)
            .build()?,
    })
}

#[tokio::test]
async fn doc_read_if_match_matches() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;
    let either: ArangoEither<OutputDoc> = conn.read(if_match_config(IfMatchKind::Match)?).await?;
    assert!(either.is_right());
    let doc = either.right_safe()?;
    assert_eq!(doc.test(), "tester");
    Ok(())
}

#[tokio::test]
async fn doc_read_if_match_doesnt_match() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;
    let res: ArangoResult<OutputDoc> = conn.read(if_match_config(IfMatchKind::NoneMatch)?).await;
    match res {
        Ok(_) => panic!("This should be an error!"),
        Err(e) => {
            let err = e.downcast_ref::<Error>().expect("unanticipated error");
            match err {
                PreconditionFailed { err } => {
                    assert!(err.is_some());
                    let pre_cond = err.as_ref().expect("this is bad!");
                    assert!(pre_cond.error());
                    assert_eq!(*pre_cond.code(), 412);
                    assert_eq!(*pre_cond.error_num(), 1200);
                    assert_eq!(pre_cond.error_message(), &Some("conflict".to_string()));
                }
                _ => panic!("Incorrect error!"),
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn doc_read_not_found() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;
    let res: ArangoResult<OutputDoc> = conn
        .read(
            ReadConfigBuilder::default()
                .collection("test_coll")
                .key("yoda")
                .build()?,
        )
        .await;
    match res {
        Ok(_) => panic!("This should be an error!"),
        Err(e) => {
            let err = e.downcast_ref::<Error>().expect("unanticipated error");
            assert_eq!(err, &DocumentNotFound);
        }
    }
    Ok(())
}

#[tokio::test]
async fn create_delete_basic() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;

    // Create a document
    let create_config = CreateConfigBuilder::default()
        .collection("test_coll")
        .document(TestDoc::default())
        .build()?;
    let create_res: ArangoEither<DocMeta<(), ()>> = conn.create(create_config).await?;
    assert!(create_res.is_right());
    let doc_meta = create_res.right_safe()?;
    let key = doc_meta.key();

    // Delete that document
    let delete_config = DeleteConfigBuilder::default()
        .collection("test_coll")
        .key(key)
        .return_old(true)
        .build()?;
    let delete_res: ArangoEither<DocMeta<(), TestDoc>> = conn.delete(delete_config).await?;
    assert!(delete_res.is_right());
    let doc_meta = delete_res.right_safe()?;
    let doc_opt = doc_meta.old_doc();
    assert!(doc_opt.is_some());
    assert_eq!(unwrap_doc(doc_opt)?.test(), "test");

    Ok(())
}

#[tokio::test]
async fn creates_deletes_basic() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;
    let docs = vec![TestDoc::default(), TestDoc::default(), TestDoc::default()];

    // Create some documents
    let create_config = CreatesConfigBuilder::default()
        .collection("test_coll")
        .document(docs.clone())
        .build()?;
    let create_res: ArangoEither<ArangoVec<DocMeta<(), ()>>> = conn.creates(create_config).await?;
    assert!(create_res.is_right());
    let doc_meta_vec = create_res.right_safe()?;
    assert_eq!(doc_meta_vec.len(), docs.len());

    let mut keys = vec![];
    for doc_meta_either in doc_meta_vec {
        assert!(doc_meta_either.is_right());
        let doc_meta = doc_meta_either.right_safe()?;
        keys.push(doc_meta.key().clone());
    }

    // Delete the documents
    let delete_config = DeletesConfigBuilder::default()
        .collection("test_coll")
        .documents(keys)
        .return_old(true)
        .build()?;
    let delete_res: ArangoEither<ArangoVec<DocMeta<(), TestDoc>>> =
        conn.deletes(delete_config).await?;
    assert!(delete_res.is_right());
    let doc_meta_vec = delete_res.right_safe()?;
    assert_eq!(doc_meta_vec.len(), docs.len());

    for doc_meta_either in doc_meta_vec {
        assert!(doc_meta_either.is_right());
        let doc_meta = doc_meta_either.right_safe()?;
        let doc_opt = doc_meta.old_doc();
        assert!(doc_opt.is_some());
        assert_eq!(unwrap_doc(doc_opt)?.test(), "test");
    }

    Ok(())
}

#[tokio::test]
async fn create_overwrite_replace_delete() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;

    // Create a document
    let create_config = CreateConfigBuilder::default()
        .collection("test_coll")
        .document(TestDoc::default())
        .build()?;
    let create_res: ArangoEither<DocMeta<(), ()>> = conn.create(create_config).await?;
    assert!(create_res.is_right());
    let doc_meta = create_res.right_safe()?;
    let key = doc_meta.key();

    // Overwrite with replace
    let mut new_doc = TestDoc::default();
    *new_doc.key_mut() = Some(key.clone());
    *new_doc.test_mut() = "testing".to_string();
    let overwrite = CreateConfigBuilder::default()
        .collection("test_coll")
        .document(new_doc)
        .overwrite(true)
        .build()?;
    let overwrite_res: ArangoEither<DocMeta<(), ()>> = conn.create(overwrite).await?;
    assert!(overwrite_res.is_right());
    let doc_meta = overwrite_res.right_safe()?;
    let key = doc_meta.key();

    // Delete that document
    let delete_config = DeleteConfigBuilder::default()
        .collection("test_coll")
        .key(key)
        .return_old(true)
        .build()?;
    let delete_res: ArangoEither<DocMeta<(), TestDoc>> = conn.delete(delete_config).await?;
    assert!(delete_res.is_right());
    let doc_meta = delete_res.right_safe()?;
    let doc_opt = doc_meta.old_doc();
    assert!(doc_opt.is_some());
    assert_eq!(unwrap_doc(doc_opt)?.test(), "testing");

    Ok(())
}

#[tokio::test]
async fn create_replace_delete() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;

    // Create a document
    let create_config = CreateConfigBuilder::default()
        .collection("test_coll")
        .document(TestDoc::default())
        .build()?;
    let create_res: ArangoEither<DocMeta<(), ()>> = conn.create(create_config).await?;
    assert!(create_res.is_right());
    let doc_meta = create_res.right_safe()?;
    let key = doc_meta.key();

    // Replace
    let mut new_doc = TestDoc::default();
    *new_doc.test_mut() = "testing".to_string();
    let replace = ReplaceConfigBuilder::default()
        .collection("test_coll")
        .key(key)
        .document(new_doc)
        .return_new(true)
        .build()?;
    let replace_res: ArangoEither<DocMeta<TestDoc, ()>> = conn.replace(replace).await?;
    assert!(replace_res.is_right());
    let doc_meta = replace_res.right_safe()?;
    let key = doc_meta.key();
    let doc_opt = doc_meta.new_doc();
    assert!(doc_opt.is_some());
    assert_eq!(unwrap_doc(doc_opt)?.test(), "testing");

    // Delete that document
    let delete_config = DeleteConfigBuilder::default()
        .collection("test_coll")
        .key(key)
        .return_old(true)
        .build()?;
    let delete_res: ArangoEither<DocMeta<(), TestDoc>> = conn.delete(delete_config).await?;
    assert!(delete_res.is_right());
    let doc_meta = delete_res.right_safe()?;
    let doc_opt = doc_meta.old_doc();
    assert!(doc_opt.is_some());
    assert_eq!(unwrap_doc(doc_opt)?.test(), "testing");

    Ok(())
}

#[tokio::test]
async fn create_update_delete() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;

    // Create a document
    let create_config = CreateConfigBuilder::default()
        .collection("test_coll")
        .document(TestDoc::default())
        .build()?;
    let create_res: ArangoEither<DocMeta<(), ()>> = conn.create(create_config).await?;
    assert!(create_res.is_right());
    let doc_meta = create_res.right_safe()?;
    let key = doc_meta.key();

    // Update
    let mut new_doc = TestDoc::default();
    *new_doc.test_mut() = "testing".to_string();
    let update = UpdateConfigBuilder::default()
        .collection("test_coll")
        .key(key)
        .document(new_doc)
        .return_old(true)
        .return_new(true)
        .build()?;
    let replace_res: ArangoEither<DocMeta<TestDoc, TestDoc>> = conn.update(update).await?;
    assert!(replace_res.is_right());
    let doc_meta = replace_res.right_safe()?;
    let key = doc_meta.key();
    let doc_opt = doc_meta.new_doc();
    assert!(doc_opt.is_some());
    assert_eq!(unwrap_doc(doc_opt)?.test(), "testing");
    let old_doc_opt = doc_meta.old_doc();
    assert!(old_doc_opt.is_some());
    assert_eq!(unwrap_doc(old_doc_opt)?.test(), "test");

    // Delete that document
    let delete_config = DeleteConfigBuilder::default()
        .collection("test_coll")
        .key(key)
        .return_old(true)
        .build()?;
    let delete_res: ArangoEither<DocMeta<(), TestDoc>> = conn.delete(delete_config).await?;
    assert!(delete_res.is_right());
    let doc_meta = delete_res.right_safe()?;
    let doc_opt = doc_meta.old_doc();
    assert!(doc_opt.is_some());
    assert_eq!(unwrap_doc(doc_opt)?.test(), "testing");

    Ok(())
}

#[tokio::test]
async fn creates_updates_deletes_basic() -> Result<()> {
    let conn = conn(ConnKind::Ruarango).await?;
    let docs = vec![TestDoc::default(), TestDoc::default(), TestDoc::default()];

    // Create some documents
    let create_config = CreatesConfigBuilder::default()
        .collection("test_coll")
        .document(docs.clone())
        .build()?;
    let create_res: ArangoEither<ArangoVec<DocMeta<(), ()>>> = conn.creates(create_config).await?;
    assert!(create_res.is_right());
    let doc_meta_vec = create_res.right_safe()?;
    assert_eq!(doc_meta_vec.len(), docs.len());

    let mut keys = vec![];
    for doc_meta_either in doc_meta_vec {
        assert!(doc_meta_either.is_right());
        let doc_meta = doc_meta_either.right_safe()?;
        keys.push(doc_meta.key().clone());
    }
    assert_eq!(keys.len(), docs.len());

    // Update the documents
    let update_docs: Vec<TestDoc> = docs
        .iter()
        .zip(keys.clone())
        .map(|(doc, key)| {
            let mut new_doc = doc.clone();
            *new_doc.key_mut() = Some(key.clone());
            *new_doc.test_mut() = "blah".to_string();
            new_doc
        })
        .collect();
    assert_eq!(update_docs.len(), 3);
    let len = update_docs.len();
    let updates_config = UpdatesConfigBuilder::default()
        .collection("test_coll")
        .documents(update_docs)
        .return_old(true)
        .return_new(true)
        .build()?;
    let updates_res: ArangoEither<ArangoVec<DocMeta<TestDoc, TestDoc>>> =
        conn.updates(updates_config).await?;
    assert!(updates_res.is_right());
    let doc_meta_vec = updates_res.right_safe()?;
    assert_eq!(doc_meta_vec.len(), len);
    for doc_meta_either in doc_meta_vec {
        assert!(doc_meta_either.is_right());
        let doc_meta = doc_meta_either.right_safe()?;
        let old_doc_opt = doc_meta.old_doc();
        assert!(old_doc_opt.is_some());
        assert_eq!(unwrap_doc(old_doc_opt)?.test(), "test");
        let new_doc_opt = doc_meta.new_doc();
        assert!(new_doc_opt.is_some());
        assert_eq!(unwrap_doc(new_doc_opt)?.test(), "blah");
    }

    // Delete the documents
    let delete_config = DeletesConfigBuilder::default()
        .collection("test_coll")
        .documents(keys)
        .return_old(true)
        .build()?;
    let delete_res: ArangoEither<ArangoVec<DocMeta<(), TestDoc>>> =
        conn.deletes(delete_config).await?;
    assert!(delete_res.is_right());
    let doc_meta_vec = delete_res.right_safe()?;
    assert_eq!(doc_meta_vec.len(), docs.len());

    for doc_meta_either in doc_meta_vec {
        assert!(doc_meta_either.is_right());
        let doc_meta = doc_meta_either.right_safe()?;
        let doc_opt = doc_meta.old_doc();
        assert!(doc_opt.is_some());
        assert_eq!(unwrap_doc(doc_opt)?.test(), "blah");
    }

    Ok(())
}
