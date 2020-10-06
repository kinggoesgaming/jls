//! Possible errors that can occur while parsing a java source.

mod core_support;

/// Indicate the starting of an error while parsing.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Start(usize);

/// Indicate the ending of an error while parsing.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct End(usize);

/// A parsing error
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Error {
    InvalidUnicode {
        start: Start,
        End: End,
    },
}