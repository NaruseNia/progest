use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs,
    os::windows::fs::MetadataExt,
    path::Path,
    time::SystemTime,
};

use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MetaType {
    File,
    Folder,
    Special,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum FileType {
    Image,
    Video,
    Audio,
    Document,
    ProjectFile,
    Model,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum RelationType {
    Parent,
    Child,
    Sibling,
    Related,
}

/// Metafile is a struct that represents a file or folder in the system.
/// It contains metadata about the file or folder.
/// The properties field is a generic type that can be used to store additional metadata.
#[derive(Getters, MutGetters, Serialize, Deserialize, Debug)]
pub struct Metafile<P = MetaProperty> {
    #[getset(get = "pub")]
    guid: String,

    #[getset(get = "pub")]
    name: String,

    #[getset(get = "pub")]
    raw_path: String,

    #[getset(get = "pub")]
    ext: String,

    #[getset(get = "pub")]
    meta_type: MetaType,

    #[getset(get = "pub")]
    size: u64,

    #[getset(get = "pub")]
    created_at: SystemTime,

    #[getset(get = "pub")]
    modified_at: SystemTime,

    #[getset(get = "pub")]
    tags: HashSet<String>,

    #[getset(get = "pub")]
    relations: HashMap<String, Relation>,

    #[getset(get = "pub")]
    property: P,
}

impl Metafile {
    pub fn add_relation(&mut self, relation: Relation) {
        self.relations.insert(relation.guid.clone(), relation);
    }

    pub fn remove_relation(&mut self, relation: &Relation) {
        self.relations.remove(&relation.guid);
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

    pub fn remove_tag(&mut self, tag: &String) {
        self.tags.remove(tag);
    }

    pub fn update_property(&mut self, property: MetaProperty) {
        self.property = property;
    }

    pub fn update_modified_now(&mut self) {
        self.modified_at = SystemTime::now();
    }

    pub fn update_modified_exact(&mut self, time: SystemTime) {
        self.modified_at = time;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Relation {
    guid: String,
    relation_type: RelationType,
}

impl Relation {
    pub fn new(guid: String, relation_type: RelationType) -> Self {
        Self {
            guid,
            relation_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaProperty {}

pub fn create_meta_from_file(path: &Path) -> Metafile {
    create_meta_from_file_with_property(path, MetaProperty {})
}

pub fn create_meta_from_file_with_property<P>(path: &Path, property: P) -> Metafile<P> {
    Metafile {
        guid: uuidv7::create(),
        raw_path: fs::canonicalize(path)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
        name: path
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_os_string()
            .into_string()
            .unwrap_or("".to_string()),
        ext: path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_os_string()
            .into_string()
            .unwrap_or("".to_string()),
        meta_type: match path.is_dir() {
            true => MetaType::Folder,
            false => MetaType::File,
        },
        size: path.metadata().unwrap().file_size(),
        created_at: path.metadata().unwrap().created().unwrap(),
        modified_at: path.metadata().unwrap().modified().unwrap(),
        tags: HashSet::new(),
        relations: HashMap::new(),
        property,
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_create_meta_from_file() {
        println!("{:?}", env::current_dir().unwrap());
        let path = Path::new("./test_files/test.txt");
        let meta = create_meta_from_file(path);
        assert_eq!(meta.name(), "test.txt");
        assert_eq!(meta.ext(), "txt");
        assert_eq!(meta.meta_type(), &MetaType::File);
    }
}
