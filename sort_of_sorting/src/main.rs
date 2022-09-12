fn main() {
    loop {
        let mut input = String::new();
        let stdin = std::io::stdin();

        match stdin.read_line(&mut input) {
            Ok(..) => {
                let str = input.trim();
                if str == "0" {
                    std::process::exit(0);
                }

                match str.parse::<usize>() {
                    Ok(names) => {
                        let mut vec = Vec::<(String, String)>::new();
                        for _ in 0..names {
                            let mut buffer = String::new();
                            stdin.read_line(&mut buffer).unwrap();
                            buffer = buffer.trim().parse().unwrap();
                            let (l, _r) = buffer.split_at(2);
                            vec.push((l.to_string(), buffer));
                        }
                        // sort by first two letters
                        vec.sort_by(|(a, _), (b, _)| a.cmp(b));

                        for v in vec {
                            println!("{}", v.1);
                        }
                        println!();
                    }
                    Err(error) => eprintln!("error: {}", error),
                }
            }
            Err(error) => eprintln!("error: {}", error),
        }
    }
}
