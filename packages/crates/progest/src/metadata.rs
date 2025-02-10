use std::{collections::HashMap, io::Read};

use brotli::{CompressorReader, Decompressor};
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

#[derive(Getters, Setters, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Project {
    #[getset(get = "pub")]
    uid: String,

    #[getset(get = "pub", set = "pub")]
    name: String,

    #[getset(get = "pub", set = "pub")]
    description: String,

    #[getset(get = "pub")]
    created_at: String,

    #[getset(get = "pub", set = "pub")]
    updated_at: String,

    #[getset(get = "pub")]
    tags: HashMap<String, Tag>,
}

impl Project {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            uid: ulid::Ulid::new().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            tags: HashMap::new(),
        }
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag.uid().to_string(), tag.clone());
    }

    pub fn remove_tag(&mut self, tag: &Tag) {
        self.tags.remove(tag.uid());
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
