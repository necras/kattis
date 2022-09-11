use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("could not read line from stdin");

    let mut white_space: f64 = 0.0;
    let mut lowercase: f64 = 0.0;
    let mut uppercase: f64 = 0.0;
    let mut symbols: f64 = 0.0;

    let str = input.trim();
    let len = str.chars().count() as f64;

    for c in str.chars() {
        if c == '_' {
            white_space += 1.0;
        } else if c.is_lowercase() {
            lowercase += 1.0;
        } else if c.is_uppercase() {
            uppercase += 1.0;
        } else {
            symbols += 1.0;
        }
    }

    println!(
        "{:.15}\n{:.15}\n{:.15}\n{:.15}",
        white_space / len,
        lowercase / len,
        uppercase / len,
        symbols / len
    );
}
