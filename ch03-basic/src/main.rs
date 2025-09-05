fn main() {
    let condition = false;

    let str = if condition {
        "if"
    } else if true {
        "else if"
    } else {
        "else"
    };

    println!("{}", str)
}
