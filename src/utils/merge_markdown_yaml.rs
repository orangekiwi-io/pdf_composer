use colored::Colorize;
use regex::Regex;
use serde_yaml::Value;
use std::collections::BTreeMap;

pub fn merge_markdown_yaml(
    yaml_btreemap: BTreeMap<String, Value>,
    markdown_content: &str,
) -> String {
    // TODO RL Look into if this could be taken out into a utility function
    let mut new_btreemap: BTreeMap<String, String> = BTreeMap::new();
    for (key, value) in yaml_btreemap {
        if let Value::String(string_value) = value {
            new_btreemap.insert(key, string_value);
        }
    }

    let hay = markdown_content;
    let regex = Regex::new(r"\{\{([^}]*)\}\}").unwrap();

    let replaced_string = regex.replace_all(hay, |captures: &regex::Captures<'_>| {
        let replacement_key = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        if let Some(replacement_value) = new_btreemap.get(replacement_key) {
            replacement_value.clone() // Clone the string to ensure ownership
        } else {
            // If no match found, return the original placeholder
            captures
                .get(0)
                .map(|m| String::from(m.as_str()))
                .unwrap_or_default()
        }
    });

    println!(
        "\n{}{}",
        "Markdown with replaced YAML values. Ready for PDF processing".bright_green(),
        replaced_string
    );

    replaced_string.to_string()
}
