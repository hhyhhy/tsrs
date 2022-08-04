use crate::tsdb::ValueType;
use std::collections::BTreeMap;
use std::time;
use std::time::UNIX_EPOCH;

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

#[derive(Clone)]
pub struct Point {
    pub measurement: String,
    pub tags: BTreeMap<String, String>,
    pub time: time::SystemTime,
    pub field: BTreeMap<String, FieldValue>,
}

impl Point {
    pub fn fields(&self) -> Vec<Field> {
        let tag = self
            .tags
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(",");
        let series = format!("{},{}", self.measurement, tag);

        let mut fields = Vec::with_capacity(self.field.len());
        for (k, v) in self.field.iter() {
            let series_id = format!("{}{}{}", series, KEY_FIELD_SEPARATOR, k);
            let value = v.clone();
            let field = Field { series_id, value };
            fields.push(field);
        }

        fields
    }

    pub fn unix_nano(&self) -> u128 {
        self.time.duration_since(UNIX_EPOCH).unwrap().as_nanos()
    }
}

pub struct Field {
    pub series_id: String,
    pub value: FieldValue,
}

#[cfg(test)]
mod tests {
    use crate::tsdb::point::{FieldValue, Point};
    use std::collections::BTreeMap;
    use std::time;

    #[test]
    fn test_fields() {
        let time = time::SystemTime::now();
        let tags = BTreeMap::from([
            ("host".to_string(), "A".to_string()),
            ("cpu".to_string(), "0".to_string()),
        ]);
        let p = Point {
            measurement: "cpu".to_string(),
            tags,
            time,
            field: BTreeMap::from([("value".to_string(), FieldValue::I64(10))]),
        };

        let fields = p.fields();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].series_id, "cpu,cpu=0,host=A#!~#value");
    }
}
