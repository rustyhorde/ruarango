// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [`Input`](crate::coll::input)/[`Output`](crate::coll::output) for [`Collection`](crate::Collection) operations

use serde::de::{self, Deserialize as Deser, Deserializer, Visitor};
#[cfg(test)]
use serde::ser::{Serialize as Ser, Serializer};
use std::fmt;

pub mod input;
pub mod output;

/// The collection kind
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollectionKind {
    /// A document collection
    Document,
    /// An edges collection
    Edges,
}

#[cfg(test)]
impl Ser for CollectionKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CollectionKind::Document => serializer.serialize_u64(2),
            CollectionKind::Edges => serializer.serialize_u64(3),
        }
    }
}

impl<'de> Deser<'de> for CollectionKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(CollectionKindVisitor)
    }
}

struct CollectionKindVisitor;

impl Visitor<'_> for CollectionKindVisitor {
    type Value = CollectionKind;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            2 => Ok(CollectionKind::Document),
            3 => Ok(CollectionKind::Edges),
            _ => Err(E::custom("Invalid collection kind")),
        }
    }
}

/// The collection status
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    /// unknown - may be corrupted
    Unknown,
    /// unloaded
    Unloaded,
    /// loaded
    Loaded,
    /// unloading
    Unloading,
    /// deleted
    Deleted,
    /// loading
    Loading,
}

#[cfg(test)]
impl Ser for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Status::Unknown => serializer.serialize_u64(0),
            Status::Unloaded => serializer.serialize_u64(2),
            Status::Loaded => serializer.serialize_u64(3),
            Status::Unloading => serializer.serialize_u64(4),
            Status::Deleted => serializer.serialize_u64(5),
            Status::Loading => serializer.serialize_u64(6),
        }
    }
}

impl<'de> Deser<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(StatusVisitor)
    }
}

struct StatusVisitor;

impl Visitor<'_> for StatusVisitor {
    type Value = Status;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            2 => Ok(Status::Unloaded),
            3 => Ok(Status::Loaded),
            4 => Ok(Status::Unloading),
            5 => Ok(Status::Deleted),
            6 => Ok(Status::Loading),
            _ => Ok(Status::Unknown),
        }
    }
}
