use xml_builder::{XML, XMLBuilder, XMLElement, XMLVersion};
use package_helper::{Meta, MetaProperty, Version};
use crate::models::ModState;

pub struct ModSettingsBuilder;

impl ModSettingsBuilder {
    pub fn build(mod_metas: &[ModState], gustav_dev_meta: &Meta) -> XML {
        let mut save = XMLElement::new("save");

        let mut version = XMLElement::new("version");
        version.add_attribute("major", "4");
        version.add_attribute("minor", "3");
        version.add_attribute("revision", "0");
        version.add_attribute("build", "300");
        save.add_child(version).expect("Failed to add 'version' child to 'save' node. 'save' should always be a node.");

        let mods_node = Self::build_mods_node(mod_metas, gustav_dev_meta);
        let mut mod_order_node = XMLElement::new("node");
        mod_order_node.add_attribute("id", "ModOrder");

        let mut children = XMLElement::new("children");
        children.add_child(mod_order_node).expect("Failed to add 'mod_order_node' child to 'children' node. 'children' should always be a node.");
        children.add_child(mods_node).expect("Failed to add 'mods_node' child to 'children' node. 'children' should always be a node.");

        let mut root_node = XMLElement::new("node");
        root_node.add_attribute("id", "root");
        root_node.add_child(children).expect("Failed to add 'children' child to 'node' node. 'node' should always be a node.");

        let mut module_settings = XMLElement::new("region");
        module_settings.add_attribute("id", "ModuleSettings");
        module_settings.add_child(root_node).expect("Failed to add 'root_node' child to 'region' node. 'region' should always be a node.");

        save.add_child(module_settings).expect("Failed to add 'module_settings' child to 'save' node. 'save' should always be a node.");

        let mut xml = XMLBuilder::new().version(XMLVersion::XML1_0)
            .encoding(String::from("UTF-8")).build();
        xml.set_root_element(save);
        xml
    }

    fn build_mods_node(mod_metas: &[ModState], gustav_dev_meta: &Meta) -> XMLElement {
        let mut children = XMLElement::new("children");

        let gustav_dev = Self::build_mod_desc(gustav_dev_meta);
        children.add_child(gustav_dev).expect("Failed to add 'gustav_dev' child to 'children' node. 'children' should always be a node.");

        for mod_state in mod_metas {
            if !mod_state.enabled { continue }
            if let Some(meta) = mod_state.meta.as_ref() {
                let mod_desc = Self::build_mod_desc(meta);
                children.add_child(mod_desc).expect("Failed to add 'mod_desc' child to 'children' node. 'children' should always be a node.");
            }
        }

        let mut mods_node = XMLElement::new("node");
        mods_node.add_attribute("id", "Mods");
        mods_node.add_child(children).expect("Failed to add 'children' child to 'mods_node' node. 'mods_node' should always be a node.");

        mods_node
    }

    fn build_mod_desc(mod_meta: &Meta) -> XMLElement {
        let folder = Self::build_mod_meta_attribute("Folder", mod_meta.folder());
        let md5 = Self::build_mod_meta_attribute("MD5", mod_meta.md5());
        let name = Self::build_mod_meta_attribute("Name", mod_meta.name());
        let uuid = Self::build_mod_meta_attribute("UUID", mod_meta.uuid());
        let version64 = Self::build_version64_attribute(mod_meta.version());

        let mut desc = XMLElement::new("node");
        desc.add_attribute("id", "ModuleShortDesc");
        desc.add_child(folder).expect("Failed to add 'folder' child to 'desc' node. 'desc' should always be a node.");
        desc.add_child(md5).expect("Failed to add 'md5' child to 'desc' node. 'desc' should always be a node.");
        desc.add_child(name).expect("Failed to add 'name' child to 'desc' node. 'desc' should always be a node.");
        desc.add_child(uuid).expect("Failed to add 'uuid' child to 'desc' node. 'desc' should always be a node.");
        desc.add_child(version64).expect("Failed to add 'version64' child to 'desc' node. 'desc' should always be a node.");
        desc
    }

    fn build_version64_attribute(version: &Version) -> XMLElement {
        let mut attribute = XMLElement::new("attribute");
        attribute.add_attribute("id", "Version64");
        attribute.add_attribute("type", "int64");
        attribute.add_attribute("value", &version.version64().to_string());
        attribute
    }

    fn build_mod_meta_attribute(name: &str, attribute_info: &MetaProperty) -> XMLElement {
        let mut attribute = XMLElement::new("attribute");
        attribute.add_attribute("id", name);
        attribute.add_attribute("type", attribute_info.value_type());
        attribute.add_attribute("value", attribute_info.value());
        attribute
    }
}