use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("steps/step2.temp")?;

    // 1) Rewrite style="k:v; k2:v2" -> [k=v][k2=v2]
    let style_re = Regex::new(r#"style="([^"]*)""#)?;
    let mut out = style_re
        .replace_all(&text, |caps: &regex::Captures| {
            let raw = caps.get(1).map(|m| m.as_str()).unwrap_or("");

            let items = raw
                .split(';')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .filter_map(|pair| {
                    let mut it = pair.splitn(2, ':');
                    let key = it.next()?.trim();
                    let val = it.next()?.trim();
                    if key.is_empty() || val.is_empty() {
                        None
                    } else {
                        Some((key, val))
                    }
                });

            let mut res = String::new();
            for (k, v) in items {
                res.push('[');
                res.push_str(k);
                res.push('=');
                res.push_str(v);
                res.push(']');
            }
            res
        })
        .into_owned();

    // 2) Rewrite the list item format -> [tag]rest (your original r1 logic)
    let re = Regex::new(
        r#"(?im)^(?P<indent>\s*)(?:-\s*)?(?:\[(?P<r>[a-z0-9]+)\]\s*)?-\s*(?P<r2>[a-z0-9]+)(?P<rest>.*)$"#,
    )?;
    out = re
        .replace_all(&out, |caps: &regex::Captures| {
            let indent = &caps["indent"];
            let r = if caps.name("r").is_some() {
                &caps["r"]
            } else {
                &caps["r2"]
            };
            format!("{}[{}]{}", indent, r, &caps["rest"])
        })
        .into_owned();

    // 3) Cleanup
    out = out.replace(" : ", " ").replace(": ", " ").replace(':', "");

    fs::write("steps/step3.temp", out)?;
    Ok(())
}
