use crate::{
    model::TestDoc,
    pool::RUARANGO_POOL,
    rand_util::{
        create_random_collection, create_random_document, create_random_graph,
        delete_random_collection, delete_random_graph, CollKind,
    },
};
use anyhow::Result;
use ruarango::{
    graph::{
        input::{
            CreateEdgeDefConfigBuilder, DeleteEdgeDefConfigBuilder, EdgeCreateConfigBuilder,
            EdgeDeleteConfigBuilder, EdgeReadConfigBuilder, EdgeReplaceConfigBuilder,
            EdgeUpdateConfigBuilder, FromToBuilder, ReadConfigBuilder, ReadEdgeDefsConfigBuilder,
            ReadVertexCollsConfigBuilder, ReplaceEdgeDefConfigBuilder,
        },
        EdgeDefinitionBuilder,
    },
    Graph,
};
use serde_derive::Serialize;

#[tokio::test]
async fn graph_list_all() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let graph_meta = create_random_graph(&conn).await?;

    let res = conn.list().await?;
    assert!(res.is_right());
    let list = res.right_safe()?;
    assert!(!list.error());
    assert_eq!(*list.code(), 200);
    assert!(list.graphs().len() >= 1);

    for graph in list.graphs() {
        assert!(!graph.id().is_empty());
        assert!(!graph.key().is_empty());
        assert!(!graph.rev().is_empty());
        assert!(!graph.name().is_empty());
        assert!(graph.edge_definitions().len() >= 1);
        let ed = graph.edge_definitions().get(0).unwrap();
        assert_eq!(ed.to().len(), 1);
        assert_eq!(ed.from().len(), 1);
    }

    delete_random_graph(&conn, graph_meta).await
}

#[tokio::test]
async fn graph_create_delete() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let graph_meta = create_random_graph(&conn).await?;
    delete_random_graph(&conn, graph_meta).await
}

#[tokio::test]
async fn graph_read() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let rand_graph_meta = create_random_graph(&conn).await?;

    let config = ReadConfigBuilder::default()
        .name(rand_graph_meta.graph())
        .build()?;
    let res = conn.read(config).await?;
    assert!(res.is_right());
    let graph_meta = res.right_safe()?;
    assert!(!graph_meta.error());
    assert_eq!(*graph_meta.code(), 200);
    let graph = graph_meta.graph();

    assert!(!graph.id().is_empty());
    assert!(!graph.key().is_empty());
    assert!(!graph.rev().is_empty());
    assert_eq!(graph.name(), rand_graph_meta.graph());
    assert_eq!(graph.edge_definitions().len(), 1);
    let ed = graph.edge_definitions().get(0).unwrap();
    assert_eq!(ed.to().len(), 1);
    assert_eq!(ed.from().len(), 1);

    delete_random_graph(&conn, rand_graph_meta).await
}

#[tokio::test]
async fn graph_create_delete_edge() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let rand_graph_meta = create_random_graph(&conn).await?;
    let graph_name = rand_graph_meta.graph();
    let edge_coll = rand_graph_meta.edge_coll();
    let from_coll = rand_graph_meta.from_coll();
    let to_coll = rand_graph_meta.to_coll();
    let from_doc = create_random_document(&conn, from_coll, TestDoc::default()).await?;
    let to_doc = create_random_document(&conn, to_coll, TestDoc::default()).await?;

    let from_to = FromToBuilder::default()
        .from(from_doc.id())
        .to(to_doc.id())
        .build()?;
    let config = EdgeCreateConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .mapping(from_to)
        .return_new(true)
        .build()?;
    let res = conn.create_edge(config).await?;
    assert!(res.is_right());
    let create_edge = res.right_safe()?;
    assert!(!create_edge.error());
    assert_eq!(*create_edge.code(), 202);
    let edge = create_edge.edge();
    assert!(!edge.id().is_empty());
    assert!(!edge.key().is_empty());
    assert!(!edge.rev().is_empty());
    assert!(edge.from().is_none());
    assert!(edge.to().is_none());
    let key = edge.key();
    let new = create_edge.new().as_ref().unwrap();
    assert!(!new.id().is_empty());
    assert!(!new.key().is_empty());
    assert!(!new.rev().is_empty());
    assert!(new.from().is_some());
    assert!(new.to().is_some());

    let delete_config = EdgeDeleteConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .key(key)
        .build()?;
    let res = conn.delete_edge(delete_config).await?;
    assert!(res.is_right());
    let delete_edge = res.right_safe()?;
    assert!(!delete_edge.error());
    assert_eq!(*delete_edge.code(), 202);
    assert!(delete_edge.removed());
    assert!(delete_edge.old().is_none());

    delete_random_graph(&conn, rand_graph_meta).await
}

#[tokio::test]
async fn graph_create_read_delete_edge() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let rand_graph_meta = create_random_graph(&conn).await?;
    let graph_name = rand_graph_meta.graph();
    let edge_coll = rand_graph_meta.edge_coll();
    let from_coll = rand_graph_meta.from_coll();
    let to_coll = rand_graph_meta.to_coll();
    let from_doc = create_random_document(&conn, from_coll, TestDoc::default()).await?;
    let to_doc = create_random_document(&conn, to_coll, TestDoc::default()).await?;

    let from_to = FromToBuilder::default()
        .from(from_doc.id())
        .to(to_doc.id())
        .build()?;
    let config = EdgeCreateConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .mapping(from_to)
        .return_new(true)
        .build()?;
    let res = conn.create_edge(config).await?;
    assert!(res.is_right());
    let create_edge = res.right_safe()?;
    assert!(!create_edge.error());
    assert_eq!(*create_edge.code(), 202);
    let edge = create_edge.edge();
    let key = edge.key();

    let read_config = EdgeReadConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .key(key)
        .build()?;
    let res = conn.read_edge(read_config).await?;
    assert!(res.is_right());
    let read_edge = res.right_safe()?;
    assert!(!read_edge.error());

    let delete_config = EdgeDeleteConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .key(key)
        .build()?;
    let res = conn.delete_edge(delete_config).await?;
    assert!(res.is_right());
    let delete_edge = res.right_safe()?;
    assert!(!delete_edge.error());
    assert_eq!(*delete_edge.code(), 202);

    delete_random_graph(&conn, rand_graph_meta).await
}

#[derive(Clone, Copy, Debug, Serialize)]
struct EdgeStuff {
    name: &'static str,
}

#[tokio::test]
async fn graph_create_update_delete_edge() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let rand_graph_meta = create_random_graph(&conn).await?;
    let graph_name = rand_graph_meta.graph();
    let edge_coll = rand_graph_meta.edge_coll();
    let from_coll = rand_graph_meta.from_coll();
    let to_coll = rand_graph_meta.to_coll();
    let from_doc = create_random_document(&conn, from_coll, TestDoc::default()).await?;
    let to_doc = create_random_document(&conn, to_coll, TestDoc::default()).await?;

    let from_to = FromToBuilder::default()
        .from(from_doc.id())
        .to(to_doc.id())
        .build()?;
    let config = EdgeCreateConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .mapping(from_to)
        .return_new(true)
        .build()?;
    let res = conn.create_edge(config).await?;
    assert!(res.is_right());
    let create_edge = res.right_safe()?;
    assert!(!create_edge.error());
    assert_eq!(*create_edge.code(), 202);
    let edge = create_edge.edge();
    let key = edge.key();

    let update_config = EdgeUpdateConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .key(key)
        .edge(EdgeStuff { name: "yoda" })
        .build()?;
    let res = conn.update_edge(update_config).await?;
    assert!(res.is_right());
    let update_edge = res.right_safe()?;
    assert!(!update_edge.error());
    assert_eq!(*update_edge.code(), 202);

    let delete_config = EdgeDeleteConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .key(key)
        .build()?;
    let res = conn.delete_edge(delete_config).await?;
    assert!(res.is_right());
    let delete_edge = res.right_safe()?;
    assert!(!delete_edge.error());
    assert_eq!(*delete_edge.code(), 202);

    delete_random_graph(&conn, rand_graph_meta).await
}

#[tokio::test]
async fn graph_create_replace_delete_edge() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let rand_graph_meta = create_random_graph(&conn).await?;
    let graph_name = rand_graph_meta.graph();
    let edge_coll = rand_graph_meta.edge_coll();
    let from_coll = rand_graph_meta.from_coll();
    let to_coll = rand_graph_meta.to_coll();
    let from_doc = create_random_document(&conn, from_coll, TestDoc::default()).await?;
    let to_doc = create_random_document(&conn, to_coll, TestDoc::default()).await?;

    let from_to = FromToBuilder::default()
        .from(from_doc.id())
        .to(to_doc.id())
        .build()?;
    let config = EdgeCreateConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .mapping(from_to)
        .return_new(true)
        .build()?;
    let res = conn.create_edge(config).await?;
    assert!(res.is_right());
    let create_edge = res.right_safe()?;
    assert!(!create_edge.error());
    assert_eq!(*create_edge.code(), 202);
    let edge = create_edge.edge();
    let key = edge.key();

    let new_from_doc = create_random_document(&conn, from_coll, TestDoc::default()).await?;
    let new_to_doc = create_random_document(&conn, to_coll, TestDoc::default()).await?;
    let from_to_new = FromToBuilder::default()
        .to(new_to_doc.id())
        .from(new_from_doc.id())
        .build()?;
    let replace_config = EdgeReplaceConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .key(key)
        .edge(from_to_new)
        .build()?;
    let res = conn.replace_edge(replace_config).await?;
    assert!(res.is_right());
    let replace_edge = res.right_safe()?;
    assert!(!replace_edge.error());
    assert_eq!(*replace_edge.code(), 202);

    let delete_config = EdgeDeleteConfigBuilder::default()
        .graph(graph_name)
        .collection(edge_coll)
        .key(key)
        .build()?;
    let res = conn.delete_edge(delete_config).await?;
    assert!(res.is_right());
    let delete_edge = res.right_safe()?;
    assert!(!delete_edge.error());
    assert_eq!(*delete_edge.code(), 202);

    delete_random_graph(&conn, rand_graph_meta).await
}

#[tokio::test]
async fn graph_create_delete_edge_def() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let rand_graph_meta = create_random_graph(&conn).await?;
    let graph_name = rand_graph_meta.graph();
    let (edge_coll, _) = create_random_collection(&conn, CollKind::Edge).await?;
    let (from_coll, _) = create_random_collection(&conn, CollKind::Document).await?;
    let (to_coll, _) = create_random_collection(&conn, CollKind::Document).await?;

    let edge_def = EdgeDefinitionBuilder::default()
        .collection(&edge_coll)
        .from(vec![from_coll.clone()])
        .to(vec![to_coll.clone()])
        .build()?;
    let config = CreateEdgeDefConfigBuilder::default()
        .graph(graph_name)
        .edge_def(edge_def)
        .build()?;
    let res = conn.create_edge_def(config).await?;
    assert!(res.is_right());
    let create_edge_def = res.right_safe()?;
    assert!(!create_edge_def.error());
    assert_eq!(*create_edge_def.code(), 202);
    let graph = create_edge_def.graph();
    assert_eq!(graph.name(), graph_name);

    let delete_config = DeleteEdgeDefConfigBuilder::default()
        .graph(graph_name)
        .edge_def(&edge_coll)
        .build()?;
    let res = conn.delete_edge_def(delete_config).await?;
    assert!(res.is_right());
    let delete_edge_def = res.right_safe()?;
    assert!(!delete_edge_def.error());
    assert_eq!(*delete_edge_def.code(), 202);

    let _ = delete_random_collection(&conn, &to_coll).await?;
    let _ = delete_random_collection(&conn, &from_coll).await?;
    let _ = delete_random_collection(&conn, &edge_coll).await?;

    delete_random_graph(&conn, rand_graph_meta).await
}

#[tokio::test]
async fn graph_read_edge_defs() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let rand_graph_meta = create_random_graph(&conn).await?;

    let config = ReadEdgeDefsConfigBuilder::default()
        .name(rand_graph_meta.graph())
        .build()?;
    let res = conn.read_edge_defs(config).await?;
    assert!(res.is_right());
    let graph_meta = res.right_safe()?;
    assert!(!graph_meta.error());
    assert_eq!(*graph_meta.code(), 200);
    assert!(graph_meta.collections().len() >= 1);

    delete_random_graph(&conn, rand_graph_meta).await
}

#[tokio::test]
async fn graph_create_replace_delete_edge_def() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let rand_graph_meta = create_random_graph(&conn).await?;
    let graph_name = rand_graph_meta.graph();
    let (edge_coll, _) = create_random_collection(&conn, CollKind::Edge).await?;
    let (from_coll, _) = create_random_collection(&conn, CollKind::Document).await?;
    let (to_coll, _) = create_random_collection(&conn, CollKind::Document).await?;

    let edge_def = EdgeDefinitionBuilder::default()
        .collection(&edge_coll)
        .from(vec![from_coll.clone()])
        .to(vec![to_coll.clone()])
        .build()?;
    let config = CreateEdgeDefConfigBuilder::default()
        .graph(graph_name)
        .edge_def(edge_def)
        .build()?;
    let res = conn.create_edge_def(config).await?;
    assert!(res.is_right());
    let create_edge_def = res.right_safe()?;
    assert!(!create_edge_def.error());
    assert_eq!(*create_edge_def.code(), 202);
    let graph = create_edge_def.graph();
    assert_eq!(graph.name(), graph_name);

    let edge_def = EdgeDefinitionBuilder::default()
        .collection(&edge_coll)
        .from(vec![to_coll.clone()])
        .to(vec![from_coll.clone()])
        .build()?;
    let replace_config = ReplaceEdgeDefConfigBuilder::default()
        .graph(graph_name)
        .edge_def(edge_def)
        .build()?;
    let res = conn.replace_edge_def(replace_config).await?;
    assert!(res.is_right());
    let replace_edge_def = res.right_safe()?;
    assert!(!replace_edge_def.error());
    assert_eq!(*replace_edge_def.code(), 202);

    let delete_config = DeleteEdgeDefConfigBuilder::default()
        .graph(graph_name)
        .edge_def(&edge_coll)
        .build()?;
    let res = conn.delete_edge_def(delete_config).await?;
    assert!(res.is_right());
    let delete_edge_def = res.right_safe()?;
    assert!(!delete_edge_def.error());
    assert_eq!(*delete_edge_def.code(), 202);

    let _ = delete_random_collection(&conn, &to_coll).await?;
    let _ = delete_random_collection(&conn, &from_coll).await?;
    let _ = delete_random_collection(&conn, &edge_coll).await?;

    delete_random_graph(&conn, rand_graph_meta).await
}

#[tokio::test]
async fn graph_read_vertex_colls() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let rand_graph_meta = create_random_graph(&conn).await?;

    let config = ReadVertexCollsConfigBuilder::default()
        .name(rand_graph_meta.graph())
        .build()?;
    let res = conn.read_vertex_colls(config).await?;
    assert!(res.is_right());
    let vertex_colls = res.right_safe()?;
    assert!(!vertex_colls.error());
    assert_eq!(*vertex_colls.code(), 200);
    assert!(vertex_colls.collections().len() >= 1);

    delete_random_graph(&conn, rand_graph_meta).await
}
