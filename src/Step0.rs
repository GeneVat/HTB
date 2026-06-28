use regex::Regex;
use std::{error::Error, fs};
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Please provide a file path.");
        eprintln!("Usage: cargo run <filename>");
        std::process::exit(1);
    }
    let file_path = &args[1];
    let text = fs::read_to_string(file_path)?;

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
    let home = std::env::var("HOME").unwrap();
    fs::create_dir_all(format!("{}/.HTB/steps/", home))?;
    fs::write(format!("{}/.HTB/steps/step0.temp", home), out)?;
    Ok(())
}
