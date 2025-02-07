use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Getters, Setters)]
pub struct Tag {
    #[getset(get = "pub")]
    uid: String,

    #[getset(get = "pub", set = "pub")]
    name: String,
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Self {
            uid: ulid::Ulid::new().to_string(),
            name: name.to_string(),
        }
    }
}
