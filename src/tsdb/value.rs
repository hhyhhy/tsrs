#[derive(PartialEq, Debug)]
enum ValueType {
    F64,
    I64,
    U64,
    Bool,
    String,
}

struct Value<T> {
    pub unix_nano: i128,
    pub value: T,
}

impl<T> Value<T> {
    pub fn new(unix_nano: i128, value: T) -> Self {
        Self { unix_nano, value }
    }
}

impl<T> Default for Value<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            unix_nano: i128::default(),
            value: T::default(),
        }
    }
}

struct Values<T> {
    inner: Vec<Value<T>>,
}

impl<T> Values<T> {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn push(&mut self, v: Value<T>) {
        self.inner.push(v);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Default for Values<T> {
    fn default() -> Self {
        Values::new()
    }
}

trait TypedValues {
    fn value_type(&self) -> ValueType;
}

type F64Values = Values<f64>;
type I64Values = Values<i64>;
type U64Values = Values<u64>;
type BoolValues = Values<bool>;
type StringValues = Values<String>;

impl TypedValues for F64Values {
    fn value_type(&self) -> ValueType {
        ValueType::F64
    }
}

impl TypedValues for I64Values {
    fn value_type(&self) -> ValueType {
        ValueType::I64
    }
}

impl TypedValues for U64Values {
    fn value_type(&self) -> ValueType {
        ValueType::U64
    }
}

impl TypedValues for BoolValues {
    fn value_type(&self) -> ValueType {
        ValueType::Bool
    }
}

impl TypedValues for StringValues {
    fn value_type(&self) -> ValueType {
        ValueType::String
    }
}

#[cfg(test)]
mod tests {
    use crate::tsdb::value::{
        BoolValues, F64Values, I64Values, StringValues, TypedValues, U64Values, Value, ValueType,
    };

    #[test]
    fn test_push() {
        let mut values = F64Values::default();
        values.push(Value::new(100, 10.1));
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn test_type() {
        assert_eq!(F64Values::default().value_type(), ValueType::F64);
        assert_eq!(I64Values::default().value_type(), ValueType::I64);
        assert_eq!(U64Values::default().value_type(), ValueType::U64);
        assert_eq!(BoolValues::default().value_type(), ValueType::Bool);
        assert_eq!(StringValues::default().value_type(), ValueType::String);
    }
}
