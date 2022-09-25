use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut vec: Vec<String> = lines.skip(1).collect();
    vec.reverse();

    for v in vec {
        println!("{v}");
    }
}
