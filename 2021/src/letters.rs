use std::collections::{HashMap, LinkedList};

const STYLED_CHAR_LENGTH: usize = 4;

pub struct LetterUtil {
  letters: HashMap<String, char>,
}

impl LetterUtil {
  pub fn from_source(source: String) -> Result<LetterUtil, String> {
    let mut letters: HashMap<String, char> = HashMap::new();
    let mut parse_char: Option<char> = None;
    let mut image: String = String::new();
    for line in source.split("\n") {
      match parse_char {
        Some(c) => {
          if line.trim().len() == 0 {
            letters.insert(image, c);
            image = String::new();
            parse_char = None;
          } else {
            image.push_str(line);
            image.push('\n');
          }
        }
        None => parse_char = line.chars().nth(0),
      }
    }

    Ok(LetterUtil { letters })
  }

  pub fn interpret(&self, styled: String) -> Result<String, String> {
    let mut chunks: LinkedList<String> = LinkedList::new();
    let mut current_chunk: String = String::new();
    let mut first_line = true;
    let mut i = 0;
    for c in styled.chars() {
      if i < STYLED_CHAR_LENGTH {
        current_chunk.push(c);
        i += 1;
      } else if i == STYLED_CHAR_LENGTH {
        current_chunk.push('\n');
        chunks.push_back(current_chunk);
        if c == '\n' {
          first_line = false;
        }
        if first_line {
          current_chunk = String::new();
        } else {
          current_chunk = chunks
            .pop_front()
            .ok_or_else(|| format!("Mismatched line lengths"))?;
        }
        i = 0;
      }
    }

    chunks.push_front(current_chunk);
    let mut result: String = String::new();
    for chunk in chunks {
      result.push(
        self
          .letters
          .get(&chunk)
          .cloned()
          .ok_or_else(|| format!("Formatted character not found in resource map"))?,
      );
    }

    Ok(result)
  }
}
