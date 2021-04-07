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
#[cfg(test)]
use {
    crate::{error::RuarangoError::InvalidMock, utils::mocks::Mock},
    anyhow::Result,
};

/// Output when [`create`](crate::Document::create) is called for a document
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct Create<T, U> {
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
    new_doc: Option<T>,
    /// Contains the old document, if `return_old` was enabled, and the
    /// [`overwrite`](crate::doc::input::OverwriteMode) mode supports it.
    #[serde(rename = "old", skip_serializing_if = "Option::is_none")]
    old_doc: Option<U>,
}

#[cfg(test)]
impl Default for Create<(), ()> {
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
impl Default for Create<OutputDoc, ()> {
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
impl Default for Create<OutputDoc, OutputDoc> {
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
impl Mock<CreateMockKind> for Create<(), ()> {
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
impl Mock<CreateMockKind> for Create<OutputDoc, ()> {
    fn try_mock(name: CreateMockKind) -> Result<Self> {
        match name {
            CreateMockKind::NewDoc => Ok(Self::default()),
            _ => Err(InvalidMock.into()),
        }
    }
}

#[cfg(test)]
impl Mock<CreateMockKind> for Create<OutputDoc, OutputDoc> {
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
