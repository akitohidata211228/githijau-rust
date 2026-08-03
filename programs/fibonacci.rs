// fibonacci.rs
// Deret Fibonacci: 15 suku pertama.

fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let terms: Vec<String> = (0..15).map(|i| fibonacci(i).to_string()).collect();
    println!("{}", terms.join(", "));
}
