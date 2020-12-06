use std::cmp::Ordering::Equal;

pub fn count_english_vowel(string: &str) -> usize {
    string.chars().fold(0, |current, ch| match ch {
        'a' | 'e' | 'o' | 'u' | 'i' => current + 1,
        _ => current,
    })
}

// log10(2^128) is about 38.5, largest decimal digit is 9,
// 39 * 9 = 351, so 2 bytes is enough
pub fn sum_digits(n: u128) -> u16 {
    let mut sum = 0;
    let mut n = n;
    while n != 0 {
        sum += (n % 10) as u16;
        n /= 10;
    }
    sum
}

pub fn is_triangle(a: f64, b: f64, c: f64) -> bool {
    if a.is_sign_negative() || b.is_sign_negative() || c.is_sign_negative() {
        panic!("Negative length provided: {}, {}, {}", a, b, c);
    }
    let mut squared_sides = [a * a, b * b, c * c];
    squared_sides.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    squared_sides[0] + squared_sides[1] >= squared_sides[2]
}

pub fn is_right_triangle(a: f64, b: f64, c: f64) -> bool {
    if a.is_sign_negative() || b.is_sign_negative() || c.is_sign_negative() {
        panic!("Negative length provided: {}, {}, {}", a, b, c);
    }
    let mut squared_sides = [a * a, b * b, c * c];
    squared_sides.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    squared_sides[0] + squared_sides[1] == squared_sides[2]
}

pub fn fibonacci(n: usize) -> Vec<u128> {
    // TODO: Implement more sophisticate version (if any)
    let mut result = Vec::with_capacity(n);
    result.push(1);
    result.push(1);
    let mut f1 = 1;
    let mut f2 = 1;
    for _ in 2..n {
        let fib = f1 + f2;
        f1 = f2;
        f2 = fib;
        result.push(fib);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::unsorted::*;

    #[test]
    fn test_count_english_vowel() {
        assert_eq!(count_english_vowel("abcdefghijklmnopqrstuvwxyz"), 5);
    }

    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits(0), 0);
        assert_eq!(sum_digits(100), 1);
        assert_eq!(sum_digits(1212), 6);
        assert_eq!(sum_digits(1402), 7);
        assert_eq!(sum_digits(12345), 15);
    }

    #[test]
    fn test_is_triangle() {
        // Scalene
        assert!(is_triangle(7.0, 4.0, 6.0));
        // Isosceles
        assert!(is_triangle(6.0, 6.0, 4.0));
        // Equilateral
        assert!(is_triangle(6.0, 6.0, 6.0));
        // Right
        assert!(is_triangle(3.0, 4.0, 5.0));
        // 3 points on a line
        assert!(!is_triangle(3.0, 4.0, 7.0));
        // Impossible triangle
        assert!(!is_triangle(1.0, 2.0, 4.0));
    }
    #[test]
    #[should_panic]
    fn test_is_triangle_panic_on_negative_input() {
        is_triangle(-1.0, 2.0, 4.0);
    }

    #[test]
    fn test_is_right_triangle() {
        // Scalene
        assert!(!is_right_triangle(7.0, 4.0, 6.0));
        // Isosceles
        assert!(!is_right_triangle(6.0, 6.0, 4.0));
        // Equilateral
        assert!(!is_right_triangle(6.0, 6.0, 6.0));
        // Right
        assert!(is_right_triangle(3.0, 4.0, 5.0));
        // 3 points on a line
        assert!(!is_right_triangle(3.0, 4.0, 7.0));
        // Impossible triangle
        assert!(!is_right_triangle(1.0, 2.0, 4.0));
    }
    #[test]
    #[should_panic]
    fn test_is_right_triangle_panic_on_negative_input() {
        is_right_triangle(-3.0, 4.0, 5.0);
    }
}
