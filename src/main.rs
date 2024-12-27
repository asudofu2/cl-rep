use arboard::Clipboard;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

const PATTERN_FILE_NAME: &str = "cl-rep-patt.json";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = PATTERN_FILE_NAME)]
    pattern_file: String,
}

fn main() {
    let args = Args::parse();
    match replace_clip_string(&args.pattern_file) {
        Ok(msg) => {
            println!("{msg}")
        }
        Err(e) => eprintln!("Error: {e}"),
    };
}

#[derive(Serialize, Deserialize)]
struct ReplacePattern {
    src: String,
    dst: String,
}

pub fn replace_clip_string(pattern_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pattern_list = read_pattern_list(pattern_file)?;

    let mut clipboard = Clipboard::new()?;
    let src_text = clipboard.get_text()?;

    let dst_text = replace(&pattern_list, &src_text)?;

    if src_text == dst_text {
        return Ok(String::from("Not found"));
    }

    clipboard.set_text(&dst_text)?;

    Ok(format!("{src_text} to {dst_text}"))
}

fn replace(pattern_list: &[ReplacePattern], src_text: &str) -> Result<String, Box<dyn Error>> {
    let mut dst_text = src_text.to_string();
    for pattern in pattern_list {
        if dst_text.contains(&pattern.src) {
            dst_text = dst_text.replace(&pattern.src, &pattern.dst);
            break;
        }
    }

    // println!("src: {src_text}, dst: {dst_text}");

    Ok(dst_text)
}

fn read_pattern_list(path: &str) -> Result<Vec<ReplacePattern>, Box<dyn std::error::Error>> {
    let f = File::open(path)?;
    let replace_pattern_list = serde_json::from_reader(f)?;
    Ok(replace_pattern_list)
}

#[cfg(test)]
mod tests {

    #[test]
    fn multiple_replace() {
        let pattern_list = vec![
            super::ReplacePattern {
                src: "a".to_string(),
                dst: "b".to_string(),
            },
            super::ReplacePattern {
                src: "b".to_string(),
                dst: "a".to_string(),
            },
        ];

        let src_text = "a";
        let dst_text = super::replace(&pattern_list, src_text).unwrap();

        assert_eq!("b", dst_text);
    }
}
