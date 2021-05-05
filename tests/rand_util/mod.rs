use anyhow::Result;
use getset::Getters;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use ruarango::{
    coll::{input::ConfigBuilder, output::Create},
    doc::{
        input::{
            CreateConfigBuilder as DocCreateConfigBuilder,
            DeleteConfigBuilder as DocDeleteConfigBuilder,
        },
        output::DocMeta,
    },
    graph::{
        input::{CreateConfigBuilder, DeleteConfigBuilder, GraphMetaBuilder},
        EdgeDefinition, EdgeDefinitionBuilder,
    },
    ArangoEither, Collection, Connection, Document, Graph,
};
use serde::Serialize;
use std::iter;

pub fn rand_name() -> String {
    // Setup a random name so CI testing won't cause collisions
    let mut rng = thread_rng();
    let mut name = String::from("ruarango-");
    let name_ext: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect();
    name.push_str(&name_ext);
    name
}

pub async fn create_random_document<T>(
    conn: &Connection,
    collection: &str,
    document: T,
) -> Result<DocMeta<(), ()>>
where
    T: Clone + Serialize + Send + Sync,
{
    let create_config = DocCreateConfigBuilder::default()
        .collection(collection)
        .document(document)
        .build()?;
    let create_res: ArangoEither<DocMeta<(), ()>> = Document::create(conn, create_config).await?;
    assert!(create_res.is_right());
    Ok(create_res.right_safe()?)
}

#[allow(dead_code)]
pub async fn delete_random_document(conn: &Connection, collection: &str, key: &str) -> Result<()> {
    let delete_config = DocDeleteConfigBuilder::default()
        .collection(collection)
        .key(key)
        .build()?;
    let delete_res: ArangoEither<DocMeta<(), ()>> = Document::delete(conn, delete_config).await?;
    assert!(delete_res.is_right());
    Ok(())
}

pub enum CollKind {
    Document,
    Edge,
}

pub async fn create_random_collection(
    conn: &Connection,
    coll_kind: CollKind,
) -> Result<(String, Create)> {
    let kind = match coll_kind {
        CollKind::Document => 2,
        CollKind::Edge => 3,
    };
    let config = ConfigBuilder::default()
        .name(rand_name())
        .kind(kind)
        .build()?;
    let res = Collection::create(conn, &config).await?;
    assert!(res.is_right());
    let create = res.right_safe()?;
    Ok((create.name().clone(), create))
}

pub async fn delete_random_collection<T>(conn: &Connection, name: T) -> Result<()>
where
    T: Into<String>,
{
    let res = conn.drop(&name.into(), false).await?;
    assert!(res.is_right());
    let dropped = res.right_safe()?;
    assert!(!dropped.error());
    Ok(())
}

pub struct EdgeDefinitionMeta {
    edge_coll: String,
    from_coll: String,
    to_coll: String,
}

async fn edge_definition(conn: &Connection) -> Result<(EdgeDefinitionMeta, Vec<EdgeDefinition>)> {
    let (edge_coll, _) = create_random_collection(conn, CollKind::Edge).await?;
    let (from_coll, _) = create_random_collection(conn, CollKind::Document).await?;
    let (to_coll, _) = create_random_collection(conn, CollKind::Document).await?;

    let ed = EdgeDefinitionBuilder::default()
        .collection(edge_coll.clone())
        .from(vec![from_coll.clone()])
        .to(vec![to_coll.clone()])
        .build()?;
    Ok((
        EdgeDefinitionMeta {
            edge_coll,
            from_coll,
            to_coll,
        },
        vec![ed],
    ))
}

#[derive(Getters)]
#[getset(get = "pub(crate)")]
pub struct GraphMeta {
    graph: String,
    edge_coll: String,
    from_coll: String,
    to_coll: String,
}

pub async fn create_random_graph(conn: &Connection) -> Result<GraphMeta> {
    let (edm, edge_defs) = edge_definition(&conn).await?;
    let graph_meta = GraphMetaBuilder::default()
        .name(rand_name())
        .edge_definitions(edge_defs)
        .build()?;
    let config = CreateConfigBuilder::default().graph(graph_meta).build()?;
    let res = Graph::create(conn, config).await?;
    assert!(res.is_right());
    let create = res.right_safe()?;
    assert!(!create.error());
    let graph_meta = create.graph();
    Ok(GraphMeta {
        graph: graph_meta.name().clone(),
        edge_coll: edm.edge_coll,
        from_coll: edm.from_coll,
        to_coll: edm.to_coll,
    })
}

pub async fn delete_random_graph(conn: &Connection, graph_meta: GraphMeta) -> Result<()> {
    let GraphMeta {
        graph,
        edge_coll,
        from_coll,
        to_coll,
    } = graph_meta;

    let _ = delete_random_collection(&conn, to_coll).await?;
    let _ = delete_random_collection(&conn, from_coll).await?;
    let _ = delete_random_collection(&conn, edge_coll).await?;

    let delete_config = DeleteConfigBuilder::default().name(graph).build()?;
    let res = Graph::delete(conn, delete_config).await?;
    assert!(res.is_right());
    Ok(())
}
