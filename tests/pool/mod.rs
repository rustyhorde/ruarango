use lazy_static::lazy_static;
use r2d2::{ManageConnection, Pool};
use ruarango::{Connection, Error};
use tokio::runtime::Runtime;

use crate::conn::{conn, ConnKind};

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    pub(crate) static ref RUARANGO_POOL: Pool<RuarangoPool> = {
        let manager = RuarangoPool {
            kind: ConnKind::Ruarango,
        };
        Pool::builder().max_size(15).build(manager).unwrap()
    };
    pub(crate) static ref RUARANGO_ASYNC_POOL: Pool<RuarangoPool> = {
        let manager = RuarangoPool {
            kind: ConnKind::RuarangoAsync,
        };
        Pool::builder().max_size(15).build(manager).unwrap()
    };
    pub(crate) static ref ROOT_POOL: Pool<RuarangoPool> = {
        let manager = RuarangoPool {
            kind: ConnKind::Root,
        };
        Pool::builder().max_size(15).build(manager).unwrap()
    };
    pub(crate) static ref ROOT_ASYNC_POOL: Pool<RuarangoPool> = {
        let manager = RuarangoPool {
            kind: ConnKind::RootAsync,
        };
        Pool::builder().max_size(15).build(manager).unwrap()
    };
}

pub(crate) struct RuarangoPool {
    kind: ConnKind,
}

impl ManageConnection for RuarangoPool {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        (*RUNTIME)
            .block_on(conn(self.kind))
            .map_err(|_e| Error::NotModified)
    }

    fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}
