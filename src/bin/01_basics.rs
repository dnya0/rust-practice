fn main() {
    // Rust 변수는 기본적으로 불변입니다.
    let immutable_count = 10;
    println!("불변 변수: {immutable_count}");

    // 값을 바꿔야 하면 mut를 명시합니다.
    let mut mutable_count = 10;
    mutable_count += 5;
    println!("가변 변수: {mutable_count}");

    // shadowing: 같은 이름으로 새 변수를 다시 선언할 수 있습니다.
    // mut와 달리 타입 변경도 가능합니다.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("공백 개수: {spaces}");

    // 타입 추론이 기본이지만, 필요한 경우 타입을 명시합니다.
    let user_id: u64 = 42;
    let temperature: f64 = 36.5;
    let is_active: bool = true;
    let grade: char = 'A';
    println!("{user_id}, {temperature}, {is_active}, {grade}");

    // tuple은 서로 다른 타입을 묶을 수 있습니다.
    let user = ("kim", 29, true);
    let (name, age, verified) = user;
    println!("name={name}, age={age}, verified={verified}");

    // array는 고정 길이이며 모든 원소 타입이 같아야 합니다.
    let scores = [90, 80, 70];
    println!("첫 번째 점수: {}", scores[0]);

    println!("add(2, 3) = {}", add(2, 3));
    println!("등급: {}", score_to_grade(87));

    // loop는 break에 값을 실어 표현식처럼 사용할 수 있습니다.
    let mut n = 0;
    let doubled = loop {
        n += 1;
        if n == 5 {
            break n * 2;
        }
    };
    println!("loop 결과: {doubled}");

    // while은 조건 기반 반복에 사용합니다.
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}");
        countdown -= 1;
    }

    // for는 iterator를 순회합니다.
    for score in scores {
        println!("score={score}");
    }
}

fn add(left: i32, right: i32) -> i32 {
    // 세미콜론이 없는 마지막 표현식이 반환값입니다.
    left + right
}

fn score_to_grade(score: u32) -> &'static str {
    // if도 표현식입니다. 각 분기의 반환 타입은 같아야 합니다.
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else {
        "C 이하"
    }
}
