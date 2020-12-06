pub fn is_palindrome(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }
    let mut forward = string.char_indices();
    let mut backward = string.char_indices().rev();
    loop {
        let (forward_index, forward_char) = forward.next().unwrap();
        let (backward_index, backward_char) = backward.next().unwrap();
        // Different pair of characters
        if forward_char != backward_char {
            return false;
        }
        // We're done and found no mismatch pair
        if forward_index >= backward_index {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string::*;

    #[test]
    fn test_is_palindrome() {
        // Empty
        assert!(is_palindrome(""));
        // Single char
        assert!(is_palindrome("1"));
        // Even number of chars
        assert!(is_palindrome("11"));
        assert!(is_palindrome("1221"));
        // Odd number of chars
        assert!(is_palindrome("111"));
        assert!(is_palindrome("121"));
        // Not palindrome
        assert!(!is_palindrome("112"));
        assert!(!is_palindrome("1232"));
        // Unicode palindrome
        assert!(is_palindrome("mềm"));
        assert!(is_palindrome("áẳậẳá"));
        // Not a Unicode palindrome
        assert!(!is_palindrome("mề"));
    }
}
