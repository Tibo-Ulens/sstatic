//!  _____         _____  _          _
//! /  ___|       /  ___|| |        | |
//! \ `--.  ______\ `--. | |_  __ _ | |_
//!  `--. \|______|`--. \| __|/ _` || __|
//! /\__/ /       /\__/ /| |_| (_| || |_
//! \____/        \____/  \__|\__,_| \__|
//!
//! # S-Stat Markup Language Transpiler
//!

pub mod ast;

mod error;

pub use error::Error;

/// Transpile the given source from S-Stat to HTML
///
/// # Examples
///
/// ```rust
/// # use sstat::*;
/// #
/// # fn main() -> Result<(), Error> {
/// #
/// let example_input = "
/// (doc
///   (sec
///     (title cool title)
///     (p cool paragraph)))
/// ";
/// let example_filename = "foo.sstat";
///
/// let ast_root = transpile(&example_input, &example_filename)?;
/// #
/// # Ok(())
/// # }
/// ```
pub fn transpile(_input: &str, _filename: &str) -> Result<(), Error> {
    Ok(())
}
