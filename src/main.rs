extern crate rand;
extern crate chrono;

use rand::Rng;
use rand::thread_rng;
use chrono::Utc;

fn key_generator(key_len: usize) -> String {
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = thread_rng();
    (0..key_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

// validate user key or generate one
fn key_offset(key: Option<String>) -> String {
    const KEY_LEN: usize = 5;
    let user_key = key.unwrap_or_default();
    if user_key.len() == KEY_LEN && user_key.chars().all(|c| c.is_numeric()) {
        return user_key
    }
    key_generator(KEY_LEN)
}

// validate user date or generate one: ddmmyy format
fn date_offset(date: Option<String>) -> String {
    let d = date.unwrap_or(Utc::today().format("%d%m%y").to_string());
    let squared = d.parse::<u64>().unwrap().pow(2);
    (squared % 10_000).to_string() // last 4 digits of squared date
}

fn get_base_ord(byte: u8) -> u8 {
    if byte >= 'a' as u8 { return 97 }; // lower case
    if byte >= 'A' as u8 { return 65 }; // uppper case
    32 // default for punctuation
}

fn rotate(byte: u8, places: u8) -> u8 {
    let mut result = byte;

    if byte.is_ascii() && byte <= 'z' as u8 { // 122 is 'z', highest possible ord
        let modulo = |a, b| { ((a % b) + b) % b };

        let base_ord = get_base_ord(byte);
        let rotation = (byte - base_ord) + places;
        let scale = if byte.is_ascii_alphabetic() { 26 } else { 32 };

        result = modulo(rotation, scale) + base_ord;
    }

    result
}

fn main() {
    // TODO: get optional user input
    let key = key_offset(None);
    let date = date_offset(None);

    let before = 'a';
    let after = rotate(before as u8, 25);

    println!("{} - {}", key, date);
    println!("{} - {}", before, after as char);
}


#[cfg(test)]
mod tests {
    use super::*;

    // key_generator
    #[test]
    fn generates_five_digit_string() {
        let key_len = 5;
        let key_one = key_generator(key_len);
        assert!(key_one.chars().all(|c| c.is_numeric()));
        assert_eq!(key_one.len(), key_len);
    }
}


