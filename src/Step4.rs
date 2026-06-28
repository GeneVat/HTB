use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let home = std::env::var("HOME").unwrap();
    let text = fs::read_to_string(format!("{}/.HTB/steps/step3.temp",home))?;

    let style_re = Regex::new(r"\[(.*?)\]")?;

    let words = ["html","head","body","main","h4","nav","id","section","footer","hr","email"];
    
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
    out = out.replace("href","url");
    out = out.replace("[a]","");
    out = out.replace("[/a]","");
    out = out.replace("[p]","");
    out = out.replace("[/p]","");
    out = out.replace("ul]","list]");
    out = out.replace("li]","*]");
    out = out.replace("[/*]","");

    fs::write("main.bb", out.clone())?;
    for entry in fs::read_dir(format!("{}/.HTB/steps", home))? {
        fs::remove_file(entry?.path())?;
    }
    Ok(())
}
