// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Output Structs

use getset::Getters;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
#[cfg(test)]
use {
    crate::{error::RuarangoErr::InvalidMock, utils::mocks::Mock},
    anyhow::Result,
};

/// Document metadata output
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct DocMeta<N, O> {
    /// Contains the document key
    #[serde(rename = "_key")]
    key: String,
    /// Contains the document identifier of the newly created document
    #[serde(rename = "_id")]
    id: String,
    /// Contains the document revision
    #[serde(rename = "_rev")]
    rev: String,
    /// Contains the old document revision, for some `overwrite`s
    #[serde(rename = "_oldRev", skip_serializing_if = "Option::is_none")]
    old_rev: Option<String>,
    /// Contains the new document, if `return_new` was enabled
    #[serde(rename = "new", skip_serializing_if = "Option::is_none")]
    new_doc: Option<N>,
    /// Contains the old document, if `return_old` was enabled, and the
    /// [`overwrite`](crate::doc::input::OverwriteMode) mode supports it.
    #[serde(rename = "old", skip_serializing_if = "Option::is_none")]
    old_doc: Option<O>,
}

#[cfg(test)]
impl Default for DocMeta<(), ()> {
    fn default() -> Self {
        Self {
            key: "abc".to_string(),
            id: "def".to_string(),
            rev: "ghi".to_string(),
            old_rev: None,
            new_doc: None,
            old_doc: None,
        }
    }
}

#[cfg(test)]
impl Default for DocMeta<OutputDoc, ()> {
    fn default() -> Self {
        Self {
            key: "abc".to_string(),
            id: "def".to_string(),
            rev: "ghi".to_string(),
            old_rev: None,
            new_doc: Some(OutputDoc::default()),
            old_doc: None,
        }
    }
}

#[cfg(test)]
impl Default for DocMeta<OutputDoc, OutputDoc> {
    fn default() -> Self {
        Self {
            key: "abc".to_string(),
            id: "def".to_string(),
            rev: "ghi".to_string(),
            old_rev: Some("ghi".to_string()),
            new_doc: Some(OutputDoc::default()),
            old_doc: Some(OutputDoc::default()),
        }
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub(crate) struct OutputDoc {
    #[serde(rename = "_key")]
    key: String,
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_rev")]
    rev: String,
    test: String,
}

#[cfg(test)]
impl Default for OutputDoc {
    fn default() -> Self {
        Self {
            key: "abc".to_string(),
            id: "def".to_string(),
            rev: "ghi".to_string(),
            test: "test".to_string(),
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CreateMockKind {
    FirstCreate,
    SecondCreate,
    NewDoc,
    NewOldDoc,
}

#[cfg(test)]
impl Mock<CreateMockKind> for DocMeta<(), ()> {
    fn try_mock(name: CreateMockKind) -> Result<Self> {
        match name {
            CreateMockKind::FirstCreate => {
                let mut create = Self::default();
                create.key = "test_key".to_string();
                Ok(create)
            }
            CreateMockKind::SecondCreate => {
                let mut create = Self::default();
                create.key = "test_key".to_string();
                create.old_rev = Some("ghi".to_string());
                Ok(create)
            }
            _ => Err(InvalidMock.into()),
        }
    }
}

#[cfg(test)]
impl Mock<CreateMockKind> for DocMeta<OutputDoc, ()> {
    fn try_mock(name: CreateMockKind) -> Result<Self> {
        match name {
            CreateMockKind::NewDoc => Ok(Self::default()),
            _ => Err(InvalidMock.into()),
        }
    }
}

#[cfg(test)]
impl Mock<CreateMockKind> for DocMeta<OutputDoc, OutputDoc> {
    fn try_mock(name: CreateMockKind) -> Result<Self> {
        match name {
            CreateMockKind::NewOldDoc => {
                let mut create = Self::default();
                create.key = "test_key".to_string();
                Ok(create)
            }
            _ => Err(InvalidMock.into()),
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReadMockKind {
    Found,
}

#[cfg(test)]
impl Mock<ReadMockKind> for OutputDoc {
    fn try_mock(name: ReadMockKind) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(match name {
            ReadMockKind::Found => OutputDoc::default(),
        })
    }
}

/// Output on a precondition failure for some endpoints
#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
#[getset(get = "pub")]
pub struct DocErr {
    /// Is this an error?
    error: bool,
    /// The error code
    code: u16,
    /// The ArangoDB code
    #[serde(rename = "errorNum")]
    error_num: usize,
    /// The error message
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
    /// Contains the document key
    #[serde(rename = "_key", skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    /// Contains the document identifier of the newly created document
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// Contains the document revision
    #[serde(rename = "_rev", skip_serializing_if = "Option::is_none")]
    rev: Option<String>,
}

impl fmt::Display for DocErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.error)?;
        write!(f, ", code: {}", self.code)?;
        write!(f, ", error_num: {}", self.error_num)?;
        if let Some(error_message) = &self.error_message {
            write!(f, ", error_message: {}", error_message)?;
        }
        if let Some(key) = &self.key {
            write!(f, ", key: {}", key)?;
        }
        if let Some(id) = &self.id {
            write!(f, ", id: {}", id)?;
        }
        if let Some(rev) = &self.rev {
            write!(f, ", rev: {}", rev)?;
        }
        Ok(())
    }
}
