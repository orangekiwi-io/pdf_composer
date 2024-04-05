// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::BTreeMap;
use serde_yaml::Value;

/// This function converts a YAML `Value::Mapping` into a `BTreeMap<String, Value>`. BTreeMaps are automatically alphabetically sorted.
///
/// # Arguments
///
/// * `yaml` - A reference to a `serde_yaml::Value` representing the YAML data to convert.
///
/// # Returns
///
/// * `Some(BTreeMap<String, Value>)` if the provided `yaml` value is a `Value::Mapping`.
/// * `None` if the provided `yaml` value is not a `Value::Mapping`, or if it contains non-string keys or non-scalar values.
///
/// # Remarks
///
/// The function first matches the provided `yaml` value against the `Value::Mapping` variant. If the match succeeds, it creates a new `BTreeMap<String, Value>` called `yaml_btreemap`.
///
/// It then iterates over the key-value pairs in the mapping. For each key-value pair, it checks if the key is a `Value::String`. If the key is a string, it inserts the cloned key-value pair into the `yaml_btreemap` using the `entry` and `or_insert` methods.
///
/// If the key is not a string, or if the value is not a scalar (e.g., another mapping or a sequence), the function returns `None`.
///
/// After iterating over all key-value pairs, the function returns the `yaml_btreemap` wrapped in a `Some` variant.
///
/// If the provided `yaml` value is not a `Value::Mapping`, the function immediately returns `None`.
///
/// # Examples
///
/// ```
/// use serde_yaml::Value;
/// use std::collections::BTreeMap;
///
/// // Construct a YAML mapping
/// let yaml_mapping = serde_yaml::Mapping::new();
/// let mut yaml_value = serde_yaml::Value::Mapping(yaml_mapping);
///
/// // Add key-value pairs to the YAML mapping
/// yaml_value.as_mapping_mut().unwrap().insert(
///     serde_yaml::Value::String("key1".to_string()),
///     serde_yaml::Value::String("value1".to_string())
/// );
/// yaml_value.as_mapping_mut().unwrap().insert(
///     serde_yaml::Value::String("key2".to_string()),
///     serde_yaml::Value::String("value2".to_string())
/// );
///
/// // Convert YAML mapping to a BTreeMap
/// let btreemap = yaml_mapping_to_btreemap(&yaml_value);
///
/// // Check if conversion was successful
/// assert_eq!(btreemap, Some({
///     let mut map = BTreeMap::new();
///     map.insert("key1".to_string(), serde_yaml::Value::String("value1".to_string()));
///     map.insert("key2".to_string(), serde_yaml::Value::String("value2".to_string()));
///     map
/// }));
/// ```
pub fn yaml_mapping_to_btreemap(yaml: &Value) -> Option<BTreeMap<String, Value>> {
    match yaml {
        // Match if `yaml` Value contains a Mapping 'object'
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

            // Return the resulting BTreeMap
            Some(yaml_btreemap)
        }
        _ => None, // Return None if `yaml` is not a mapping
    }
}
