use hex;
use std::borrow::Cow;

fn extrapolate_key(base_key: &str, target_len: usize) -> Vec<u8> {
    let key_b = base_key.as_bytes();
    let multiplier = target_len / key_b.len();

    let mut key_ext: Vec<u8> = Vec::with_capacity(key_b.len());
    for _ in 0..multiplier {
        key_ext.extend_from_slice(&key_b)
    }
    let rest = target_len - key_ext.len();
    if rest > 0 {
        key_ext.extend_from_slice(&key_b[..rest]);
    }

    key_ext
}

fn cipher(text: &str, key: &str) -> Vec<u8> {
    let text_b = Vec::from(text.as_bytes());
    let key_b = extrapolate_key(key, text.len());

    let mut cipher: Vec<u8> = Vec::with_capacity(text_b.len());
    for i in 0..text_b.len() {
        cipher.push(text_b[i] ^ key_b[i])
    }

    cipher
}

fn one_pad(plaintext: &str, key: &str) -> String {
    let cipher_bytes = cipher(plaintext, key);
    let ciphertext = match String::from_utf8_lossy(&cipher_bytes) {
        Cow::Owned(v) => v,
        Cow::Borrowed(v) => v.to_string(),
    };
    let _cipher_hex = hex::encode(&cipher_bytes).to_uppercase();

    ciphertext
}

pub fn run() {
    let plaintext = "Hello World!";
    let enc_key = "Goodbye";
    let encrypted = one_pad(plaintext, enc_key);
    let decrypted = one_pad(&encrypted, enc_key);
    println!("---------------------------------");
    println!("plaintext: {}", plaintext);
    println!("encrypted: {:?}", encrypted);
    println!("decrypted: {}", decrypted);
    println!("---------------------------------");
}
