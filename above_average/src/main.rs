use std::io::BufRead;
use std::convert::TryInto;

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines().skip(1).map(|l| l.unwrap()) {
        let n: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let divisor = n[0];

        let test_scores: i32 = n.iter().skip(1).sum();
        let average = test_scores / divisor;

        let above_average: i32 = n
            .iter()
            .skip(1)
            .filter(|x| *x > &average)
            .count()
            .try_into()
            .unwrap();

        let percentage: f32 = ((above_average as f32) / (divisor as f32)) * 100.0;

        println!("{:.3}%", percentage);
    }
}
