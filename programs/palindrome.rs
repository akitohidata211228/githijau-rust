// palindrome.rs
// Cek apakah sebuah kata palindrom.

fn is_palindrome(text: &str) -> bool {
    let cleaned: Vec<char> = text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    let reversed: Vec<char> = cleaned.iter().rev().cloned().collect();
    cleaned == reversed
}

fn main() {
    for s in ["Racecar", "Hello", "Kasur ini rusak"] {
        println!("{} -> {}", s, is_palindrome(s));
    }
}
