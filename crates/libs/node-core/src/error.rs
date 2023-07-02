use std::borrow::Cow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use anyhow::Error;

/// A simple error type that lets the creator specify both the error message and
/// the error class name. This type is private; externally it only ever appears
/// wrapped in an `anyhow::Error`. To retrieve the error class name from a wrapped
/// `CustomError`, use the function `get_custom_error_class()`.
///
#[derive(Debug)]
struct CustomError {
    class: &'static str,
    message: Cow<'static, str>,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}


pub fn error_from_io_error(error: std::io::Error) -> Error {
    let kind = error.kind();
    generic_error(kind.to_string())
}


impl std::error::Error for CustomError {}

/// If this error was crated with `custom_error()`, return the specified error
/// class name. In all other cases this function returns `None`.
pub fn get_custom_error_class(error: &Error) -> Option<&'static str> {
    error.downcast_ref::<CustomError>().map(|e| e.class)
}

pub fn get_custom_error_message(error: &Error) -> Option<Cow<str>> {
    error.downcast_ref::<CustomError>().map(|e| e.message.clone())
}

pub type AnyError = anyhow::Error;


pub fn custom_error(
    class: &'static str,
    message: impl Into<Cow<'static, str>>,
) -> Error {
    CustomError {
        class,
        message: message.into(),
    }
        .into()
}

pub fn generic_error(message: impl Into<Cow<'static, str>>) -> Error {
    custom_error("Error", message)
}

pub fn type_error(message: impl Into<Cow<'static, str>>) -> Error {
    custom_error("TypeError", message)
}