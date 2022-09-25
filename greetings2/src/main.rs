use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("could not read line from stdin");

    let result = input.chars().fold(String::new(), |mut res, c| {
        if c == 'e' {
            res.extend(&[c, 'e']);
        } else {
            res.push(c);
        }
        res
    });

    println!("{}", result.trim());
}
