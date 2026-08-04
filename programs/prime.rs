// prime.rs
// Bilangan prima 1-100.

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    for i in 1..=100 {
        if is_prime(i) {
            print!("{} ", i);
        }
    }
    println!();
}
