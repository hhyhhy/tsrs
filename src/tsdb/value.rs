use tokio::sync::RwLock;

enum ValueType {
    Float,
    Integer,
    Bool,
    String,
}

struct Value<T> {
    unix_nano: i128,
    value: T,
}

struct Values<T> {
    inner: RwLock<Vec<Value<T>>>,
}

impl<T> Values<T> {
    pub async fn push<T>(&mut self, v: Value<T>) {
        self.inner.write().await.push(v);
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
