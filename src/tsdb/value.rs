use tokio::sync::RwLock;

#[derive(PartialEq, Debug)]
enum ValueType {
    Float,
    Integer,
    Bool,
    String,
}

struct Value<T> {
    pub unix_nano: i128,
    pub value: T,
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
    inner: RwLock<Vec<Value<T>>>,
}

impl<T> Values<T> {
    pub async fn push(&mut self, v: Value<T>) {
        self.inner.write().await.push(v);
    }

    pub async fn len(&self) -> usize {
        self.inner.read().await.len()
    }
}

impl<T> Default for Values<T> {
    fn default() -> Self {
        Self {
            inner: RwLock::new(Vec::new()),
        }
    }
}

trait TypedValues {
    fn value_type(&self) -> ValueType;
}

type FloatValues = Values<f64>;
type InterValues = Values<i64>;
type BoolValues = Values<bool>;
type StringValues = Values<String>;

impl TypedValues for FloatValues {
    fn value_type(&self) -> ValueType {
        ValueType::Float
    }
}

impl TypedValues for InterValues {
    fn value_type(&self) -> ValueType {
        ValueType::Integer
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
        BoolValues, FloatValues, InterValues, StringValues, TypedValues, Value, ValueType,
    };
    use std::collections::hash_map::Values;

    #[tokio::test]
    async fn test_push() {
        let mut values = FloatValues::default();
        values
            .push(Value {
                unix_nano: 100,
                value: 10.1,
            })
            .await;
        assert_eq!(values.len().await, 1);
    }

    #[test]
    fn test_type() {
        assert_eq!(FloatValues::default().value_type(), ValueType::Float);
        assert_eq!(InterValues::default().value_type(), ValueType::Integer);
        assert_eq!(BoolValues::default().value_type(), ValueType::Bool);
        assert_eq!(StringValues::default().value_type(), ValueType::String)
    }
}
