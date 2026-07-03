fn main() {
    scalar_types();
    compound_types();
    string_types();
    explicit_conversion();
    option_and_result_types();
}

fn scalar_types() {
    // 정수 타입은 크기와 부호 여부를 이름에 명시합니다.
    // i: signed integer, u: unsigned integer
    let small_signed: i8 = -128;
    let small_unsigned: u8 = 255;
    let default_integer = 10; // 기본 추론은 보통 i32입니다.
    let pointer_sized: usize = 3; // 인덱스나 길이에 자주 사용합니다.

    println!(
        "integers: {small_signed}, {small_unsigned}, {default_integer}, {pointer_sized}"
    );

    // 부동소수점은 f32, f64가 있습니다. 기본은 f64입니다.
    let ratio: f64 = 0.75;
    let compact_ratio: f32 = 0.75;
    println!("floats: {ratio}, {compact_ratio}");

    let is_enabled: bool = true;

    // char는 4바이트 Unicode scalar value입니다.
    // C/Java의 1바이트 문자와 다르게 한글, 이모지도 하나의 char가 될 수 있습니다.
    let initial: char = '김';
    println!("bool={is_enabled}, char={initial}");
}

fn compound_types() {
    // tuple은 서로 다른 타입을 고정된 순서로 묶습니다.
    let user: (&str, u32, bool) = ("Kim", 29, true);
    let (name, age, active) = user;
    println!("tuple destructuring: {name}, {age}, {active}");
    println!("tuple index access: {}", user.0);

    // array는 고정 길이이며 모든 원소 타입이 같아야 합니다.
    let fixed_scores: [i32; 3] = [90, 80, 70];
    println!("array={fixed_scores:?}");

    // Vec는 길이가 변하는 동적 배열입니다. 다른 언어의 ArrayList/List에 가깝습니다.
    let mut dynamic_scores: Vec<i32> = vec![90, 80, 70];
    dynamic_scores.push(100);
    println!("vec={dynamic_scores:?}");
}

fn string_types() {
    // &str은 문자열 슬라이스입니다.
    // 보통 문자열 리터럴은 프로그램 바이너리 안에 들어 있고, &str로 빌려 봅니다.
    let borrowed_text: &str = "hello";

    // String은 힙에 저장되는, 소유권을 가진 문자열입니다.
    let mut owned_text: String = String::from("hello");
    owned_text.push_str(" rust");

    println!("&str={borrowed_text}, String={owned_text}");

    // 함수 인자로는 &str을 받으면 String과 문자열 리터럴을 모두 받을 수 있어 유연합니다.
    print_text(borrowed_text);
    print_text(&owned_text);
}

fn explicit_conversion() {
    let small: u8 = 10;

    // Rust는 숫자 타입을 자동으로 넓혀 변환하지 않습니다.
    // 의도하지 않은 손실이나 오버플로를 피하기 위해 as 또는 From/TryFrom을 명시합니다.
    let bigger: u32 = small as u32;
    println!("converted={bigger}");

    let parsed: i32 = "42".parse().expect("숫자 문자열이어야 합니다");
    println!("parsed={parsed}");
}

fn option_and_result_types() {
    // null 대신 Option<T>를 사용합니다.
    let maybe_name: Option<&str> = Some("Kim");
    let empty_name: Option<&str> = None;

    println!("maybe_name={maybe_name:?}, empty_name={empty_name:?}");

    // 실패할 수 있는 연산은 Result<T, E>를 자주 사용합니다.
    let ok_number: Result<i32, _> = "10".parse();
    let bad_number: Result<i32, _> = "abc".parse();

    println!("ok_number={ok_number:?}");
    println!("bad_number={bad_number:?}");
}

fn print_text(text: &str) {
    println!("text={text}");
}
