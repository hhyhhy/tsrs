use crate::tsdb::ValueType;
use std::collections::BTreeMap;
use std::time;

const KEY_FIELD_SEPARATOR: &str = "#!~#";

#[derive(Clone)]
pub enum FieldValue {
    F64(f64),
    I64(i64),
    U64(u64),
    Bool(bool),
    String(String),
}

impl FieldValue {
    pub fn value_type(&self) -> ValueType {
        match self {
            FieldValue::F64(_) => ValueType::F64,
            FieldValue::I64(_) => ValueType::I64,
            FieldValue::U64(_) => ValueType::U64,
            FieldValue::Bool(_) => ValueType::Bool,
            FieldValue::String(_) => ValueType::String,
        }
    }
}

pub struct Point {
    pub measurement: String,
    pub tags: BTreeMap<String, String>,
    pub time: time::SystemTime,
    pub field: BTreeMap<String, FieldValue>,
}

impl Point {
    pub fn fields(&self) -> Vec<Field> {
        let mut series_id = String::with_capacity(self.tags.len());
        series_id.push_str(&self.measurement);
        series_id.push_str(",");
        for (key, value) in self.tags.iter() {
            series_id.push_str(key);
            series_id.push_str("=");
            series_id.push_str(value);
        }

        let mut fields = Vec::with_capacity(self.field.len());
        for (key, value) in self.field.iter() {
            let mut series_id = series_id.clone();
            series_id.push_str(KEY_FIELD_SEPARATOR);
            series_id.push_str(key);
            let value = value.clone();
            let field = Field { series_id, value };
            fields.push(field);
        }

        fields
    }
}

pub struct Field {
    pub series_id: String,
    pub value: FieldValue,
}
