use std::collections::HashMap;
use std::time;

pub enum FieldValue {
    Float(f64),
    Integer(i64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Point {
    pub measurement: String,
    pub tags: HashMap<String, String>,
    pub time: time::SystemTime,
    pub field: HashMap<String, FieldValue>,
}
