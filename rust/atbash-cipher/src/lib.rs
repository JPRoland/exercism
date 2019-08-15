/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    atbash(plain)
        .enumerate()
        .flat_map(|(i, c)| if i % 5 == 0 { vec![' ', c] } else { vec![c] }.into_iter())
        .skip(1)
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    atbash(cipher).collect()
}

fn transcode_char(c: char) -> char {
    if c.is_ascii_digit() {
        c
    } else {
        (b'z' + b'a' - c as u8) as char
    }
}

fn atbash<'a>(s: &'a str) -> impl Iterator<Item = char> + 'a {
    s.chars().filter_map(|c| {
        Some(c)
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .map(transcode_char)
    })
}
