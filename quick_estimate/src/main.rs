use std::io::BufRead;

fn read_input() -> (usize, Vec<String>) {
    let stdin = std::io::stdin();
    let n: usize = stdin
        .lock()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("could not read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");

    let input = stdin
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    (n, input)
}

fn main() {
    let (n, lines) = read_input();

    for (i, _) in lines.iter().enumerate().take(n) {
        println!("{}", lines[i].len());
    }
}
