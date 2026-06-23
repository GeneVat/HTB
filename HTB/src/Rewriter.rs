use regex::Regex;
use std::collections::{HashMap};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "formatted.temp";
    let pattern = r"(?i)^\s*-\s([a-z0-9-]+)(?:;.*)?$";
    let re = Regex::new(pattern)?;

    let text = fs::read_to_string(path)?;
    let mut codes: HashMap<&str, &str> = HashMap::new();

    codes.insert("float", "float");
    codes.insert("align", "align");
    codes.insert("tab", "tab");
    codes.insert("box", "box");
    codes.insert("sidebar", "sidebar");
    codes.insert("blockquote", "sidebar");
    codes.insert("b", "b");
    codes.insert("strong", "b");
    codes.insert("i", "i");
    codes.insert("em", "i");
    codes.insert("u", "u");
    codes.insert("sup", "sup");
    codes.insert("sub", "sub");
    codes.insert("strike", "strike");
    codes.insert("del", "strike");
    codes.insert("s", "strike");
    codes.insert("pre", "pre");
    codes.insert("hr", "hr");
    codes.insert("a", "url");
    codes.insert("anchor", "anchor");
    codes.insert("nation", "nation");
    codes.insert("region", "region");
    codes.insert("proposal", "proposal");
    codes.insert("quote", "quote");
    codes.insert("title", "size=180");
    codes.insert("h1", "size=150");
    codes.insert("h2", "size=135");
    codes.insert("h3", "size=120");
    codes.insert("h4", "size=105");
    codes.insert("h5", "size=90");
    codes.insert("color", "color");
    codes.insert("background", "background");
    codes.insert("background-block", "background-block");
    codes.insert("spoiler", "spoiler");
    codes.insert("ul", "list");
    codes.insert("ol", "list");
    codes.insert("li", "*");
    codes.insert("table", "table");
    codes.insert("tr", "tr");
    codes.insert("td", "td");
    codes.insert("th", "td");

    

    let mut out = String::new();
    let mut last = 0usize;

    for mat in re.find_iter(&text) {
        out.push_str(&text[last..mat.start()]);

        let caps = re.captures(mat.as_str()).unwrap();
        let tag = caps
            .get(2)
            .map(|m| m.as_str().to_ascii_lowercase())
            .unwrap_or_default();

        let tag_str = tag.as_str();

        

        if let Some(v) = codes.get(tag_str) {
            let repl = format!("[{}]", v);
            out.push_str(&repl);
        }

        last = mat.end();
    }

    out.push_str(&text[last..]);

    fs::write("Writer.temp", out)?;
    Ok(())
}
