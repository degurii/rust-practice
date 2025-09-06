use ch03_basic::read_until_parse;

struct Converter;

fn main() {
    let unit = read_until_parse("단위를 입력하세요(C/F):", |s| {
        match s.trim().to_ascii_uppercase().as_str() {
            "C" => Ok('C'),
            "F" => Ok('F'),
            _ => Err("비정상 온도 단위"),
        }
    });
    let temp = read_until_parse("온도를 입력하세요:", |s| s.trim().parse::<f64>());

    let converted = if unit == 'C' {
        Converter::c_to_f(temp)
    } else {
        Converter::f_to_c(temp)
    };

    println!("변환 완료: {converted}")
}

impl Converter {
    pub fn c_to_f(temp: f64) -> f64 {
        temp * 9.0 / 5.0 + 32.0
    }
    pub fn f_to_c(temp: f64) -> f64 {
        (temp - 32.0) * 5.0 / 9.0
    }
}
