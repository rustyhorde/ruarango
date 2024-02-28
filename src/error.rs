// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` error

use crate::model::{doc::output::DocErr, BaseErr};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::error::Error;
#[cfg(test)]
use std::num::ParseIntError;

/// When bad things happen
#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
#[allow(variant_size_differences)]
pub enum RuarangoErr {
    /// Invalid body error
    #[error("{}\nInvalid Body: {}", err, body)]
    InvalidBody {
        /// error
        err: String,
        /// body
        body: String,
    },
    /// Unreachable
    #[error("Unreachable: {}", msg)]
    Unreachable {
        /// message
        msg: String,
    },
    /// Invalid connection url
    #[error("You have supplied an invalid connection url")]
    InvalidConnectionUrl,
    /// invalid document response
    #[error("Invalid document response: {}\n{}", status, doc_err(err))]
    InvalidDocResponse {
        /// status
        status: u16,
        /// error
        err: Option<DocErr>,
    },
    /// Invalid cursor response
    #[error("Invalid cursor response: {}", status)]
    InvalidCursorResponse {
        /// status
        status: u16,
    },
    /// Un-authorized
    #[error("You are not authorized to perform the request action")]
    Forbidden {
        /// error
        err: Option<DocErr>,
    },
    /// The request resource is not found
    #[error("The server can not find the requested resource.")]
    NotFound {
        /// error
        err: Option<DocErr>,
    },
    /// Unmodified document
    #[error("The document you requested has not been modified")]
    NotModified,
    /// A precondition has failed
    #[error("A precondition has failed: '{}'", doc_err(err))]
    PreconditionFailed {
        /// error
        err: Option<DocErr>,
    },
    /// A bad request was made
    #[error(
        "The server could not understand the request due to invalid syntax.: '{}'",
        doc_err(err)
    )]
    BadRequest {
        /// error
        err: Option<DocErr>,
    },
    /// A conflict has occurred
    #[error("A conflict has occurred: '{}'", doc_err(err))]
    Conflict {
        /// error
        err: Option<DocErr>,
    },
    /// cursor request error
    #[error("A cursor request error has occurred: {}", base_err(err))]
    Cursor {
        /// Error
        err: Option<BaseErr>,
    },
    #[cfg(test)]
    #[error("Unable to parse the given value")]
    ParseInt(#[from] ParseIntError),
    #[cfg(test)]
    #[error("A test error has occurred: {}", val)]
    TestError { val: String },
    #[cfg(test)]
    #[error("You have requested an invalid mock")]
    InvalidMock,
}

impl Serialize for RuarangoErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RuarangoErr", 2)?;
        state.serialize_field("reason", &format!("{self}"))?;
        if let Some(source) = self.source() {
            state.serialize_field("source", &format!("{source}"))?;
        }
        state.end()
    }
}

fn doc_err(err: &Option<DocErr>) -> String {
    err.as_ref().map_or_else(
        || "No matching document found".to_string(),
        ToString::to_string,
    )
}

fn base_err(err: &Option<BaseErr>) -> String {
    err.as_ref()
        .map_or_else(|| "cursor error".to_string(), ToString::to_string)
}

#[cfg(test)]
impl From<&str> for RuarangoErr {
    fn from(val: &str) -> Self {
        Self::TestError {
            val: val.to_string(),
        }
    }
}

#[cfg(test)]
impl From<String> for RuarangoErr {
    fn from(val: String) -> Self {
        Self::TestError { val }
    }
}

#[cfg(test)]
mod test {
    use super::RuarangoErr::{self, TestError};
    use anyhow::Result;

    #[test]
    fn serialize_with_source_works() -> Result<()> {
        match str::parse::<usize>("test") {
            Ok(_) => panic!("this shouldn't happen"),
            Err(e) => {
                let err: RuarangoErr = e.into();
                let result = serde_json::to_string(&err)?;
                assert_eq!("{\"reason\":\"Unable to parse the given value\",\"source\":\"invalid digit found in string\"}", result);
            }
        }

        Ok(())
    }

    #[test]
    fn serialize_no_source_works() -> Result<()> {
        let err: RuarangoErr = TestError {
            val: "test".to_string(),
        };
        let result = serde_json::to_string(&err)?;
        assert_eq!("{\"reason\":\"A test error has occurred: test\"}", result);
        Ok(())
    }

    #[test]
    fn from_str_works() -> Result<()> {
        let err: RuarangoErr = "test".into();
        let result = serde_json::to_string(&err)?;
        assert_eq!("{\"reason\":\"A test error has occurred: test\"}", result);
        Ok(())
    }

    #[test]
    fn from_string_works() -> Result<()> {
        let err: RuarangoErr = String::from("test").into();
        let result = serde_json::to_string(&err)?;
        assert_eq!("{\"reason\":\"A test error has occurred: test\"}", result);
        Ok(())
    }
}
