use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("steps/step3.temp")?;

    let style_re = Regex::new(r"\[(.*?)\]")?;

    let words = ["html","head","body","main","p","h4"];
    
    let words_re = Regex::new(&format!(r"({})", words.join("|")))?;

    let mut out: String;

    out = style_re
        .replace_all(&text, |caps: &regex::Captures| {
            let raw = caps.get(1).map(|m| m.as_str()).unwrap_or("");

            if words_re.is_match(raw) {
                String::new() 
            } else {
                caps.get(0).unwrap().as_str().to_string() 
            }
        })
        .into_owned();
    out = out.replace("[title","[size=180");
    out = out.replace("[/title","[/size");
    out = out.replace("[h1","[size=160");
    out = out.replace("[h2","[size=140");
    out = out.replace("[h3","[size=120");
    out = out.replace("[/h1","[/size");
    out = out.replace("[/h2","[/size");
    out = out.replace("[/h3","[/size");

    fs::write("main.bb", out)?;
    Ok(())
}
