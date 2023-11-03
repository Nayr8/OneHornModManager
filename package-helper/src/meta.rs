use serde::{Serialize, Deserialize};
use crate::error::MetaReadError;

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaProperty {
    value_type: String,
    value: String,
}

impl MetaProperty {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn value_type(&self) -> &str {
        &self.value_type
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    major: u64,
    minor: u64,
    revision: u64,
    build: u64,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}.{}", self.major, self.minor, self.revision, self.build)
    }
}

impl Version {
    pub fn version64(&self) -> u64 {
        (self.major << 55) | (self.minor << 47) | (self.revision << 31) | self.build
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    name: MetaProperty,
    description: String,
    folder: MetaProperty,
    uuid: MetaProperty,
    md5: MetaProperty,
    version: Version,
}

impl TryFrom<roxmltree::Document<'_>> for Meta {
    type Error = MetaReadError;

    fn try_from(xml: roxmltree::Document) -> Result<Self, Self::Error> {
        let module_info = xml.descendants().find(|n| {
            n.attribute("id") == Some("ModuleInfo")
        }).ok_or_else(|| {
            MetaReadError::MetaDataMissingModuleInfo
        })?;

        let version = Meta::read_version(&xml)?;

        Meta::from_module_info_node(module_info, version)
    }
}

impl Meta {
    pub fn gustav_dev() -> Meta {
        Meta {
            name: MetaProperty {
                value_type: String::from("LSString"),
                value: String::from("GustavDev"),
            },
            description: String::new(),
            folder: MetaProperty {
                value_type: String::from("LSString"),
                value: String::from("GustavDev"),
            },
            uuid: MetaProperty {
                value_type: String::from("FixedString"),
                value: String::from("28ac9ce2-2aba-8cda-b3b5-6e922f71b6b8"),
            },
            md5: MetaProperty {
                value_type: String::from("LSString"),
                value: String::new(),
            },
            version: Version {
                major: 1,
                minor: 0,
                revision: 0,
                build: 0,
            },
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn name(&self) -> &MetaProperty {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn folder(&self) -> &MetaProperty {
        &self.folder
    }

    pub fn uuid(&self) -> &MetaProperty {
        &self.uuid
    }

    pub fn md5(&self) -> &MetaProperty {
        &self.md5
    }

    fn from_module_info_node(module_info: roxmltree::Node, version: Version) -> Result<Meta, MetaReadError> {
        let name = Self::read_property(&module_info, "Name")?;
        let folder = Self::read_property(&module_info, "Folder")?;
        let uuid = Self::read_property(&module_info, "UUID")?;
        let md5 = Self::read_property(&module_info, "MD5")
            .unwrap_or(MetaProperty {
                value_type: String::from("LSString"),
                value: String::new(),
            });
        let description = Self::read_property(&module_info, "Description")
            .map(|description| description.value).unwrap_or(String::new());

        Ok(Meta {
            name,
            description,
            folder,
            uuid,
            md5,
            version,
        })
    }

    fn read_version(xml: &roxmltree::Document) -> Result<Version, MetaReadError> {
        let module_info = xml.descendants().find(|n| {
            n.has_tag_name("version")
        }).ok_or_else(|| {
            MetaReadError::MetaDataMissingVersion
        })?;

        Ok(Version {
            major: module_info.attribute("major")
                .ok_or_else(|| MetaReadError::MetaDataInvalidVersion)?
                .parse::<u64>().map_err(|_| MetaReadError::MetaDataInvalidVersion)?,
            minor: module_info.attribute("minor")
                .ok_or_else(|| MetaReadError::MetaDataInvalidVersion)?
                .parse::<u64>().map_err(|_| MetaReadError::MetaDataInvalidVersion)?,
            revision: module_info.attribute("revision")
                .ok_or_else(|| MetaReadError::MetaDataInvalidVersion)?
                .parse::<u64>().map_err(|_| MetaReadError::MetaDataInvalidVersion)?,
            build: module_info.attribute("build")
                .ok_or_else(|| MetaReadError::MetaDataInvalidVersion)?
                .parse::<u64>().map_err(|_| MetaReadError::MetaDataInvalidVersion)?,
        })
    }

    fn read_property(module_info: &roxmltree::Node, id: &str) -> Result<MetaProperty, MetaReadError> {
        let node = module_info.children()
            .find(|n| n.attribute("id") == Some(id))
            .ok_or(MetaReadError::MetaDataMissingModuleInfo)?;

        Ok(MetaProperty {
            value_type: node.attribute("type").ok_or(MetaReadError::MetaDataMissingModuleInfo)?.into(),
            value: node.attribute("value").ok_or(MetaReadError::MetaDataMissingModuleInfo)?.into(),
        })
    }
}