//!  _____         _____  _          _
//! /  ___|       /  ___|| |        | |
//! \ `--.  ______\ `--. | |_  __ _ | |_
//!  `--. \|______|`--. \| __|/ _` || __|
//! /\__/ /       /\__/ /| |_| (_| || |_
//! \____/        \____/  \__|\__,_| \__|
//!
//! # S-Stat Markup Language Transpiler
//!

#![warn(missing_docs)]
#![feature(assert_matches)]
#![feature(let_chains)]
#![feature(type_alias_impl_trait)]

use std::path::Path;

mod error;
pub mod parse;

#[cfg(test)]
mod test;

pub use error::*;
use parse::{FileInfo, Parser};

/// Transpile the given source from S-Stat to HTML
pub fn transpile(file_name: &str, file_path: &Path, file_src: &str) -> Result<(), Error> {
    let file_info = FileInfo::new(file_name, file_path, file_src);
    file_info.init_thread_local();

    let _ = Parser::parse_page(file_src);

    todo!()
}
