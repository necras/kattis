use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines().skip(1).map(|l| l.unwrap()) {
        let k: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let n = k[1];

        let s1: i32 = (0..n + 1).sum();

        let s2: i32 = (0..n * 2).filter(|x| x % 2 == 1).sum();

        let s3: i32 = (0..n * 2 + 1).filter(|x| x % 2 == 0).sum();

        println!("{} {} {} {}", k[0], s1, s2, s3);
    }
}
