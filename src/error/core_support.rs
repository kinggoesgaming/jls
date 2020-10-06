use crate::error;
use nom::error as nom_error;;

impl<T> From<nom_error::ParseError<T>> for error::Error {
    fn from(_: nom_error::ParseError<T>) -> Self {
        todo!()
    }
}