use arboard::Clipboard;
use serde::{Deserialize, Serialize};
use std::fs::File;

const PATTERN_FILE_NAME: &str = "cl-rep-patt.json";

fn main() {
    match replace_clip_string() {
        Ok(msg) => {println!("{msg}")},
        Err(e) => eprintln!("Error: {e}"),
    };
}

#[derive(Serialize, Deserialize)]
struct ReplacePattern {
    src: String,
    dst: String,
}

pub fn replace_clip_string() -> Result<String, Box<dyn std::error::Error>> {
    let pattern_list = read_pattern_list(PATTERN_FILE_NAME)?;
    
    let mut clipboard = Clipboard::new()?;
    let src_text = clipboard.get_text()?;
    let mut dst_text = src_text.clone();
    for pattern in pattern_list {
        dst_text = dst_text.replace(&pattern.src, &pattern.dst);
    }

    // println!("src: {src_text}, dst: {dst_text}");
    
    if src_text == dst_text {
        return Ok(String::from("Not found"))
    }

    clipboard.set_text(&dst_text)?;
    
    Ok(format!("{src_text} to {dst_text}"))
}


fn read_pattern_list(path: &str) -> Result<Vec<ReplacePattern>, Box<dyn std::error::Error>> {
    let f = File::open(path)?;
    let replace_pattern_list = serde_json::from_reader(f)?;
    Ok(replace_pattern_list)
}

#[cfg(test)]
mod tests {
    use crate::read_pattern_list;

    #[test]
    fn read_pattern_file() {
        let res = read_pattern_list(super::PATTERN_FILE_NAME).unwrap();

        assert_eq!(2, res.len());
        assert_eq!("a src string", res[0].src);
        assert_eq!("a dst string", res[0].dst);
        assert_eq!("b src string", res[1].src);
        assert_eq!("b dst string", res[1].dst);
    }
}
