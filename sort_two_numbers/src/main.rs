use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("could not read line from stdin");

    let mut line: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    line.sort();
    println!("{} {}", line[0], line[1]);
}
