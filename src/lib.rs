//! Java Language Specification compliant parser, written in Rust

#![forbid(
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_doc_code_examples,
    missing_docs,
)]

/// The parser signature.
pub type ParserFn<'input, Input, Output> = fn(&'input Input) -> Result<(&'input Input, Output), ()>;