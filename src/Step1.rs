use scraper::{ElementRef, Html};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "main.html";
    let text = fs::read_to_string(path)?;
    let document = Html::parse_document(&text);
    let root = document.root_element();

    let lines = process_element(root, 0, &[]);
    let output = lines.join("\n");
    fs::write("steps/step1.temp", output)?;
    Ok(())
}

/// Process an element, passing down inherited attributes.
/// Returns a vector of lines, already indented.
fn process_element(el: ElementRef, indent: usize, inherited: &[(String, String)]) -> Vec<String> {
    let tag = el.value().name();
    let formatted_tag = tag;

    // Merge own attributes with inherited ones.
    let own_attrs: Vec<(String, String)> = el.value().attrs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    let merged_attrs = merge_attrs(inherited, &own_attrs);

    // Separate children into element nodes and text nodes (non‑empty text only).
    let children: Vec<_> = el.children().collect();
    let element_children: Vec<ElementRef> = children
        .iter()
        .filter_map(|n| ElementRef::wrap(*n))
        .collect();
    let text_nodes: Vec<String> = children
        .iter()
        .filter_map(|n| n.value().as_text())
        .map(|t| t.text.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Case 1: Leaf – no element children.
    if element_children.is_empty() {
        let text = el.text().collect::<String>().trim().to_string();
        let base = format_tag_with_attrs(&formatted_tag, &merged_attrs);
        let line = if text.is_empty() {
            format!("{}- {}", " ".repeat(indent), base)
        } else {
            format!("{}- {} : {}", " ".repeat(indent), base, text)
        };
        return vec![line];
    }

    // Case 3: Container (multiple children, or single child that is not a leaf).
    // Output the container without attributes, then process each child with the merged attributes.
    let mut lines = vec![format!("{}- {}:", " ".repeat(indent), formatted_tag)];
    for child in element_children {
        let mut child_lines = process_element(child, indent + 4, &merged_attrs);
        lines.append(&mut child_lines);
    }
    // Also output any direct text nodes (if any) – they become separate lines.
    // (In practice, containers rarely have direct text in well‑structured HTML.)
    for text in text_nodes {
        lines.push(format!("{}- {}", " ".repeat(indent + 4), text));
    }
    lines
}

/// Merge two lists of attributes, combining values for the same key with a space.
fn merge_attrs(inherited: &[(String, String)], own: &[(String, String)]) -> Vec<(String, String)> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (k, v) in inherited {
        map.entry(k.clone()).or_insert_with(|| v.clone());
    }
    for (k, v) in own {
        if let Some(existing) = map.get_mut(k) {
            existing.push(' ');
            existing.push_str(v);
        } else {
            map.insert(k.clone(), v.clone());
        }
    }
    map.into_iter().collect()
}

/// Format a tag with its attributes, e.g. `P[class="f"]` or `Head`.
fn format_tag_with_attrs(tag: &str, attrs: &[(String, String)]) -> String {
    if attrs.is_empty() {
        tag.to_string()
    } else {
        let attr_string = attrs
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<_>>()
            .join(" ");
        format!("{} {}", tag, attr_string)
    }
}
