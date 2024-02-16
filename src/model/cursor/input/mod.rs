// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Cursor Input Structs

mod create;
mod delete;
mod next;

pub use create::{
    Config as CreateConfig, ConfigBuilder as CreateConfigBuilder,
    ConfigBuilderError as CreateConfigBuilderError, Options, OptionsBuilder, OptionsBuilderError,
    ProfileKind, Rules,
};
pub use delete::{
    Config as DeleteConfig, ConfigBuilder as DeleteConfigBuilder,
    ConfigBuilderError as DeleteConfigBuilderError,
};
pub use next::{
    Config as NextConfig, ConfigBuilder as NextConfigBuilder,
    ConfigBuilderError as NextConfigBuilderError,
};
