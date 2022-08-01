use super::Result;
use super::TypeMismatchError;
use super::ValueType;

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

pub struct Row {
    unix_nano: i128,
    value: Value,
}

impl Row {
    pub fn new(unix_nano: i128, value: Value) -> Self {
        Self { unix_nano, value }
    }
}

pub struct Entry {
    value_type: ValueType,
    values: Vec<Value>,
}

impl Entry {
    pub fn new(value_type: ValueType) -> Self {
        let values = Vec::new();
        Self { value_type, values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn push(&mut self, v: Value) -> Result<()> {
        if self.value_type != v.value_type() {
            return Err(TypeMismatchError::new(self.value_type, v.value_type()));
        }

        self.values.push(v);

        Ok(())
    }
}
