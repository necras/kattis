use std::io::{self, BufRead};

fn read_input() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let _n: i8 = lines.next().unwrap().parse().unwrap();
    lines.collect()
}

fn order(names: Vec<String>) -> String {
    let mut inc = true;
    let mut dec = true;

    for (a, b) in names.iter().zip(names.iter().skip(1)) {
        match a.cmp(b) {
            std::cmp::Ordering::Greater => inc = false,
            std::cmp::Ordering::Less => dec = false,
            std::cmp::Ordering::Equal => (),
        }
    }

    if inc {
        "INCREASING".to_string()
    } else if dec {
        "DECREASING".to_string()
    } else {
        "NEITHER".to_string()
    }
}

fn main() {
    let input = read_input();
    println!("{}", order(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn increasing() {
        let names = vec_of_strings!["A", "B", "C", "D"];
        assert_eq!(String::from("INCREASING"), order(names));
    }
    #[test]
    fn decreasing() {
        let names = vec_of_strings!["D", "C", "B", "A"];
        assert_eq!(String::from("DECREASING"), order(names));
    }
    #[test]
    fn neither() {
        let names = vec_of_strings!["A", "B", "C", "B", "A"];
        assert_eq!(String::from("NEITHER"), order(names));
    }
}
