use std::iter::FromIterator;

pub struct CaesarCipher {
    key: usize,
    alphabet: Vec<char>,
}

pub trait Cipher {
    fn encrypt(&self, message: &str) -> String;
    fn decrypt(&self, message: &str) -> String;
}

impl CaesarCipher {
    // Constructors
    pub fn new(key: usize, alphabet: &str) -> Self {
        let mut alphabet = Vec::from_iter(alphabet.chars());
        alphabet.sort();
        Self { key, alphabet }
    }
    // Helper methods
    fn encrypt_char(&self, ch: char) -> char {
        match self.alphabet.binary_search(&ch) {
            Ok(code) => {
                let index = (code + self.key) % self.alphabet.len();
                self.alphabet[index]
            }
            // Characters outside the alphabet are kept as-is
            Err(_) => ch,
        }
    }
    fn decrypt_char(&self, ch: char) -> char {
        match self.alphabet.binary_search(&ch) {
            Ok(code) => {
                // Subtract first to avoid overflow in case
                // code + alphabet.len() is too large
                let index = (code + (self.alphabet.len() - self.key))
                    % self.alphabet.len();
                self.alphabet[index]
            }
            // Characters outside the alphabet are kept as-is
            Err(_) => ch,
        }
    }
}

impl Cipher for CaesarCipher {
    fn encrypt(&self, message: &str) -> String {
        let mut result = String::new();
        for ch in message.chars() {
            result.push(self.encrypt_char(ch));
        }
        result
    }
    fn decrypt(&self, message: &str) -> String {
        let mut result = String::new();
        for ch in message.chars() {
            result.push(self.decrypt_char(ch));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::cryptology::*;

    #[test]
    fn test_caesar_encrypt() {
        let cipher = CaesarCipher::new(3, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!(
            cipher.encrypt("THIS IS A HELLO WORLD ZZZ!"),
            "WKLV LV D KHOOR ZRUOG CCC!"
        );
    }

    #[test]
    fn test_caesar_decrypt() {
        let cipher = CaesarCipher::new(3, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!(
            cipher.decrypt("WKLV LV D KHOOR ZRUOG CCC!"),
            "THIS IS A HELLO WORLD ZZZ!"
        );
    }
}
