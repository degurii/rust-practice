use ch03_basic::read_until_parse;

fn main() {
    let n: i32 = read_until_parse("0 이상의 정수 n 입력:", |s| {
        match s.trim().parse::<i32>() {
            Ok(v) if v >= 0 => Ok(v),
            _ => Err("0 이상의 정수만 입력 가능"),
        }
    });

    println!("{n}번째 피보나치 수: {}", fibo(n))
}

fn fibo(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}
