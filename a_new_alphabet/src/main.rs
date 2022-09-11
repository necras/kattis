use std::fmt::Write;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("could not read line from stdin");

    let mut translation = String::new();
    let input = input.trim().to_lowercase();
    for char in input.chars() {
        match char {
            'a' => write!(&mut translation, "@").expect("could not handle 'a'"),
            'b' => write!(&mut translation, "8").expect("could not handle 'b'"),
            'c' => write!(&mut translation, "(").expect("could not handle 'c'"),
            'd' => write!(&mut translation, "|)").expect("could not handle 'd'"),
            'e' => write!(&mut translation, "3").expect("could not handle 'e'"),
            'f' => write!(&mut translation, "#").expect("could not handle 'f'"),
            'g' => write!(&mut translation, "6").expect("could not handle 'g'"),
            'h' => write!(&mut translation, "[-]").expect("could not handle 'h'"),
            'i' => write!(&mut translation, "|").expect("could not handle 'i'"),
            'j' => write!(&mut translation, "_|").expect("could not handle 'j'"),
            'k' => write!(&mut translation, "|<").expect("could not handle 'k'"),
            'l' => write!(&mut translation, "1").expect("could not handle 'l'"),
            'm' => write!(&mut translation, "[]\\/[]").expect("could not handle 'm'"),
            'n' => write!(&mut translation, "[]\\[]").expect("could not handle 'n'"),
            'o' => write!(&mut translation, "0").expect("could not handle 'o'"),
            'p' => write!(&mut translation, "|D").expect("could not handle 'p'"),
            'q' => write!(&mut translation, "(,)").expect("could not handle 'q'"),
            'r' => write!(&mut translation, "|Z").expect("could not handle 'r'"),
            's' => write!(&mut translation, "$").expect("could not handle 's'"),
            't' => write!(&mut translation, "']['").expect("could not handle 't'"),
            'u' => write!(&mut translation, "|_|").expect("could not handle 'u'"),
            'v' => write!(&mut translation, "\\/").expect("could not handle 'v'"),
            'w' => write!(&mut translation, "\\/\\/").expect("could not handle 'w'"),
            'x' => write!(&mut translation, "}}{{").expect("could not handle 'x'"),
            'y' => write!(&mut translation, "`/").expect("could not handle 'y'"),
            'z' => write!(&mut translation, "2").expect("could not handle 'z'"),
            _ => translation.push(char),
        }
    }
    println!("{}", translation);
}
