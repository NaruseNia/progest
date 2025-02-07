use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs,
    io::Read,
    os::windows::fs::MetadataExt,
    path::Path,
    time::SystemTime,
};

use brotli::{CompressorReader, Decompressor};
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
#[derive(Getters, MutGetters, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Metafile<P = MetaProperty> {
    #[getset(get = "pub")]
    uid: String,

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

    pub fn serialize(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(&self).unwrap()
    }

    pub fn deserialize(data: Vec<u8>) -> Self {
        rmp_serde::from_slice(&data).unwrap()
    }

    pub fn deserialize_compressed(data: &Vec<u8>) -> Self {
        let decompressed_data = Self::decompress_data(data);
        rmp_serde::from_slice(&decompressed_data).unwrap()
    }

    pub fn serialize_compressed(&self) -> Vec<u8> {
        let serialized_data = rmp_serde::to_vec_named(&self).unwrap();
        Self::compress_data(serialized_data)
    }

    fn compress_data(data: Vec<u8>) -> Vec<u8> {
        let mut compressor = CompressorReader::new(data.as_slice(), 4096, 11, 22);
        let mut compressed_bytes = Vec::new();
        compressor.read_to_end(&mut compressed_bytes).unwrap();
        compressed_bytes
    }

    fn decompress_data(data: &Vec<u8>) -> Vec<u8> {
        let mut decompressor = Decompressor::new(data.as_slice(), 4096);
        let mut decompressed_bytes = Vec::new();
        decompressor.read_to_end(&mut decompressed_bytes).unwrap();
        decompressed_bytes
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MetaProperty {}

pub fn create_meta_from_file(path: &Path) -> Metafile {
    create_meta_from_file_with_property(path, MetaProperty {})
}

pub fn create_meta_from_file_with_property<P>(path: &Path, property: P) -> Metafile<P> {
    Metafile {
        uid: ulid::Ulid::new().to_string(),
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
    use super::*;

    #[test]
    fn test_create_meta_from_file() {
        let path = Path::new("./test_files/test.txt");
        let meta = create_meta_from_file(path);
        assert_eq!(meta.name(), "test.txt");
        assert_eq!(meta.ext(), "txt");
        assert_eq!(meta.meta_type(), &MetaType::File);

        let path_out = format!("./test_files/{}@{}.pgmeta", meta.name(), meta.uid());
        fs::write(&path_out, meta.serialize_compressed()).unwrap();
        let meta_out = Metafile::deserialize_compressed(&fs::read(&path_out).unwrap());

        assert_eq!(meta, meta_out);
        let _ = fs::remove_file(path_out);
    }
}
