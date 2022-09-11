use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("could not read line from stdin");

    match input.trim() {
        "OCT 31" => println!("yup"),
        "DEC 25" => println!("yup"),
        _ => println!("nope")
    }
}
