use std::cmp::max;
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

    let side_len = next_num();
    let horz_cut = next_num();
    let vert_cut = next_num();

    let h = max(horz_cut, side_len - horz_cut);
    let w = max(vert_cut, side_len - vert_cut);

    println!("{}", h * w * 4);
}
