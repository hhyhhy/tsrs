use super::Result;
use super::TypeMismatchError;
use super::ValueType;
use crate::tsdb::point::FieldValue;

pub enum Value {
    F64(f64),
    I64(i64),
    U64(u64),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::F64(_) => ValueType::F64,
            Value::I64(_) => ValueType::I64,
            Value::U64(_) => ValueType::U64,
            Value::Bool(_) => ValueType::Bool,
            Value::String(_) => ValueType::String,
        }
    }
}

impl From<&FieldValue> for Value {
    fn from(f: &FieldValue) -> Self {
        match f {
            FieldValue::F64(v) => Value::F64(*v),
            FieldValue::I64(v) => Value::I64(*v),
            FieldValue::U64(v) => Value::U64(*v),
            FieldValue::Bool(v) => Value::Bool(*v),
            FieldValue::String(v) => Value::String(v.clone()),
        }
    }
}

pub struct Row {
    unix_nano: u128,
    value: Value,
}

impl Row {
    pub fn new(unix_nano: u128, value: Value) -> Self {
        Self { unix_nano, value }
    }
}

pub struct Entry {
    value_type: ValueType,
    values: Vec<Row>,
}

impl Entry {
    pub fn new(value_type: ValueType) -> Self {
        let values = Vec::new();
        Self { value_type, values }
    }

    pub fn value_type(&self) -> ValueType {
        self.value_type
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn push(&mut self, r: Row) -> Result<()> {
        if self.value_type != r.value.value_type() {
            let e = TypeMismatchError::new(self.value_type, r.value.value_type());
            return Err(e);
        }

        self.values.push(r);

        Ok(())
    }

    pub fn append(&mut self, other: &mut Self) -> Result<()> {
        if self.value_type != other.value_type {
            return Err(TypeMismatchError::new(self.value_type, other.value_type));
        }

        self.values.append(&mut other.values);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tsdb::value::{Entry, Row, Value};
    use crate::tsdb::{TypeMismatchError, ValueType};

    #[test]
    fn test_entry() {
        use ValueType::{I64, U64};

        let mut entry = Entry::new(I64);
        let r = entry.push(Row::new(100, Value::I64(10)));
        assert_eq!(r, Ok(()));

        assert_eq!(entry.len(), 1);

        let r = entry.push(Row::new(200, Value::U64(20)));
        assert_eq!(r, Err(TypeMismatchError::new(I64, U64)));
    }
}
