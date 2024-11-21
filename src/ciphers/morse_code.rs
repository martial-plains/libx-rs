use alloc::{string::String, vec::Vec};

#[must_use]
pub fn char_to_morse(c: char) -> &'static str {
    match c {
        'a' => ".-",
        'b' => "-...",
        'c' => "-.-.",
        'd' => "-..",
        'e' => ".",
        'f' => "..-.",
        'g' => "--.",
        'h' => "....",
        'i' => "..",
        'j' => ".---",
        'k' => "-.-",
        'l' => ".-..",
        'm' => "--",
        'n' => "-.",
        'o' => "---",
        'p' => ".--.",
        'q' => "--.-",
        'r' => ".-.",
        's' => "...",
        't' => "-",
        'u' => "..-",
        'v' => "...-",
        'w' => ".--",
        'x' => "-..-",
        'y' => "-.--",
        'z' => "--..",
        '1' => ".----",
        '2' => "..---",
        '3' => "...--",
        '4' => "....-",
        '5' => ".....",
        '6' => "-....",
        '7' => "--...",
        '8' => "---..",
        '9' => "----.",
        '0' => "-----",
        _ => panic!("Found invalid character: {c}"),
    }
}

#[must_use]
#[warn(clippy::comparison_chain)]
pub fn morse_to_char(s: &str) -> char {
    match s {
        ".-" => 'a',
        "-..." => 'b',
        "-.-." => 'c',
        "-.." => 'd',
        "." => 'e',
        "..-." => 'f',
        "--." => 'g',
        "...." => 'h',
        ".." => 'i',
        ".---" => 'j',
        "-.-" => 'k',
        ".-.." => 'l',
        "--" => 'm',
        "-." => 'n',
        "---" => 'o',
        ".--." => 'p',
        "--.-" => 'q',
        ".-." => 'r',
        "..." => 's',
        "-" => 't',
        "..-" => 'u',
        "...-" => 'v',
        ".--" => 'w',
        "-..-" => 'x',
        "-.--" => 'y',
        "--.." => 'z',
        ".----" => '1',
        "..---" => '2',
        "...--" => '3',
        "....-" => '4',
        "....." => '5',
        "-...." => '6',
        "--..." => '7',
        "---.." => '8',
        "----." => '9',
        "-----" => '0',
        _ => panic!("Found invalid Morse code: {s}"),
    }
}

pub fn encrypt(text: &str) -> String {
    let mut encrypted_text = String::new();

    for c in text.chars() {
        encrypted_text.push_str(char_to_morse(c));
        encrypted_text.push(' ');
    }

    encrypted_text
}

#[must_use]
pub fn decrypt(text: &str) -> String {
    let mut decrypted_text = String::new();

    let splits = text
        .split(char::is_whitespace)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    for s in splits {
        decrypted_text.push(morse_to_char(s));
    }

    decrypted_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_morse() {
        let text = "01234567890";
        let encrypted = encrypt(text);
        let decrypted = decrypt(&encrypted);
        assert_eq!(text, decrypted);

        let text = "abcdefghijklmnopqrstuvwxyz";
        let encrypted = encrypt(text);
        let decrypted = decrypt(&encrypted);
        assert_eq!(text, decrypted);
    }
}
