use getset::Getters;
use metafile::tag::Tag;
use serde::{Deserialize, Serialize};

#[derive(Getters, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProjectRoot {
    #[getset(get = "pub")]
    uid: String,

    #[getset(get = "pub")]
    name: String,

    #[getset(get = "pub")]
    path: String,

    #[getset(get = "pub")]
    description: String,

    #[getset(get = "pub")]
    created_at: String,

    #[getset(get = "pub")]
    updated_at: String,

    #[getset(get = "pub")]
    tags: Vec<Tag>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!();
    }
}
