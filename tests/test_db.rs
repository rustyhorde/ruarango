use anyhow::Result;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use ruarango::{db::input::CreateBuilder, ConnectionBuilder, Database};
use std::iter;

#[tokio::test]
async fn current() -> Result<()> {
    let conn = ConnectionBuilder::default()
        .url(env!("ARANGODB_URL"))
        .username("ruarango")
        .password(env!("ARANGODB_RUARANGO_PASSWORD"))
        .database("ruarango")
        .build()
        .await?;

    let res = conn.current().await?;

    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    let current = res.result();
    assert_eq!(current.name(), "ruarango");
    assert!(!current.is_system());

    Ok(())
}

#[tokio::test]
async fn user() -> Result<()> {
    let conn = ConnectionBuilder::default()
        .url(env!("ARANGODB_URL"))
        .username("ruarango")
        .password(env!("ARANGODB_RUARANGO_PASSWORD"))
        .database("ruarango")
        .build()
        .await?;

    let res = conn.user().await?;

    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert_eq!(res.result().len(), 1);
    assert_eq!(res.result()[0], "ruarango");

    Ok(())
}

#[tokio::test]
async fn list() -> Result<()> {
    let conn = ConnectionBuilder::default()
        .url(env!("ARANGODB_URL"))
        .username("root")
        .password(env!("ARANGODB_ROOT_PASSWORD"))
        .build()
        .await?;

    let res = conn.list().await?;

    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(res.result().len() > 0);
    assert!(res.result().contains(&"ruarango".to_string()));

    Ok(())
}

#[tokio::test]
async fn create_drop() -> Result<()> {
    let conn = ConnectionBuilder::default()
        .url(env!("ARANGODB_URL"))
        .username("root")
        .password(env!("ARANGODB_ROOT_PASSWORD"))
        .build()
        .await?;

    let mut rng = thread_rng();
    let mut db_name = String::from("abc");
    let db_name_ext: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect();
    db_name.push_str(&db_name_ext);
    let create = CreateBuilder::default().name(db_name.clone()).build()?;
    let res = conn.create(&create).await?;

    assert!(!res.error());
    assert_eq!(*res.code(), 201);
    assert!(res.result());

    let res = conn.drop(&db_name).await?;
    assert!(!res.error());
    assert_eq!(*res.code(), 200);
    assert!(res.result());

    Ok(())
}
