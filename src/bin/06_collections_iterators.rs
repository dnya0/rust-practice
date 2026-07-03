use std::collections::HashMap;

fn main() {
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    println!("Vec: {numbers:?}");

    // get은 인덱스 범위를 벗어나도 panic 대신 Option을 반환합니다.
    match numbers.get(10) {
        Some(value) => println!("값: {value}"),
        None => println!("해당 인덱스 없음"),
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Kim"), 90);
    scores.insert(String::from("Lee"), 85);

    // entry API는 없으면 삽입, 있으면 기존 값을 재사용하는 패턴에 적합합니다.
    scores.entry(String::from("Park")).or_insert(70);
    scores.entry(String::from("Kim")).or_insert(0);
    println!("HashMap: {scores:?}");

    let names = vec!["kim", "lee", "park", "choi"];

    let upper_long_names: Vec<String> = names
        .iter()
        .filter(|name| name.len() >= 4)
        .map(|name| name.to_uppercase())
        .collect();

    println!("iterator 결과: {upper_long_names:?}");

    let total: i32 = numbers.iter().sum();
    println!("합계: {total}");

    // into_iter는 컬렉션의 소유권을 소비합니다.
    let labeled: Vec<String> = numbers
        .into_iter()
        .enumerate()
        .map(|(index, value)| format!("{index}: {value}"))
        .collect();
    println!("라벨링: {labeled:?}");
}
