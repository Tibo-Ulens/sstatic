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

use codespan_reporting::files::SimpleFile;

mod error;
pub mod parse;

#[cfg(test)]
mod test;

pub use error::*;
use parse::Parser;

/// Transpile the given source from S-Stat to HTML
pub fn transpile(filename: String, source: String) -> Result<(), Error> {
    let file = SimpleFile::new(filename, source);
    let parser = Parser::new(file);

    parser.parse()?;

    Ok(())
}
