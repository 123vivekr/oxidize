use std::collections::HashMap;
use std::fs::read_to_string;
use toml::Value;

pub fn fetch_alternatives() -> HashMap<String, String> {
    let mut alternatives_map: HashMap<String, String> = HashMap::new();

    let alternatives_contents =
        read_to_string("alternatives.toml").expect("Something went wrong reading the file");
    let value = alternatives_contents.parse::<Value>().unwrap();
    if let Some(alternatives) = value.get("alternatives").and_then(|v| v.as_table()) {
        alternatives_map = alternatives
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();
    }

    alternatives_map
}
