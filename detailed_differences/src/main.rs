use std::io::BufRead;

fn main() {
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

    let lines = stdin
        .lock()
        .lines()
        .map(|x| x.expect("could not map from stdin"))
        .collect::<Vec<String>>();

    for i in (0..n * 2).step_by(2) {
        let s1 = lines[i].as_str();
        let s2 = lines[i + 1].as_str();
        let mut result = String::from("");

        if s1.chars().count() == s2.chars().count() {
            let len = s1.chars().count();
            for j in 0..len {
                let c1 = s1
                    .chars()
                    .nth(j)
                    .expect(&format!("could not get the {:?}th char", j));
                let c2 = s2
                    .chars()
                    .nth(j)
                    .expect(&format!("could not get the {:?}th char", j));
                if c1 != c2 {
                    result.push('*');
                } else {
                    result.push('.');
                }
            }
        }
        println!("{}\n{}\n{}\n", s1, s2, result);
    }
}
