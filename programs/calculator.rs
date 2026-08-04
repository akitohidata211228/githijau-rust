// calculator.rs
// Kalkulator dua angka.

fn calculate(a: f64, b: f64, op: char) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err("pembagian nol".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err("operasi tidak dikenal".to_string()),
    }
}

fn main() {
    match calculate(12.0, 3.0, '/') {
        Ok(res) => println!("12 / 3 = {}", res),
        Err(e) => println!("Error: {}", e),
    }
}
