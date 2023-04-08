/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

#![no_std]
extern crate alloc;
extern crate lazy_static;
extern crate log;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(all(not(target_arch = "wasm32"), not(test)))]
extern crate libc_print;

pub(crate) mod compiler;
pub(crate) mod config;
pub(crate) mod constants;
pub(crate) mod formatter;
pub(crate) mod logger;
pub(crate) mod session;
pub(crate) mod smartcalc;
pub(crate) mod syntax;
pub(crate) mod token;
pub(crate) mod tokinizer;
pub(crate) mod tools;
pub(crate) mod types;
pub(crate) mod variable;

#[cfg(test)]
mod tests;

pub use compiler::{money, DataItem};
pub use config::SmartCalcConfig;
pub use session::Session;
pub use smartcalc::RuleTrait;
pub use smartcalc::SmartCalc;
pub use token::ui_token::UiToken;
pub use token::ui_token::UiTokenType;
pub use types::FieldType;
pub use types::NumberType;
pub use types::SmartCalcAstType;
pub use types::TimeOffset;
pub use types::TokenType;
