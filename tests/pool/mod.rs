use lazy_static::lazy_static;
use r2d2::{ManageConnection, Pool};
use ruarango::{AsyncKind, Connection, ConnectionBuilder, Error};
use tokio::runtime::Runtime;

use crate::conn::ConnKind;

lazy_static! {
    pub(crate) static ref RUARANGO_POOL: Pool<RuarangoPool> = {
        let manager = RuarangoPool {
            kind: ConnKind::Ruarango,
        };
        let pool = Pool::builder().max_size(15).build(manager).unwrap();
        pool
    };
    pub(crate) static ref RUARANGO_ASYNC_POOL: Pool<RuarangoPool> = {
        let manager = RuarangoPool {
            kind: ConnKind::RuarangoAsync,
        };
        let pool = Pool::builder().max_size(15).build(manager).unwrap();
        pool
    };
}

pub(crate) struct RuarangoPool {
    kind: ConnKind,
}

impl ManageConnection for RuarangoPool {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let rt = Runtime::new().unwrap();

        let mut cb = ConnectionBuilder::default()
            .url(env!("ARANGODB_URL"))
            .username(self.kind.username())
            .password(self.kind.password());

        if let Some(db) = self.kind.database() {
            cb = cb.database(db);
        }

        if self.kind.is_async() {
            cb = cb.async_kind(AsyncKind::Store);
        }

        rt.block_on(cb.build()).map_err(|_e| Error::NotModified)
    }

    fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}
