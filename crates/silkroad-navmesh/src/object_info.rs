use crate::object::ObjectFile;
use encoding_rs::WINDOWS_1252;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

static OBJ_MAGIC: &str = "JMXVOBJI1000";

#[derive(Error, Debug)]
pub enum ObjectInfoError {
    #[error("Invalid Magic at the start of the file")]
    InvalidMagic,
    #[error("Couldn't parse number")]
    ParseError(#[from] ParseIntError),
    #[error("The file didn't contain a count")]
    MissingCount,
    #[error("The file was too short")]
    NotEnoughLines,
}

impl From<ObjectInfoError> for io::Error {
    fn from(value: ObjectInfoError) -> Self {
        io::Error::new(ErrorKind::Other, value)
    }
}

#[derive(Clone, Debug)]
pub struct ObjectInfoEntry {
    id: u32,
    flag: u32,
    file: String,
}

impl ObjectInfoEntry {
    pub fn file_name(&self) -> &str {
        &self.file
    }

    pub fn object_file(&self) -> ObjectFile {
        ObjectFile::from(&self.file)
    }

    pub fn flag(&self) -> u32 {
        self.flag
    }
}

impl FromStr for ObjectInfoEntry {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = line.splitn(3, ' ').collect();
        let id = u32::from_str(split[0])?;
        let flag = u32::from_str_radix(split[1].trim_start_matches("0x"), 16)?;
        let file = split[2].trim_matches('"').replace('\\', "/");
        Ok(ObjectInfoEntry { id, flag, file })
    }
}

#[derive(Clone, Debug)]
pub struct ObjectInfo {
    entries: HashMap<u32, ObjectInfoEntry>,
}

impl ObjectInfo {
    pub fn from(data: &[u8]) -> Result<Self, ObjectInfoError> {
        let (content, _enc, _bool) = WINDOWS_1252.decode(data);
        let mut lines = content.split('\n');
        let magic = lines.next().unwrap();
        if magic != OBJ_MAGIC {
            return Err(ObjectInfoError::InvalidMagic);
        }

        let count_str = lines.next().ok_or(ObjectInfoError::MissingCount)?;
        let count = usize::from_str(count_str)?;
        let mut entries = HashMap::with_capacity(count);
        for _ in 0..count {
            let line = lines.next().ok_or(ObjectInfoError::NotEnoughLines)?;
            let entry = ObjectInfoEntry::from_str(line)?;
            entries.insert(entry.id, entry);
        }
        Ok(ObjectInfo { entries })
    }
}

impl IntoIterator for ObjectInfo {
    type Item = (u32, ObjectInfoEntry);
    type IntoIter = IntoIter<u32, ObjectInfoEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

pub struct ObjectStringInfo {
    pub unique_id: u32,
    pub flag: u32,
    pub region_x: u8,
    pub region_y: u8,
    pub x_offset: f32,
    pub y_offset: f32,
    pub z_offset: f32,
    pub rotation: f32,
    pub name: String,
}

impl FromStr for ObjectStringInfo {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = line.splitn(9, ' ').collect();
        let unique_id = u32::from_str_radix(split[0].trim_start_matches("0x"), 16)?;
        let flag = u32::from_str_radix(split[1].trim_start_matches("0x"), 16)?;
        let region_x = split[2].parse()?;
        let region_y = split[3].parse()?;
        let x_offset = f32::from_bits(u32::from_str_radix(split[4].trim_start_matches("0x"), 16)?);
        let y_offset = f32::from_bits(u32::from_str_radix(split[5].trim_start_matches("0x"), 16)?);
        let z_offset = f32::from_bits(u32::from_str_radix(split[6].trim_start_matches("0x"), 16)?);
        let rotation = f32::from_bits(u32::from_str_radix(split[7].trim_start_matches("0x"), 16)?);
        let name = split[8].to_owned();

        let unique_id = if (unique_id & 0xFFFF0000) != 0 {
            unique_id & 0xFFFF
        } else {
            unique_id
        };

        Ok(ObjectStringInfo {
            unique_id,
            flag,
            region_x,
            region_y,
            x_offset,
            y_offset,
            z_offset,
            rotation,
            name,
        })
    }
}

pub struct ObjectStringsInfo {
    objects: HashMap<u32, ObjectStringInfo>,
}

impl ObjectStringsInfo {
    pub fn from(data: &[u8]) -> Result<ObjectStringsInfo, ObjectInfoError> {
        let (content, _enc, _bool) = WINDOWS_1252.decode(data);
        let mut lines = content.split('\n');
        let magic = lines.next().unwrap();
        if magic != OBJ_MAGIC {
            return Err(ObjectInfoError::InvalidMagic);
        }

        let count_str = lines.next().ok_or(ObjectInfoError::MissingCount)?;
        let count = usize::from_str(count_str)?;
        let mut objects = Vec::with_capacity(count);
        for _ in 0..count {
            let line = lines.next().ok_or(ObjectInfoError::NotEnoughLines)?;
            objects.push(ObjectStringInfo::from_str(line)?);
        }

        Ok(ObjectStringsInfo {
            objects: HashMap::from_iter(objects.into_iter().map(|obj| (obj.unique_id, obj))),
        })
    }

    pub fn by_id(&self, global_id: u32) -> Option<&ObjectStringInfo> {
        self.objects.get(&global_id)
    }

    pub fn by_local_id(&self, region: u16, local_id: u16) -> Option<&ObjectStringInfo> {
        let region_part = (region as u32) << 16;
        self.by_id(region_part | local_id as u32)
    }

    pub fn objects(&self) -> impl ExactSizeIterator<Item = &ObjectStringInfo> {
        self.objects.values()
    }
}
