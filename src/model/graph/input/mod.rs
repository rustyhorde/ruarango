// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Input Structs

mod create;
mod delete;
mod read;

pub use create::{
    Config as CreateConfig, ConfigBuilder as CreateConfigBuilder,
    ConfigBuilderError as CreateConfigBuilderError, GraphMeta, GraphMetaBuilder,
    GraphMetaBuilderError,
};
pub use delete::{
    Config as DeleteConfig, ConfigBuilder as DeleteConfigBuilder,
    ConfigBuilderError as DeleteConfigBuilderError,
};
pub use read::{
    Config as ReadConfig, ConfigBuilder as ReadConfigBuilder,
    ConfigBuilderError as ReadConfigBuilderError,
};
