use wasm_bindgen::JsCast;

pub struct DomHelper;

impl DomHelper {
    pub fn read_input(id: &str) -> Option<String> {
        let input_node = web_sys::window()?
            .document()?
            .get_element_by_id(id)?
            .dyn_into::<web_sys::HtmlInputElement>().ok()?;
        Some(input_node.value())
    }
}