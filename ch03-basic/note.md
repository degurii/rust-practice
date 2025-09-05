# Ch03: Basic

한줄평: 불변최고 / 전형적인 정적타입 언어 체계에 함수형 피처를 실용적으로 녹였다. 굉장히 맛있음

## 1. 선언

### let
- 기본적으로 불변

- 뮤터블은 mut 키워드 추가

### const
- 타입 필수 명시

- 선언 스코프 제한 X

- 런타임 계산값 할당 불가 -> *런타임 시점의 불안정성이 전파되지 않음이 보장되는 장치인가?? 진짜 "상수"로 쓰는 용도같음*
<img src="image.png" width="600">

### shadowing
- 같은 스코프여도 이미 선언된 변수를 재선언할 수 있음. 타입 변경도 됨;

  -> *남용하면 스파게티 생산기일듯. **안티패턴 사례, 권장 가이드라인 확인 필요***

  -> 작은 스코프 단위에서의 타입 전환이나, 큰 스코프에서 좁은 스코프로 줄여가는 방향에서는 사용에 큰 이슈 없을듯

## 2. 타입

### 스칼라 타입
- 정수, 실수, bool, string 타입들 -> c++이랑 비슷

#### 정수형
- `i8 ... i128` / `u8 ... u128`
- `isize` / `usize`: 아키텍처 bit에 맞춤
- 특정 타입 리터럴: `57u8`처럼 쓸 수 있음

|기수 |	리터럴|
|-|-|
Decimal|	`98_222`
Hex	|`0xff`
Octal	|`0o77`
Binary	|`0b1111_0000`
Byte (u8 only)	|`b'A'`

#### 부동 소수점
- `f32`, `f64`. 기본은 64비트

#### bool
- 1바이트

#### Character
- `char`가 4바이트다 충격. 유니코드 표현이 가능

### Compound type
#### 튜플
```rs
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring
    let (x, y, z) = tup;

    // dot indexing
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

#### 배열
- 러스트는 c랑 다르게 인덱스 넘어서 접근하면 런타임에서 터진다고 한다. 캬 이게 언어지
```rs
fn main() {
    // [type, length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // [init, length]
    let a = [3; 5];
}
```

## 3. 함수
- 선언 위치에 상관 없이 호출 가능
### 구문과 표현식
- statement는 값을 반환하지 않는다
```rs
fn main() {
    let x = (let y = 6); // 불가
}
```

- expression은 세미콜론을 붙이지 않는다;;

  -> 아래 코드에서 `x+1`에 세미콜론을 붙이면 문으로 간주되고, 값을 반환하지 않는다.
```rs
{
    let x = 3;
    x + 1
}
```

### 리턴
- 화살표(`->`)로 반환타입 지정

- `return` 사용 가능하지만, 사용하지 않아도 암묵적으로 마지막 expression을 반환한다.
```rs
fn five() -> i32 {
    5
}
fn main() {
    let x = five();

    println!("The value of x is: {x}"); // "x is: 5"
}
```

## 4. Control Flow
### `if` expression
- 러스트에서 `if`는 표현식이다🚀

- 조건식은 괄호로 감싸지 않는다

- 조건식은 항상 boolean이어야 함

- 제어절이 식이라서 이런 코드가 가능하다. 아주 맘에 든다
```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```
- 단, 변수는 한 타입만 가질 수 있음을 주의해야 함. 이런 케이스에서 `if` / `else if` / `else`는 모두 같은 타입을 반환해야한다
```rs 
fn main() {
    let condition = true;

    // ERROR!!
    let number = if condition { 5 } else { "six" };
}
```

### 반복문
- loop도 값 반환이 된다. break 뒤에 값을 넣자
```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // result == 20
    println!("The result is {result}");
}
```

- 작은 따옴표로 시작하는 `loop label`을 명시할 수 있다. 한정적인 goto 느낌 
```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

- 근본 `while`문도 있다

- 근본 `for`문도 있다. 근데 얘는 컬렉션 순회용이다
```rs
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

- 범위 리터럴이 지원된다. 하프오픈 구간이고, 아래 리터럴 `(1..4)`의 반환 타입은 `std::ops::Range<i32>`
  
  레인지 제네릭 타입은 추론되는거에 따라 알아서 정해진다. 위 케이스에서는 정수가 기본적으로 int로 간주돼서 그렇고, `1u64..4u64` 같은건 `Range<u64>`를 반환
```rs
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

## 책에서 내준 숙제
내일 짜보자..
* 화씨 온도와 섭씨 온도 간 변환하기
* n번째 피보나치 수 생성하기
* 크리스마스 캐롤 ‘The Twelve Days of Christmas’ 노래의 반복성을 활용하여 가사 출력해보기