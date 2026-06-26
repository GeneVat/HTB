use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("main.html")?;

    let lang_re = Regex::new(r#"(?i)<[^>]*\blang="[^"]*"[^>]*>"#)?;
    let meta_re = Regex::new(r#"(?i)<meta\s+[^>]*>"#)?;
    let style_re = Regex::new(r#"(?i)<style>\s+[^>]*</style>"#)?;

    let out = lang_re.replace_all(&text, "")
        .to_string();
        
    let out = style_re.replace_all(&out, "")
        .to_string();

    let out = meta_re.replace_all(&out, "")
        .to_string();

    let out = out.replace("<!DOCTYPE html>", "");

    fs::write("steps/step0.temp", out)?;
    Ok(())
}