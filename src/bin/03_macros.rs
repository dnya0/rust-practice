fn main() {
    // Rust에서 이름 뒤에 !가 붙으면 "매크로 호출"입니다.
    // 이름 뒤에 !가 없으면 일반 함수 호출입니다.
    normal_function("함수 호출");
    macro_like_examples();

    let name = "Kim";
    let age = 29;

    // println!은 함수가 아니라 매크로입니다.
    // 포맷 문자열과 인자를 컴파일 타임에 분석해서 출력 코드를 만들어냅니다.
    println!("name={name}, age={age}");

    // format!도 매크로입니다. 출력하지 않고 String을 만들어 반환합니다.
    let message = format!("사용자: {name} ({age})");
    println!("{message}");

    // vec!도 매크로입니다. Vec를 간단히 만들 수 있는 문법을 제공합니다.
    let numbers = vec![1, 2, 3];
    println!("numbers={numbers:?}");

    // dbg!는 디버깅용 매크로입니다.
    // 파일명, 라인 번호, 표현식과 값을 stderr에 출력하고 값의 소유권을 반환합니다.
    let doubled = dbg!(age * 2);
    println!("doubled={doubled}");
}

fn normal_function(message: &str) {
    // 함수는 정해진 타입의 인자를 받고, 런타임에 호출됩니다.
    println!("normal_function: {message}");
}

fn macro_like_examples() {
    // 함수는 이런 형태로 호출합니다.
    let sum = add(1, 2);

    // 매크로는 이런 형태로 호출합니다.
    println!("sum={sum}");

    // 중요한 차이:
    // - 함수: 인자 개수와 타입이 함수 시그니처로 고정됩니다.
    // - 매크로: Rust 코드 조각을 입력으로 받아 컴파일 전에 다른 코드로 확장됩니다.
    //
    // 그래서 println!은 아래처럼 인자 개수가 달라도 동작합니다.
    println!("인자 없음");
    println!("인자 하나: {}", 1);
    println!("인자 둘: {}, {}", 1, 2);
}

fn add(left: i32, right: i32) -> i32 {
    left + right
}
