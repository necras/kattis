use std::io::BufRead;

fn read_input() -> (usize, Vec<String>) {
    let stdin = std::io::stdin();
    let mut input = stdin
        .lock()
        .lines()
        .map(|l| l.expect("could not map input"));
    let n: usize = input
        .next()
        .unwrap()
        .parse()
        .expect("could not parse first line");
    (n, input.collect())
}

fn main() {
    let (n, lines) = read_input();

    for (i, _) in lines.iter().enumerate().take(n) {
        let nums: Vec<i64> = lines[i]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if result_reached_by_addition(nums[0], nums[1], nums[2]) {
            println!("Possible");
            continue;
        } else if result_reached_by_subtraction(nums[0], nums[1], nums[2]) {
            println!("Possible");
            continue;
        } else if nums[0] * nums[1] == nums[2] {
            println!("Possible");
            continue;
        } else if result_reached_by_division(nums[0], nums[1], nums[2]) {
            println!("Possible");
            continue;
        } else {
            println!("Impossible");
        }
    }
}

fn result_reached_by_division(int1: i64, int2: i64, res: i64) -> bool {
    let div = int1 / int2;
    if int1 % int2 == 0 && div == res {
        return true;
    }

    let div = int2 / int1;
    if int2 % int1 == 0 && div == res {
        return true;
    }

    false
}

fn result_reached_by_addition(int1: i64, int2: i64, res: i64) -> bool {
    let div = int1 + int2;
    if div == res {
        return true;
    }

    let div = int2 + int1;
    if div == res {
        return true;
    }

    false
}

fn result_reached_by_subtraction(int1: i64, int2: i64, res: i64) -> bool {
    let div = int1 - int2;
    if div == res {
        return true;
    }

    let div = int2 - int1;
    if div == res {
        return true;
    }

    false
}
