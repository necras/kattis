use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("could not read line from stdin");

    let mut result = String::new();

    for c in input.trim().chars() {
        match c {
            '<' => {
                result.pop();
            }
            _ => {
                result.push(c);
            }
        }
    }

    println!("{}", result);
}
