fn get_digits(mut i: u32) -> u32 {
    let mut sum = 0;
    while i != 0 {
        sum += i % 10;
        i /= 10;
    }
    sum
}

fn main() {
    loop {
        let mut input = String::new();
        let stdin = std::io::stdin();

        match stdin.read_line(&mut input) {
            Ok(..) => {
                let n: u32 = input.trim().parse().expect("not a number");
                if n == 0 {
                    break;
                }

                let mut min_p = 11;
                while get_digits(n) != get_digits(n * min_p) {
                    min_p += 1;
                }

                println!("{min_p}");
            }
            Err(error) => eprintln!("{}", error),
        }
    }
}
