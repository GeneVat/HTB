use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("steps/step1.temp")?;

    let re = Regex::new(r"(?im)^(?P<indent>\s*)(?:-\s*)?(?:\[(?P<r>[a-z0-9]+)\]\s*)?-\s*(?P<r2>[a-z0-9]+)(?P<rest>.*)$")?;

    let out = re.replace_all(&text, |caps: &regex::Captures| {
        let indent = &caps["indent"];
        let r = if caps.name("r").is_some() { &caps["r"] } else { &caps["r2"] };
        format!("{}[{}]{}", indent, r, &caps["rest"])
    });

    fs::write("steps/step2.temp", out.to_string())?;
    Ok(())
}
