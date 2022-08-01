use std::fmt::{Display, Formatter};

mod cache;
pub mod point;
mod value;

type Result<T> = std::result::Result<T, TypeMismatchError>;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ValueType {
    F64,
    I64,
    U64,
    Bool,
    String,
}

#[derive(Debug, PartialEq)]
pub struct TypeMismatchError {
    expect: ValueType,
    got: ValueType,
}

impl TypeMismatchError {
    pub(crate) fn new(expect: ValueType, got: ValueType) -> Self {
        Self { expect, got }
    }
}

impl Display for TypeMismatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "type mismatch expect: {:?}, got: {:?}",
            self.expect, self.got
        )
    }
}

impl std::error::Error for TypeMismatchError {}
