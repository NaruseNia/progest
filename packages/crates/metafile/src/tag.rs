use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Getters, Setters)]
pub struct Tag {
    #[getset(get = "pub")]
    uid: String,

    #[getset(get = "pub", set = "pub")]
    name: String,
}
