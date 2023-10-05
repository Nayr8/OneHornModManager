use xml_builder::{XML, XMLBuilder, XMLElement, XMLVersion};
use crate::mod_package::{ModInfoNode, ModMeta};

pub(crate) struct ModSettingsBuilder;

impl ModSettingsBuilder {
    pub fn build(mod_metas: &[(ModMeta, String)], gustav_dev_meta: &ModMeta) -> XML {
        let mut save = XMLElement::new("save");

        let mut version = XMLElement::new("version");
        version.add_attribute("major", "4");
        version.add_attribute("minor", "2");
        version.add_attribute("revision", "0");
        version.add_attribute("build", "100");
        save.add_child(version).unwrap();

        let mods_node = Self::build_mods_node(mod_metas, gustav_dev_meta);
        let mut mod_order_node = XMLElement::new("node");
        mod_order_node.add_attribute("id", "ModOrder");

        let mut children = XMLElement::new("children");
        children.add_child(mod_order_node).unwrap();
        children.add_child(mods_node).unwrap();

        let mut root_node = XMLElement::new("node");
        root_node.add_attribute("id", "root");
        root_node.add_child(children).unwrap();

        let mut module_settings = XMLElement::new("region");
        module_settings.add_attribute("id", "ModuleSettings");
        module_settings.add_child(root_node).unwrap();

        save.add_child(module_settings).unwrap();

        let mut xml = XMLBuilder::new().version(XMLVersion::XML1_0)
            .encoding(String::from("UTF-8")).build();
        xml.set_root_element(save);
        xml
    }

    fn build_mods_node(mod_metas: &[(ModMeta, String)], gustav_dev_meta: &ModMeta) -> XMLElement {
        let mut children = XMLElement::new("children");

        let gustav_dev = Self::build_mod_desc(gustav_dev_meta);
        children.add_child(gustav_dev).unwrap();

        for (mod_meta, _) in mod_metas.iter() {
            let mod_desc = Self::build_mod_desc(mod_meta);
            children.add_child(mod_desc).unwrap();
        }

        let mut mods_node = XMLElement::new("node");
        mods_node.add_attribute("id", "Mods");
        mods_node.add_child(children).unwrap();

        mods_node
    }

    fn build_mod_desc(mod_meta: &ModMeta) -> XMLElement {
        let folder = Self::build_mod_meta_attribute("Folder", &mod_meta.folder);
        let md5 = Self::build_mod_meta_attribute("MD5", &mod_meta.folder);
        let name = Self::build_mod_meta_attribute("Name", &mod_meta.folder);
        let uuid = Self::build_mod_meta_attribute("UUID", &mod_meta.folder);
        let version64 = Self::build_mod_meta_attribute("Version64", &mod_meta.folder);

        let mut desc = XMLElement::new("node");
        desc.add_attribute("id", "ModuleShortDesc");
        desc.add_child(folder).unwrap();
        desc.add_child(md5).unwrap();
        desc.add_child(name).unwrap();
        desc.add_child(uuid).unwrap();
        desc.add_child(version64).unwrap();
        desc
    }

    fn build_mod_meta_attribute(name: &str, attribute_info: &ModInfoNode) -> XMLElement {
        let mut attribute = XMLElement::new("attribute");
        attribute.add_attribute("id", name);
        attribute.add_attribute("type", &attribute_info.value_type);
        attribute.add_attribute("value", &attribute_info.value);
        attribute
    }
}