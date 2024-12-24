use hex;
use std::borrow::Cow;

fn extend_key(base_key: &u16, target_len: usize) -> Vec<u8> {
    let key_b = base_key.to_be_bytes();
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

fn cipher(text: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut cipher: Vec<u8> = Vec::with_capacity(text.len());
    for i in 0..text.len() {
        cipher.push(text[i] ^ key[i])
    }

    cipher
}

fn vigenere(plaintext: &str, key: u16) -> String {
    let text_b = Vec::from(plaintext.as_bytes());
    let key_ext = extend_key(&key, text_b.len());

    let cipher_b = cipher(&text_b, &key_ext);
    let cipher_text = match String::from_utf8_lossy(&cipher_b) {
        Cow::Owned(v) => v,
        Cow::Borrowed(v) => v.to_string(),
    };
    let _cipher_hex = hex::encode(&cipher).to_uppercase();

    cipher_text
}

fn main() {
    let plaintext = "Hello!";
    let enc_key: u16 = 0xA12F;
    let encrypted = vigenere(plaintext, enc_key);
    println!("---------------------------------");
    println!("plaintext: {}", plaintext);

    println!("encrypted: {}", encrypted);
    println!("---------------------------------");
}
