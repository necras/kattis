use std::collections::BTreeMap;
use std::io::BufRead;

fn no_duplicates() -> String {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("could not read line from stdin");

    let mut counts = BTreeMap::new();

    for word in input.split_whitespace() {
        if counts.contains_key(&word) {
            return "no".to_string();
        }
        *counts.entry(word).or_insert(0usize) += 1;
    }

    "yes".to_string()
}

fn main() {
    let res = no_duplicates();
    println!("{}", res);
}
