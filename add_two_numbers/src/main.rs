use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("could not read line from stdin");

    let mut split = input.split_whitespace();
    let mut next_num = || -> usize {
        split
            .next()
            .expect("not enough input numbers")
            .parse()
            .expect("given input is not a number")
    };

    let a = next_num();
    let b = next_num();

    println!("{}", a + b);
}
