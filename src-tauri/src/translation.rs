use hashbrown::HashMap;
use log::info;
use serde_json::Value;


#[tauri::command]
pub fn load_translation(translation: String) -> Result<HashMap<String, String>, ()> {
    info!("Loading translation {translation}");
    let json = std::fs::read_to_string(format!("../public/translations/{translation}.json")).map_err(|e| {
        ()
    })?;

    let mut map = HashMap::new();
    parse_into_flat_map(String::new(), &serde_json::from_str(&json).map_err(|_| ())?, &mut map);
    Ok(map)
}

fn parse_into_flat_map(prefix: String, value: &Value, map: &mut HashMap<String, String>) {
    match value {
        Value::Object(map_inner) => {
            for (key, value) in map_inner.iter() {
                let new_prefix = if prefix.is_empty() {
                    key.to_string()
                } else {
                    format!("{}:{}", prefix, key)
                };

                parse_into_flat_map(new_prefix, value, map);
            }
        }
        Value::String(str_val) => {
            map.insert(prefix.clone(), str_val.clone());
        }
        _ => {} // Ignore other JSON value types
    }
}