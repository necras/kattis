use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .skip(1)
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    for mut line in lines {
        line = line.trim().to_lowercase();
        let mut missing = String::new();
        for char in 'a'..='z' {
            if !line.contains(char) {
                missing.push(char);
            }
        }
        if missing.is_empty() {
            println!("pangram");
        } else {
            println!("missing {}", missing);
        }
    }
}
