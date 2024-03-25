use std::collections::BTreeMap;
use serde_yaml::Value;

/// Converts a YAML mapping into a Rust BTreeMap with string keys and arbitrary values. BTreeMaps are automatically alphabetically sorted.
///
/// # Arguments
///
/// * `yaml` - A reference to a serde_yaml::Value containing the YAML data to convert.
///
/// # Returns
///
/// An Option containing the resulting BTreeMap<String, Value>. If the YAML data is successfully
/// converted into a BTreeMap, it returns Some(btreemap); otherwise, it returns None.
///
/// # Examples
///
/// ```
/// use serde_yaml::Value;
/// use std::collections::BTreeMap;
///
/// let yaml_data = serde_yaml::from_str("name: John\nage: 30").unwrap();
/// let btreemap = yaml_mapping_to_btreemap(&yaml_data).unwrap();
/// assert_eq!(btreemap.get("name"), Some(&Value::String("John".to_string())));
/// assert_eq!(btreemap.get("age"), Some(&Value::Number(30.into())));
/// ```
pub fn yaml_mapping_to_btreemap(yaml: &Value) -> Option<BTreeMap<String, Value>> {
    match yaml {
        // Match if yaml Value contains a Mapping 'object'
        Value::Mapping(mapping_value) => {
            // Create a new BTreeMap to hold the YAML data
            let mut yaml_btreemap: BTreeMap<String, Value> = BTreeMap::new();

            // Iterate over key-value pairs in the mapping
            for (key, value) in mapping_value.iter() {
                // Destructure the key-value tuple, if the key is of type Value::String.
                if let (Value::String(key), value) = (key, value) {
                    // Insert key-value pair into the BTreeMap. The key and value values are cloned because or_insert takes ownership of the arguments
                    yaml_btreemap.entry(key.clone()).or_insert(value.clone());
                } else {
                    // Handle non-string keys or non-scalar values
                    return None;
                }
            }

            // println!("{} {:#?}", "yaml_btreemap".cyan(), yaml_btreemap);
            // Return the resulting BTreeMap
            Some(yaml_btreemap)
        }
        _ => None, // Return None if yaml is not a mapping
    }
}