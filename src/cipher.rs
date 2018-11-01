use rayon::prelude::*;

lazy_static! {
    static ref ALPHABET: Vec<char> = "abcdefghijklmnopqrstuvwxyz"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    static ref LENGTH: u32 = ALPHABET.len() as u32;
}

fn normalize_key(key: String, text_length: usize) -> String {
    let key_length = if key.len() > 0 { key.len() } else { 1 };

    if key_length > text_length {
        key
    } else {
        let ratio = text_length / key_length;
        let fill = text_length % key_length;

        format!(
            "{}{}",
            key.repeat(ratio),
            key.chars()
                .collect::<Vec<char>>()
                .iter()
                .take(fill)
                .collect::<String>()
        )
    }
}

#[allow(dead_code)]
pub fn encode(text: String, key: String) -> String {
    let normalized_key = normalize_key(key, text.len());

    text.chars()
        .collect::<Vec<char>>()
        .par_iter()
        .zip(normalized_key.chars().collect::<Vec<char>>())
        .map(|(character, key)| {
            if *character == ' ' {
                return ' ';
            }

            let (from_text, from_key) = ((*character as i32) % 97, (key as i32) % 97);
            if let Some(encoded) = ALPHABET.get(((from_text + from_key) as i32 % *LENGTH as i32) as usize)
            {
                *encoded
            } else {
                'a'
            }
        }).collect::<String>()
}

#[allow(dead_code)]
pub fn decode(text: String, key: String) -> String {
    let normalized_key = normalize_key(key, text.len());

    text.chars()
        .collect::<Vec<char>>()
        .par_iter()
        .zip(normalized_key.chars().collect::<Vec<char>>())
        .map(|(character, key)| {
            if *character == ' ' {
                return ' ';
            }

            let (from_text, from_key) = ((*character as u32) % 97, (key as u32) % 97);
            if let Some(encoded) =
                ALPHABET.get((((from_text.wrapping_sub(from_key) as u32).wrapping_add(*LENGTH)) % *LENGTH) as usize)
            {
                *encoded
            } else {
                'a'
            }
        }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let word = "lorem".to_string();
        assert_eq!(encode(word.clone(), "".to_string()), "".to_string());
        assert_eq!(encode(word.clone(), "aa".to_string()), word);
        assert_eq!(encode(word.clone(), "z".to_string()), "knqdl".to_string());

        let word = "lorem lorem".to_string();
        assert_eq!(encode(word.clone(), "".to_string()), "".to_string());
        assert_eq!(encode(word.clone(), "aa".to_string()), word);
        assert_eq!(
            encode(word.clone(), "z".to_string()),
            "knqdl knqdl".to_string()
        );
    }

    #[test]
    fn test_decode() {
        let word = "knqdl".to_string();
        assert_eq!(decode(word.clone(), "".to_string()), "".to_string());
        assert_eq!(decode(word.clone(), "aa".to_string()), word);
        assert_eq!(decode(word.clone(), "z".to_string()), "lorem".to_string());

        let word = "knqdl knqdl".to_string();
        assert_eq!(decode(word.clone(), "".to_string()), "".to_string());
        assert_eq!(decode(word.clone(), "aa".to_string()), word);
        assert_eq!(
            decode(word.clone(), "z".to_string()),
            "lorem lorem".to_string()
        );
    }
}
