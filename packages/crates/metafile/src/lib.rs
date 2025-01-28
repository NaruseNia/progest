use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MetaType {
    File,
    Folder,
    Special,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metafile {
    guid: String,
    name: String,
    ext: String,
    meta_type: MetaType,
    size: u64,
    created_at: String,
    updated_at: String,
    tags: Vec<String>,
    relations: Vec<Relation>,
    properties: SpecialProperty,
}

#[derive(Serialize, Deserialize, Debug)]
struct Relation {
    guid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpecialProperty {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        panic!("This test will fail");
    }
}
