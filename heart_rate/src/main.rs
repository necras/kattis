use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines().skip(1).map(|l| l.unwrap()) {
        let n: Vec<f32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let b = n[0];
        let p = n[1];

        let abpm = 60.0 * b / p;
        let var = 60.0 / p;

        println!("{:.4} {:.4} {:.4}", abpm - var, abpm, abpm + var);
    }
}
