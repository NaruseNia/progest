use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum RelationType {
    Parent,
    Child,
    Sibling,
    Related,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Getters, Setters, Clone)]
pub struct Relation {
    #[getset(get = "pub")]
    uid: String,

    #[getset(get = "pub", set = "pub")]
    relation_type: RelationType,
}

impl Relation {
    pub fn new(relation_type: RelationType) -> Self {
        Self {
            uid: ulid::Ulid::new().to_string(),
            relation_type,
        }
    }
}
