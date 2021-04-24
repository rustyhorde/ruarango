use crate::{
    model::{unwrap_doc, OutputDoc, TestDoc},
    pool::RUARANGO_POOL,
};
use anyhow::Result;
use ruarango::{
    cursor::{
        input::{
            CreateConfigBuilder, DeleteConfigBuilder, NextConfigBuilder, OptionsBuilder,
            ProfileKind,
        },
        output::CursorMeta,
    },
    doc::{
        input::{CreatesConfigBuilder, DeletesConfigBuilder},
        output::DocMeta,
    },
    ArangoEither, ArangoResult, ArangoVec, Cursor, Document,
    Error::{self, Cursor as CursorError},
};

#[tokio::test]
async fn cursor_create() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let config = CreateConfigBuilder::default()
        .query("FOR d IN test_coll RETURN d")
        .count(true)
        .build()?;
    let res: ArangoEither<CursorMeta<OutputDoc>> = Cursor::create(conn, config).await?;
    assert!(res.is_right());
    let cursor_meta = res.right_safe()?;
    assert!(cursor_meta.result().is_some());
    assert!(cursor_meta.result().as_ref().unwrap().len() >= 1);
    assert!(cursor_meta.count().is_some());
    assert!(*cursor_meta.count().as_ref().unwrap() >= 1);
    assert!(cursor_meta.id().is_none());
    assert!(!cursor_meta.has_more());
    assert!(!cursor_meta.cached());
    assert!(!cursor_meta.error());
    assert_eq!(*cursor_meta.code(), 201);
    assert!(cursor_meta.extra().is_some());
    let extra = cursor_meta.extra().as_ref().unwrap();
    assert_eq!(*extra.stats().writes_executed(), 0);
    assert_eq!(*extra.stats().writes_ignored(), 0);
    assert!(*extra.stats().scanned_full() >= 1);
    assert_eq!(*extra.stats().scanned_index(), 0);
    assert_eq!(*extra.stats().filtered(), 0);
    assert_eq!(*extra.stats().http_requests(), 0);
    assert!(*extra.stats().execution_time() > 0.);
    assert!(*extra.stats().peak_memory_usage() > 0);
    Ok(())
}

#[tokio::test]
async fn cursor_create_profile() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let options = OptionsBuilder::default()
        .profile(ProfileKind::ProfileOnly)
        .build()?;
    let config = CreateConfigBuilder::default()
        .query("FOR d IN test_coll RETURN d")
        .count(true)
        .options(options)
        .build()?;
    let res: ArangoEither<CursorMeta<OutputDoc>> = Cursor::create(conn, config).await?;
    assert!(res.is_right());
    let cursor_meta = res.right_safe()?;
    assert!(cursor_meta.result().is_some());
    assert!(cursor_meta.result().as_ref().unwrap().len() >= 1);
    assert!(cursor_meta.count().is_some());
    assert!(*cursor_meta.count().as_ref().unwrap() >= 1);
    assert!(cursor_meta.id().is_none());
    assert!(!cursor_meta.has_more());
    assert!(!cursor_meta.cached());
    assert!(!cursor_meta.error());
    assert_eq!(*cursor_meta.code(), 201);
    assert!(cursor_meta.extra().is_some());
    let extra = cursor_meta.extra().as_ref().unwrap();
    assert_eq!(*extra.stats().writes_executed(), 0);
    assert_eq!(*extra.stats().writes_ignored(), 0);
    assert!(*extra.stats().scanned_full() >= 1);
    assert_eq!(*extra.stats().scanned_index(), 0);
    assert_eq!(*extra.stats().filtered(), 0);
    assert_eq!(*extra.stats().http_requests(), 0);
    assert!(*extra.stats().execution_time() > 0.);
    assert!(*extra.stats().peak_memory_usage() > 0);
    assert!(extra.warnings().is_empty());
    assert!(extra.profile().is_some());
    let profile = extra.profile().unwrap();
    assert!(*profile.initializing() > 0.);
    assert!(*profile.parsing() > 0.);
    assert!(*profile.optimizing_ast() > 0.);
    assert!(*profile.loading_collections() > 0.);
    assert!(*profile.instantiating_plan() > 0.);
    assert!(*profile.optimizing_plan() > 0.);
    assert!(*profile.executing() > 0.);
    assert!(*profile.finalizing() > 0.);
    Ok(())
}

#[tokio::test]
async fn cursor_create_400() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let config = CreateConfigBuilder::default().query("YODA").build()?;
    let res: ArangoResult<CursorMeta<OutputDoc>> = Cursor::create(conn, config).await;
    match res {
        Ok(_) => panic!("This call should fail!"),
        Err(e) => {
            let opt_err = e.downcast_ref::<Error>();
            assert!(opt_err.is_some());
            let ruarango_err = opt_err.unwrap();
            match ruarango_err {
                CursorError { err } => {
                    assert!(err.is_some());
                    let err = err.as_ref().unwrap();
                    assert!(err.error());
                    assert_eq!(*err.code(), 400);
                    assert_eq!(*err.error_num(), 1501);
                    assert!(err.error_message().is_some());
                    let msg = err.error_message().as_ref().unwrap();
                    assert_eq!(msg, "AQL: syntax error, unexpected identifier near 'YODA' at position 1:1 (while parsing)");
                }
                _ => panic!("This is the wrong error type!"),
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn cursor_create_404() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let config = CreateConfigBuilder::default()
        .query("REMOVE 'yoda' IN test_coll")
        .build()?;
    let res: ArangoResult<CursorMeta<OutputDoc>> = Cursor::create(conn, config).await;
    match res {
        Ok(_) => panic!("This call should fail!"),
        Err(e) => {
            let opt_err = e.downcast_ref::<Error>();
            assert!(opt_err.is_some());
            let ruarango_err = opt_err.unwrap();
            match ruarango_err {
                CursorError { err } => {
                    assert!(err.is_some());
                    let err = err.as_ref().unwrap();
                    assert!(err.error());
                    assert_eq!(*err.code(), 404);
                    assert_eq!(*err.error_num(), 1202);
                    assert!(err.error_message().is_some());
                    let msg = err.error_message().as_ref().unwrap();
                    assert_eq!(msg, "AQL: document not found (while executing)");
                }
                _ => panic!("This is the wrong error type!"),
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn cursor_delete() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
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

    // Cursor
    let config = CreateConfigBuilder::default()
        .query("FOR d IN test_coll LIMIT 5 RETURN d")
        .batch_size(2)
        .count(true)
        .build()?;
    let res: ArangoEither<CursorMeta<OutputDoc>> = Cursor::create(conn, config).await?;
    assert!(res.is_right());
    let cursor_meta = res.right_safe()?;
    assert!(cursor_meta.has_more());
    assert!(cursor_meta.id().is_some());
    let id = cursor_meta.id().as_ref().unwrap();

    // Delete the cursor
    let config = DeleteConfigBuilder::default().id(id).build()?;
    let res: ArangoEither<()> = Cursor::delete(conn, config).await?;
    assert!(res.is_right());

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
async fn cursor_next() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
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

    // Cursor
    let config = CreateConfigBuilder::default()
        .query("FOR d IN test_coll LIMIT 5 RETURN d")
        .batch_size(2)
        .count(true)
        .build()?;
    let res: ArangoEither<CursorMeta<OutputDoc>> = Cursor::create(conn, config).await?;
    assert!(res.is_right());
    let cursor_meta = res.right_safe()?;
    assert!(cursor_meta.has_more());
    assert!(cursor_meta.id().is_some());
    let id = cursor_meta.id().as_ref().unwrap();
    assert_eq!(cursor_meta.result().as_ref().unwrap().len(), 2);

    // Get the next batch
    let config = NextConfigBuilder::default().id(id).build()?;
    let res: ArangoEither<CursorMeta<OutputDoc>> = conn.next(config).await?;
    assert!(res.is_right());
    assert!(cursor_meta.id().is_some());
    assert_eq!(cursor_meta.result().as_ref().unwrap().len(), 2);

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
