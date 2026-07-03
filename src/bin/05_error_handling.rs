use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ParseUserError {
    input: String,
}

impl fmt::Display for ParseUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "사용자 입력 형식이 올바르지 않습니다: {}", self.input)
    }
}

impl Error for ParseUserError {}

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    for input in ["1:Kim", "bad-input", "2:Lee"] {
        match parse_user(input) {
            Ok(user) => println!("파싱 성공: id={}, name={}", user.id, user.name),
            Err(error) => eprintln!("파싱 실패: {error}"),
        }
    }

    // unwrap은 예제나 테스트에서는 편하지만, 운영 코드에서는 명시적 처리를 선호합니다.
    let parsed = parse_number("42").expect("정상 숫자 문자열이어야 합니다");
    println!("숫자: {parsed}");
}

fn parse_user(input: &str) -> Result<User, Box<dyn Error>> {
    // split_once는 구분자가 있으면 Some((left, right)), 없으면 None입니다.
    let (id_text, name) = input.split_once(':').ok_or_else(|| ParseUserError {
        input: input.to_string(),
    })?;

    // ?는 Err를 만나면 현재 함수에서 즉시 반환합니다.
    // Box<dyn Error>로 반환하므로 ParseIntError도 자동 변환됩니다.
    let id = id_text.parse::<u64>()?;

    Ok(User {
        id,
        name: name.to_string(),
    })
}

fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}
