use crate::{
    common::process_async_doc_result,
    model::{unwrap_doc, OutputDoc, SearchDoc, TestDoc},
    pool::{RUARANGO_ASYNC_POOL, RUARANGO_POOL},
};
use anyhow::Result;
use ruarango::{
    doc::{
        input::{
            CreateConfigBuilder, CreatesConfigBuilder, DeleteConfigBuilder, DeletesConfigBuilder,
            ReadConfig, ReadConfigBuilder, ReadsConfigBuilder, ReplaceConfigBuilder,
            UpdateConfigBuilder, UpdatesConfigBuilder,
        },
        output::DocMeta,
    },
    ArangoEither, ArangoResult, ArangoVec, Connection, Document,
    Error::{self, NotFound, PreconditionFailed},
};

const TEST_COLL: &str = "test_coll";
const DOC_KEY: &str = "2916385";
const ACTUAL_REV: &str = r#""_cRwlNbO---""#;
const FAKE_REV: &str = r#""_cJG9Tz1---""#;
const TEST_FIELD_VAL: &str = "tester";

#[ignore = "This seems to give back a 304 Not Modified rather than the result"]
#[tokio::test]
async fn doc_read_async() -> Result<()> {
    let conn = &*RUARANGO_ASYNC_POOL.get()?;
    let config = ReadConfigBuilder::default()
        .collection(TEST_COLL)
        .key(DOC_KEY)
        .build()?;
    let res: ArangoEither<OutputDoc> = conn.read(config).await?;
    assert!(res.is_left());
    let doc: OutputDoc = process_async_doc_result(res, conn).await?;
    assert_eq!(doc.test(), TEST_FIELD_VAL);
    Ok(())
}

#[tokio::test]
async fn doc_read() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let config = ReadConfigBuilder::default()
        .collection(TEST_COLL)
        .key(DOC_KEY)
        .build()?;
    let res: ArangoEither<OutputDoc> = conn.read(config).await?;
    assert!(res.is_right());
    let doc = res.right_safe()?;
    assert_eq!(doc.test(), TEST_FIELD_VAL);
    Ok(())
}

#[tokio::test]
async fn doc_reads() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let search_docs = vec![SearchDoc::new(DOC_KEY), SearchDoc::new("abcd")];
    let config = ReadsConfigBuilder::default()
        .collection(TEST_COLL)
        .documents(search_docs)
        .build()?;
    let res: ArangoEither<ArangoVec<OutputDoc>> = conn.reads(config).await?;
    assert!(res.is_right());
    let docs = res.right_safe()?;
    assert_eq!(docs.len(), 2);
    let output_doc = docs.get(0).unwrap().clone();
    assert!(output_doc.is_right());
    let doc = output_doc.right_safe()?;
    assert_eq!(doc.key(), DOC_KEY);
    assert_eq!(doc.test(), TEST_FIELD_VAL);
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
            .collection(TEST_COLL)
            .key(DOC_KEY)
            .if_none_match(ACTUAL_REV)
            .build()?,
        IfNoneMatchKind::NoneMatch => ReadConfigBuilder::default()
            .collection(TEST_COLL)
            .key(DOC_KEY)
            .if_none_match(FAKE_REV)
            .build()?,
    })
}

#[ignore = "upstream call is flaky for some reason"]
#[tokio::test]
async fn doc_read_if_none_match_matches_async() -> Result<()> {
    let conn = &*RUARANGO_ASYNC_POOL.get()?;
    let res: ArangoEither<OutputDoc> = conn
        .read(if_none_match_config(IfNoneMatchKind::Match)?)
        .await?;
    let none_match: Result<OutputDoc> = process_async_doc_result(res, conn).await;
    assert!(none_match.is_err());
    Ok(())
}

#[tokio::test]
async fn doc_read_if_none_match_matches() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let res: ArangoResult<OutputDoc> = conn
        .read(if_none_match_config(IfNoneMatchKind::Match)?)
        .await;
    assert!(res.is_err());
    Ok(())
}

#[ignore = "upstream call is flaky for some reason"]
#[tokio::test]
async fn doc_read_if_none_match_doesnt_match_async() -> Result<()> {
    let conn = &*RUARANGO_ASYNC_POOL.get()?;
    let res: ArangoEither<OutputDoc> = conn
        .read(if_none_match_config(IfNoneMatchKind::NoneMatch)?)
        .await?;
    let doc: OutputDoc = process_async_doc_result(res, conn).await?;
    assert_eq!(doc.test(), TEST_FIELD_VAL);
    Ok(())
}

#[tokio::test]
async fn doc_read_if_none_match_doesnt_match() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let either: ArangoEither<OutputDoc> = conn
        .read(if_none_match_config(IfNoneMatchKind::NoneMatch)?)
        .await?;
    assert!(either.is_right());
    let doc = either.right_safe()?;
    assert_eq!(doc.test(), TEST_FIELD_VAL);
    Ok(())
}

enum IfMatchKind {
    Match,
    NoneMatch,
}

fn if_match_config(kind: IfMatchKind) -> Result<ReadConfig> {
    Ok(match kind {
        IfMatchKind::Match => ReadConfigBuilder::default()
            .collection(TEST_COLL)
            .key(DOC_KEY)
            .if_match(ACTUAL_REV)
            .build()?,
        IfMatchKind::NoneMatch => ReadConfigBuilder::default()
            .collection(TEST_COLL)
            .key(DOC_KEY)
            .if_match(FAKE_REV)
            .build()?,
    })
}

#[tokio::test]
async fn doc_read_if_match_matches() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let either: ArangoEither<OutputDoc> = conn.read(if_match_config(IfMatchKind::Match)?).await?;
    assert!(either.is_right());
    let doc = either.right_safe()?;
    assert_eq!(doc.test(), TEST_FIELD_VAL);
    Ok(())
}

#[tokio::test]
async fn doc_read_if_match_doesnt_match() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
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
    let conn = &*RUARANGO_POOL.get()?;
    let res: ArangoResult<OutputDoc> = conn
        .read(
            ReadConfigBuilder::default()
                .collection(TEST_COLL)
                .key("yoda")
                .build()?,
        )
        .await;
    match res {
        Ok(_) => panic!("This should be an error!"),
        Err(e) => {
            let err = e.downcast_ref::<Error>().expect("unanticipated error");
            match err {
                NotFound { err } => {
                    assert!(err.is_some());
                }
                _ => panic!("Wrong error kind!"),
            }
        }
    }
    Ok(())
}

pub async fn create_doc(conn: &Connection) -> Result<String> {
    // Create a document
    let create_config = CreateConfigBuilder::default()
        .collection(TEST_COLL)
        .document(TestDoc::default())
        .build()?;
    let create_res: ArangoEither<DocMeta<(), ()>> = conn.create(create_config).await?;
    assert!(create_res.is_right());
    let doc_meta = create_res.right_safe()?;
    Ok(doc_meta.key().clone())
}

pub async fn create_docs(conn: &Connection, count: usize) -> Result<Vec<String>> {
    let docs: Vec<TestDoc> = (0..count).map(|_| TestDoc::default()).collect();
    let create_config = CreatesConfigBuilder::default()
        .collection(TEST_COLL)
        .document(docs)
        .build()?;
    let create_res: ArangoEither<ArangoVec<DocMeta<(), ()>>> = conn.creates(create_config).await?;
    assert!(create_res.is_right());
    let doc_meta_vec = create_res.right_safe()?;
    assert_eq!(doc_meta_vec.len(), count);

    let mut keys = vec![];
    for doc_meta_either in doc_meta_vec {
        assert!(doc_meta_either.is_right());
        let doc_meta = doc_meta_either.right_safe()?;
        keys.push(doc_meta.key().clone());
    }
    Ok(keys)
}

pub async fn delete_doc(conn: &Connection, key: &str, val: &str) -> Result<()> {
    let delete_config = DeleteConfigBuilder::default()
        .collection(TEST_COLL)
        .key(key)
        .return_old(true)
        .build()?;
    let delete_res: ArangoEither<DocMeta<(), TestDoc>> = conn.delete(delete_config).await?;
    assert!(delete_res.is_right());
    let doc_meta = delete_res.right_safe()?;
    let doc_opt = doc_meta.old_doc();
    assert!(doc_opt.is_some());
    assert_eq!(unwrap_doc(doc_opt)?.test(), val);
    Ok(())
}

pub async fn delete_docs(conn: &Connection, keys: Vec<String>, val: &str) -> Result<()> {
    let len = keys.len();
    let delete_config = DeletesConfigBuilder::default()
        .collection(TEST_COLL)
        .documents(keys)
        .return_old(true)
        .build()?;
    let delete_res: ArangoEither<ArangoVec<DocMeta<(), TestDoc>>> =
        conn.deletes(delete_config).await?;
    assert!(delete_res.is_right());
    let doc_meta_vec = delete_res.right_safe()?;
    assert_eq!(doc_meta_vec.len(), len);

    for doc_meta_either in doc_meta_vec {
        assert!(doc_meta_either.is_right());
        let doc_meta = doc_meta_either.right_safe()?;
        let doc_opt = doc_meta.old_doc();
        assert!(doc_opt.is_some());
        assert_eq!(unwrap_doc(doc_opt)?.test(), val);
    }
    Ok(())
}

#[tokio::test]
async fn doc_create_delete_basic() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let key = create_doc(conn).await?;
    delete_doc(conn, &key, "test").await
}

#[tokio::test]
async fn doc_creates_deletes_basic() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let keys = create_docs(conn, 3).await?;
    delete_docs(conn, keys, "test").await
}

#[tokio::test]
async fn doc_create_overwrite_replace_delete() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;

    // Create a document
    let key = create_doc(conn).await?;

    // Overwrite with replace
    let mut new_doc = TestDoc::default();
    *new_doc.key_mut() = Some(key.clone());
    *new_doc.test_mut() = "testing".to_string();
    let overwrite = CreateConfigBuilder::default()
        .collection(TEST_COLL)
        .document(new_doc)
        .overwrite(true)
        .build()?;
    let overwrite_res: ArangoEither<DocMeta<(), ()>> = conn.create(overwrite).await?;
    assert!(overwrite_res.is_right());
    let doc_meta = overwrite_res.right_safe()?;
    let key = doc_meta.key();

    // Delete that document
    delete_doc(conn, key, "testing").await
}

#[tokio::test]
async fn doc_create_replace_delete() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;

    // Create a document
    let key = create_doc(conn).await?;

    // Replace
    let mut new_doc = TestDoc::default();
    *new_doc.test_mut() = "testing".to_string();
    let replace = ReplaceConfigBuilder::default()
        .collection(TEST_COLL)
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
    delete_doc(conn, key, "testing").await
}

#[tokio::test]
async fn doc_create_update_delete() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;

    // Create a document
    let key = create_doc(conn).await?;

    // Update
    let mut new_doc = TestDoc::default();
    *new_doc.test_mut() = "testing".to_string();
    let update = UpdateConfigBuilder::default()
        .collection(TEST_COLL)
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
    delete_doc(conn, key, "testing").await
}

#[tokio::test]
async fn doc_creates_updates_deletes_basic() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;

    // Create some documents
    let keys = create_docs(conn, 3).await?;

    // Update the documents
    let update_docs: Vec<TestDoc> = keys
        .iter()
        .map(|key| {
            let mut new_doc = TestDoc::default();
            *new_doc.key_mut() = Some(key.clone());
            *new_doc.test_mut() = "blah".to_string();
            new_doc
        })
        .collect();
    assert_eq!(update_docs.len(), 3);
    let len = update_docs.len();
    let updates_config = UpdatesConfigBuilder::default()
        .collection(TEST_COLL)
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
    delete_docs(conn, keys, "blah").await
}
