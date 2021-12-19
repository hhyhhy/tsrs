use std::time;

struct Tag {
    pub(crate) key: String,
    pub(crate) value: String,
}

struct Point<T> {
    pub(crate) measurement: String,
    pub(crate) tags: Vec<Tag>,
    pub(crate) time: time::Instant,
    pub(crate) field: T,
}



