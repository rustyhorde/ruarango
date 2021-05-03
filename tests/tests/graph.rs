use crate::{pool::RUARANGO_POOL, rand_util::rand_name};
use anyhow::Result;
use ruarango::{
    graph::{
        input::{
            CreateConfigBuilder, DeleteConfigBuilder, EdgeCreateConfigBuilder,
            EdgeDeleteConfigBuilder, EdgeReadConfigBuilder, FromToBuilder, GraphMetaBuilder,
            ListEdgesConfigBuilder, ReadConfigBuilder,
        },
        output::{CreateEdge, DeleteEdge, EdgesMeta, GraphMeta, List, ReadEdge},
        EdgeDefinition, EdgeDefinitionBuilder,
    },
    ArangoEither, Graph,
};

#[tokio::test]
async fn graph_list_all() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let res: ArangoEither<List> = conn.list().await?;
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
        assert_eq!(graph.orphan_collections().len(), 0);

        if graph.name() == "test_graph" {
            assert_eq!(graph.edge_definitions().len(), 1);
            let ed = graph.edge_definitions().get(0).unwrap();
            assert_eq!(ed.collection(), "test_edge");
            assert_eq!(ed.to().len(), 1);
            assert_eq!(ed.from().len(), 1);
        }
    }
    Ok(())
}

fn ve(vec: Vec<&str>) -> Vec<String> {
    vec.into_iter().map(str::to_string).collect()
}

fn edge_definition() -> Result<Vec<EdgeDefinition>> {
    let ed = EdgeDefinitionBuilder::default()
        .collection("test_edge")
        .from(ve(vec!["test_coll"]))
        .to(ve(vec!["test_coll"]))
        .build()?;
    Ok(vec![ed])
}

#[tokio::test]
async fn graph_create_delete() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let graph_meta = GraphMetaBuilder::default()
        .name(rand_name())
        .edge_definitions(edge_definition()?)
        .build()?;
    let config = CreateConfigBuilder::default().graph(graph_meta).build()?;
    let res: ArangoEither<GraphMeta> = conn.create(config).await?;
    assert!(res.is_right());
    let create = res.right_safe()?;
    assert!(!create.error());
    let graph_meta = create.graph();
    let name = graph_meta.name();

    let delete_config = DeleteConfigBuilder::default()
        .name(name)
        .drop_collections(true)
        .build()?;
    let res: ArangoEither<()> = conn.delete(delete_config).await?;
    assert!(res.is_right());
    Ok(())
}

#[tokio::test]
async fn graph_read() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let config = ReadConfigBuilder::default().name("test_graph").build()?;
    let res: ArangoEither<GraphMeta> = conn.read(config).await?;
    assert!(res.is_right());
    let graph_meta = res.right_safe()?;
    assert!(!graph_meta.error());
    assert_eq!(*graph_meta.code(), 200);
    let graph = graph_meta.graph();

    assert!(!graph.id().is_empty());
    assert!(!graph.key().is_empty());
    assert!(!graph.rev().is_empty());
    assert_eq!(graph.name(), "test_graph");
    assert_eq!(graph.orphan_collections().len(), 0);
    assert_eq!(graph.edge_definitions().len(), 1);
    let ed = graph.edge_definitions().get(0).unwrap();
    assert_eq!(ed.collection(), "test_edge");
    assert_eq!(ed.to().len(), 1);
    assert_eq!(ed.from().len(), 1);

    Ok(())
}

#[tokio::test]
async fn graph_list_edges() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let config = ListEdgesConfigBuilder::default()
        .name("test_graph")
        .build()?;
    let res: ArangoEither<EdgesMeta> = conn.list_edges(config).await?;
    assert!(res.is_right());
    let graph_meta = res.right_safe()?;
    assert!(!graph_meta.error());
    assert_eq!(*graph_meta.code(), 200);
    assert!(graph_meta.collections().len() >= 1);

    Ok(())
}

#[tokio::test]
async fn graph_create_delete_edge() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let from_to = FromToBuilder::default()
        .from("test_coll/1637032")
        .to("test_coll/1637052")
        .build()?;
    let config = EdgeCreateConfigBuilder::default()
        .graph("test_graph")
        .collection("test_edge")
        .mapping(from_to)
        .return_new(true)
        .build()?;
    let res: ArangoEither<CreateEdge> = conn.create_edge(config).await?;
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
        .graph("test_graph")
        .collection("test_edge")
        .key(key)
        .build()?;
    let res: ArangoEither<DeleteEdge> = conn.delete_edge(delete_config).await?;
    assert!(res.is_right());
    let delete_edge = res.right_safe()?;
    assert!(!delete_edge.error());
    assert_eq!(*delete_edge.code(), 202);
    assert!(delete_edge.removed());
    assert!(delete_edge.old().is_none());
    Ok(())
}

#[tokio::test]
async fn graph_create_get_delete_edge() -> Result<()> {
    let conn = &*RUARANGO_POOL.get()?;
    let from_to = FromToBuilder::default()
        .from("test_coll/1637032")
        .to("test_coll/1637052")
        .build()?;
    let config = EdgeCreateConfigBuilder::default()
        .graph("test_graph")
        .collection("test_edge")
        .mapping(from_to)
        .return_new(true)
        .build()?;
    let res: ArangoEither<CreateEdge> = conn.create_edge(config).await?;
    assert!(res.is_right());
    let create_edge = res.right_safe()?;
    assert!(!create_edge.error());
    assert_eq!(*create_edge.code(), 202);
    let edge = create_edge.edge();
    let key = edge.key();

    let read_config = EdgeReadConfigBuilder::default()
        .graph("test_graph")
        .collection("test_edge")
        .key(key)
        .build()?;
    let res: ArangoEither<ReadEdge> = conn.read_edge(read_config).await?;
    assert!(res.is_right());
    let read_edge = res.right_safe()?;
    assert!(!read_edge.error());

    let delete_config = EdgeDeleteConfigBuilder::default()
        .graph("test_graph")
        .collection("test_edge")
        .key(key)
        .build()?;
    let res: ArangoEither<DeleteEdge> = conn.delete_edge(delete_config).await?;
    assert!(res.is_right());
    let delete_edge = res.right_safe()?;
    assert!(!delete_edge.error());
    assert_eq!(*delete_edge.code(), 202);
    Ok(())
}
