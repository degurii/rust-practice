use std::io::{self, Write};

pub fn read_until_parse<T, F, E>(prompt: &str, mut parser: F) -> T
where
    F: FnMut(&str) -> Result<T, E>,
{
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("stdout flush 에러");

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            println!("다시 시도하세요.");
            continue;
        }

        match parser(&input) {
            Ok(parsed) => return parsed,
            Err(_) => {
                println!("포맷에 맞게 입력하세요.");
                continue;
            }
        }
    }
}
