//! Java Language Specification compliant parser, written in Rust

#![forbid(
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_doc_code_examples,
    missing_docs,
)]

pub mod error;

/// The parser signature.
pub type ParserFn<'input, Input, Output, Error> = fn(&'input Input) -> Result<(&'input Input, Output), Error>;

/// Get a valid unicode character
pub fn unicode_char<'input, Input>(input: &'input Input) -> Result<(&'input Input, char), Error> 
where
    Input: nom::InputIter
{
    let (rest, output) = nom::character::complete::anychar(input)?;

    Ok((rest, output))
}