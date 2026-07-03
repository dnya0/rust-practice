fn main() {
    // String은 힙 데이터를 소유합니다. 대입하면 소유권이 이동(move)합니다.
    let original = String::from("rust");
    let moved = original;
    // println!("{original}"); // 컴파일 에러: original은 더 이상 값을 소유하지 않습니다.
    println!("moved={moved}");

    // i32 같은 Copy 타입은 대입 후에도 원본을 사용할 수 있습니다.
    let a = 10;
    let b = a;
    println!("a={a}, b={b}");

    let name = String::from("Ferris");
    print_without_taking_ownership(&name);
    println!("함수 호출 후에도 사용 가능: {name}");

    let mut title = String::from("Rust");
    append_suffix(&mut title);
    println!("수정된 문자열: {title}");

    // 같은 스코프에서 불변 참조는 여러 개 가능하지만, 가변 참조와 동시에 존재할 수 없습니다.
    let r1 = &title;
    let r2 = &title;
    println!("불변 참조들: {r1}, {r2}");

    // r1, r2가 더 이상 사용되지 않은 뒤 가변 참조를 만들 수 있습니다.
    let r3 = &mut title;
    r3.push_str(" language");
    println!("가변 참조 결과: {r3}");

    let first = first_word("hello rust world");
    println!("첫 단어: {first}");
}

fn print_without_taking_ownership(value: &String) {
    // &String은 빌린 참조입니다. 이 함수는 값을 소유하지 않습니다.
    println!("borrowed={value}");
}

fn append_suffix(value: &mut String) {
    // 가변 참조를 받으면 원본 데이터를 수정할 수 있습니다.
    value.push_str(" syntax");
}

fn first_word(sentence: &str) -> &str {
    // &str은 문자열 슬라이스입니다. String뿐 아니라 문자열 리터럴에도 잘 맞습니다.
    for (index, byte) in sentence.bytes().enumerate() {
        if byte == b' ' {
            return &sentence[..index];
        }
    }
    sentence
}
