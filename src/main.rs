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

fn main() {
    // TODO: get optional user input
    let key = key_offset(None);
    let date = date_offset(None);
    println!("{} - {}", key, date)
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


