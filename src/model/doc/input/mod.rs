// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Input Structs

mod create;
mod creates;
mod delete;
mod deletes;
mod read;
mod reads;
mod replace;
mod replaces;
mod update;
mod updates;

pub use create::{
    Config as CreateConfig, ConfigBuilder as CreateConfigBuilder,
    ConfigBuilderError as CreateConfigBuilderError,
};
pub use creates::{
    Config as CreatesConfig, ConfigBuilder as CreatesConfigBuilder,
    ConfigBuilderError as CreatesConfigBuilderError,
};
pub use delete::{
    Config as DeleteConfig, ConfigBuilder as DeleteConfigBuilder,
    ConfigBuilderError as DeleteConfigBuilderError,
};
pub use deletes::{
    Config as DeletesConfig, ConfigBuilder as DeletesConfigBuilder,
    ConfigBuilderError as DeletesConfigBuilderError,
};
pub use read::{
    Config as ReadConfig, ConfigBuilder as ReadConfigBuilder,
    ConfigBuilderError as ReadConfigBuilderError,
};
pub use reads::{
    Config as ReadsConfig, ConfigBuilder as ReadsConfigBuilder,
    ConfigBuilderError as ReadsConfigBuilderError,
};
pub use replace::{
    Config as ReplaceConfig, ConfigBuilder as ReplaceConfigBuilder,
    ConfigBuilderError as ReplaceConfigBuilderError,
};
pub use replaces::{
    Config as ReplacesConfig, ConfigBuilder as ReplacesConfigBuilder,
    ConfigBuilderError as ReplacesConfigBuilderError,
};
pub use update::{
    Config as UpdateConfig, ConfigBuilder as UpdateConfigBuilder,
    ConfigBuilderError as UpdateConfigBuilderError,
};
pub use updates::{
    Config as UpdatesConfig, ConfigBuilder as UpdatesConfigBuilder,
    ConfigBuilderError as UpdatesConfigBuilderError,
};

use anyhow::Result;
use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt;

/// Overwrite Modes
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OverwriteMode {
    /// If a document with the specified `_key` value exists already,
    /// nothing will be done and no write operation will be carried out. The
    /// insert operation will return success in this case. This mode does not
    /// support returning the old document version using `return_old`. When using
    /// `return_new`, `None` will be returned in case the document already existed.
    Ignore,
    /// If a document with the specified `_key` value exists already,
    /// it will be patched (partially updated) with the specified document value.
    /// The `overwrite_mode` can be further controlled via the `keep_null` and
    /// `merge_objects` configuration.
    Update,
    /// If a document with the specified `_key` value exists already,
    /// it will be overwritten with the specified document value. This mode will
    /// also be used when no `overwrite_mode` is specified but the `overwrite`
    /// flag is set to true.
    Replace,
    /// If a document with the specified `_key` value exists already,
    /// return a unique constraint violation error so that the insert operation
    /// fails. This is also the default behavior in case the `overwrite_mode` is
    /// not set, and the `overwrite` flag is false or not set either.
    Conflict,
}

impl fmt::Display for OverwriteMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Ignore => "ignore",
                Self::Update => "update",
                Self::Replace => "replace",
                Self::Conflict => "conflict",
            }
        )
    }
}

impl Serialize for OverwriteMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OverwriteMode::Ignore => serializer.serialize_str("ignore"),
            OverwriteMode::Update => serializer.serialize_str("update"),
            OverwriteMode::Replace => serializer.serialize_str("replace"),
            OverwriteMode::Conflict => serializer.serialize_str("conflict"),
        }
    }
}

impl<'de> Deserialize<'de> for OverwriteMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OverwriteModeVisitor)
    }
}

/// # Example
///
/// ```
/// # use ruarango::doc::input::OverwriteMode;
/// let mode: String = OverwriteMode::Ignore.into();
/// ```
impl From<OverwriteMode> for String {
    /// # Example
    ///
    /// ```
    /// # use ruarango::doc::input::OverwriteMode;
    /// let mode: String = OverwriteMode::Ignore.into();
    /// ```
    fn from(mode: OverwriteMode) -> String {
        match mode {
            OverwriteMode::Ignore => "ignore",
            OverwriteMode::Conflict => "conflict",
            OverwriteMode::Replace => "replace",
            OverwriteMode::Update => "update",
        }
        .to_string()
    }
}
struct OverwriteModeVisitor;

impl Visitor<'_> for OverwriteModeVisitor {
    type Value = OverwriteMode;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let normalized = value.to_lowercase();
        match &normalized[..] {
            "ignore" => Ok(OverwriteMode::Ignore),
            "replace" => Ok(OverwriteMode::Replace),
            "update" => Ok(OverwriteMode::Update),
            "conflict" => Ok(OverwriteMode::Conflict),
            _ => Err(E::custom("Invalid overwrite mode")),
        }
    }
}
