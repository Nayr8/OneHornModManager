use std::path::Path;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use models::{Mod, UnpackingFileError};
use models::UnpackingFileError::MetaDataMissingModuleInfo;
use crate::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModInfoNode {
    pub value_type: String,
    pub value: String,
}

impl ModInfoNode {
    fn new(node: Node) -> Result<ModInfoNode, UnpackingFileError> {
        Ok(ModInfoNode {
            value_type: node.attribute("type").ok_or(MetaDataMissingModuleInfo)?.into(),
            value: node.attribute("value").ok_or(MetaDataMissingModuleInfo)?.into(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModMeta {
    pub name: ModInfoNode,
    pub description: ModInfoNode,
    pub folder: ModInfoNode,
    pub uuid: ModInfoNode,
    pub md5: ModInfoNode,
    pub version64: ModInfoNode,
}

impl ModMeta {
    pub(crate) fn new(module_info: Node) -> Result<ModMeta, UnpackingFileError> {
        let name = match Self::read_property(&module_info, "Name") {
            Ok(name) => name,
            Err(error) => {
                error!("Meta missing mod name");
                return Err(error);
            }
        };
        let folder = match Self::read_property(&module_info, "Folder") {
            Ok(name) => name,
            Err(error) => {
                error!("Meta missing mod name");
                return Err(error);
            }
        };
        let uuid = match Self::read_property(&module_info, "UUID") {
            Ok(name) => name,
            Err(error) => {
                error!("Meta missing mod name");
                return Err(error);
            }
        };
        let md5 = Self::read_property(&module_info, "MD5")
            .unwrap_or(ModInfoNode {
                value_type: String::from("LSString"),
                value: String::new(),
            });
        let version64 = Self::read_property(&module_info, "Version64")
            .unwrap_or(ModInfoNode {
                value_type: String::from("int64"),
                value: String::from("36028797018963968"),
            });
        let description = Self::read_property(&module_info, "Description")
            .unwrap_or(ModInfoNode {
                value_type: String::from("LSString"),
                value: String::new(),
            });

        Ok(ModMeta {
            name,
            description,
            folder,
            uuid,
            md5,
            version64,
        })
    }

    fn read_property(module_info: &Node, id: &str) -> Result<ModInfoNode, UnpackingFileError> {
        ModInfoNode::new(module_info.children()
            .find(|n| n.attribute("id") == Some(id))
            .ok_or(MetaDataMissingModuleInfo)?)
    }

    pub fn get_mod_details(meta: &Option<ModMeta>, file_path: &Path) -> Mod {
        match meta.as_ref() {
            Some(meta) => Mod {
                name: meta.name.value.clone(),
                description: meta.description.value.clone(),
            },
            None => {
                let name = file_path.file_name().unwrap()
                    .to_string_lossy()
                    .trim_end_matches(".pak")
                    .to_string();
                Mod {
                    name,
                    description: String::new()
                }
            }
        }
    }
}