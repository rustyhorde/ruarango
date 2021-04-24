use anyhow::Result;
use ruarango::{AsyncKind, Connection, ConnectionBuilder};

#[derive(PartialEq)]
pub enum ConnKind {
    Ruarango,
    RuarangoAsync,
    Root,
    RootAsync,
}

impl ConnKind {
    fn username(&self) -> &'static str {
        match self {
            Self::Ruarango | Self::RuarangoAsync => "ruarango",
            Self::Root | Self::RootAsync => "root",
        }
    }

    fn password(&self) -> &'static str {
        match self {
            Self::Ruarango | Self::RuarangoAsync => env!("ARANGODB_RUARANGO_PASSWORD"),
            Self::Root | Self::RootAsync => env!("ARANGODB_ROOT_PASSWORD"),
        }
    }

    fn database(&self) -> Option<&'static str> {
        match self {
            Self::Ruarango | Self::RuarangoAsync => Some("ruarango"),
            Self::Root | Self::RootAsync => None,
        }
    }

    fn is_async(&self) -> bool {
        *self == Self::RuarangoAsync || *self == Self::RootAsync
    }
}

pub async fn conn(kind: ConnKind) -> Result<Connection> {
    let mut cb = ConnectionBuilder::default()
        .url(env!("ARANGODB_URL"))
        .username(kind.username())
        .password(kind.password());

    if let Some(db) = kind.database() {
        cb = cb.database(db);
    }

    if kind.is_async() {
        cb = cb.async_kind(AsyncKind::Store);
    }

    cb.build().await
}
