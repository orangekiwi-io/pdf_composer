use regex::Regex;
use serde_yaml::Value;
use std::collections::BTreeMap;

/// This function merges the YAML data from a `BTreeMap<String, Value>` into a given Markdown content string.
///
/// # Arguments
///
/// * `yaml_btreemap` - A `BTreeMap<String, Value>` containing the YAML data to be merged into the Markdown content.
/// * `markdown_content` - A string slice (`&str`) representing the Markdown content into which the YAML data should be merged.
///
/// # Returns
///
/// A `String` containing the Markdown content with the YAML data merged into it.
///
/// # Remarks
///
/// The function performs the following steps:
///
/// 1. Creates a new `BTreeMap<String, String>` called `new_btreemap` to store the string values from the `yaml_btreemap`.
/// 2. Iterates over the key-value pairs in `yaml_btreemap` and inserts the string values into `new_btreemap`.
/// 3. Defines a regular expression pattern (`r"\{\{(\w+)\}\}"`) to match placeholders in the Markdown content.
/// 4. Uses the `regex` crate's `replace_all` function to replace the placeholders in the Markdown content with the corresponding values from `new_btreemap`.
/// 5. If a matching value is found in `new_btreemap`, it replaces the placeholder with the value.
/// 6. If no matching value is found, it leaves the original placeholder unchanged.
/// 7. Returns the resulting string with the YAML data merged into the Markdown content.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use serde_yaml::Value;
///
/// // Define YAML data as a BTreeMap
/// let mut yaml_data = BTreeMap::new();
/// yaml_data.insert("name".to_string(), serde_yaml::Value::String("Richard".to_string()));
/// yaml_data.insert("age".to_string(), serde_yaml::Value::String("23".to_string()));
///
/// // Define Markdown content with placeholders
/// let markdown_content = "Name: {{name}}\nAge: {{age}}";
///
/// // Merge YAML data into Markdown content
/// let merged_content = merge_markdown_yaml(yaml_data, markdown_content);
///
/// // Check if merging was successful
/// assert_eq!(merged_content, "Name: Richard\nAge: 23");
/// ```
pub fn merge_markdown_yaml(
    yaml_btreemap: BTreeMap<String, Value>,
    markdown_content: &str,
) -> String {
    // Create a new BTreeMap to store string values from the YAML data
    let mut new_btreemap: BTreeMap<String, String> = BTreeMap::new();

    // Iterate over the key-value pairs in yaml_btreemap
    for (key, value) in yaml_btreemap {
        // If the value is a string, insert it into new_btreemap
        if let Value::String(string_value) = value {
            new_btreemap.insert(key, string_value);
        }
    }

    // Create a reference to the markdown_content string
    let hay = markdown_content;

    // Define a regular expression pattern to match placeholders in the Markdown content
    let regex = Regex::new(r"\{\{(\w+)\}\}").unwrap();

    // Replace the placeholders in the Markdown content with the corresponding values from new_btreemap
    let replaced_string = regex.replace_all(hay, |captures: &regex::Captures<'_>| {
        // Get the placeholder key from the capture group
        let replacement_key = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        // Check if the key exists in new_btreemap
        if let Some(replacement_value) = new_btreemap.get(replacement_key) {
            // If the key exists, return the corresponding value (cloned to ensure ownership)
            replacement_value.clone()
        } else {
            // If the key doesn't exist, return the original placeholder
            captures
                .get(0)
                .map(|m| String::from(m.as_str()))
                .unwrap_or_default()
        }
    });
    // Return the resulting string with the YAML data merged into the Markdown content
    replaced_string.to_string()
}
