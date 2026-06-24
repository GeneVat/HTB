use std::{fs, path::PathBuf};

fn indent_level(line: &str) -> usize {
    line.chars()
        .take_while(|c| c.is_whitespace() && *c != '\n' && *c != '\r')
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = PathBuf::from("steps/step2.temp");
    let output_path = PathBuf::from("steps/step3.temp");

    let input = fs::read_to_string(&input_path)?;

    let mut stack: Vec<String> = Vec::new();
    let mut out = String::new();
    let mut prev_indent = 0usize;

    for line in input.lines() {
        let cur_indent = indent_level(line);

        // If indentation decreased, close tags until we match the previous structure.
        // This assumes one tag level corresponds to a consistent indentation unit.
        // We approximate by closing until stack size corresponds to indentation “levels”.
        // Tweak `unit` if your file uses a different indentation width.
        let unit = 4usize;
        let target_levels = cur_indent / unit;
        while stack.len() > target_levels {
            if let Some(tag) = stack.pop() {
                out.push_str(&format!("[/{}]\n", tag));
            }
        }

        let bytes = line.as_bytes();
        let mut i = 0usize;

        while i < bytes.len() {
            // find '['
            let slice = &line[i..];
            let open_rel = match slice.find('[') {
                Some(p) => p,
                None => break,
            };
            let open = i + open_rel;

            // copy text before '['
            out.push_str(&line[i..open]);

            // find closing ']'
            let close_rel = match line[open..].find(']') {
                Some(p) => p,
                None => {
                    out.push('[');
                    i = open + 1;
                    continue;
                }
            };
            let close = open + close_rel;

            let token = line[open + 1..close].trim();

            if token.is_empty() {
                out.push_str(&line[open..=close]);
                i = close + 1;
                continue;
            }

            let is_closing = token.starts_with('/');
            if is_closing {
                let name = token[1..].trim();
                // If the input has explicit closing tags, honor them.
                if let Some(top) = stack.last() {
                    if top == name {
                        stack.pop();
                    }
                }

                out.push('[');
                out.push('/');
                out.push_str(name);
                out.push(']');
            } else {
    let name = token.split('=').next().unwrap_or("").trim();
    stack.push(name.to_string());
    out.push('[');
    out.push_str(token);
    out.push(']');
}


            i = close + 1;
        }

        // copy any trailing text after last '['
        if i < line.len() {
            out.push_str(&line[i..]);
        }

        out.push('\n');
        prev_indent = cur_indent;
    }

    // Close anything still open at end of file (this should be minimal now).
    while let Some(tag) = stack.pop() {
        out.push_str(&format!("[/{}]\n", tag));
    }

    fs::write(&output_path, out)?;
    Ok(())
}
