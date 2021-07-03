extern crate rand;

use rand::Rng;
use rand::thread_rng;

fn key_generator() -> String {
    const CHARSET: &[u8] = b"0123456789";
    const KEY_LEN: usize = 5;
    let mut rng = thread_rng();
    (0..KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn main() {
    let key = key_generator();
    println!("{}", key)
}


#[cfg(test)]
mod tests {
    use super::*;

    // key_generator
    #[test]
    fn generates_five_digit_string() {
        let key_one = key_generator();
        assert!(key_one.chars().all(|c| c.is_numeric()));
    }
}


