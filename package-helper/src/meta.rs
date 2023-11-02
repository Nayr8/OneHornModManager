use serde::{Serialize, Deserialize};
use crate::error::MetaReadError;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Meta {
    name: MetaProperty,
    description: String,
    folder: MetaProperty,
    uuid: MetaProperty,
    md5: MetaProperty,
    version64: MetaProperty,
}

impl TryFrom<roxmltree::Document<'_>> for Meta {
    type Error = MetaReadError;

    fn try_from(xml: roxmltree::Document) -> Result<Self, Self::Error> {
        let module_info = xml.descendants().find(|n| {
            n.attribute("id") == Some("ModuleInfo")
        }).ok_or_else(|| {
            MetaReadError::MetaDataMissingModuleInfo
        })?;

        Meta::from_module_info_node(module_info)
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
            version64: MetaProperty {
                value_type: String::from("int64"),
                value: String::from("36028797018963968"),
            },
        }
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

    pub fn version64(&self) -> &MetaProperty {
        &self.version64
    }

    fn from_module_info_node(module_info: roxmltree::Node) -> Result<Meta, MetaReadError> {
        let name = Self::read_property(&module_info, "Name")?;
        let folder = Self::read_property(&module_info, "Folder")?;
        let uuid = Self::read_property(&module_info, "UUID")?;
        let md5 = Self::read_property(&module_info, "MD5")
            .unwrap_or(MetaProperty {
                value_type: String::from("LSString"),
                value: String::new(),
            });
        let version64 = Self::read_property(&module_info, "Version64")
            .unwrap_or(MetaProperty {
                value_type: String::from("int64"),
                value: String::from("36028797018963968"),
            });
        let description = Self::read_property(&module_info, "Description")
            .map(|description| description.value).unwrap_or(String::new());

        Ok(Meta {
            name,
            description,
            folder,
            uuid,
            md5,
            version64,
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