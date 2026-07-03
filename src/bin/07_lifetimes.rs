fn main() {
    let left = String::from("short");
    let right = String::from("much longer");

    let result = longest(left.as_str(), right.as_str());
    println!("더 긴 문자열: {result}");

    let config_text = String::from("host=localhost");
    let config = Config {
        raw: config_text.as_str(),
    };
    println!("config raw={}", config.raw);
}

// 반환 참조가 x와 y 중 하나를 가리키므로, 두 입력과 반환값의 lifetime 관계를 명시합니다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

struct Config<'a> {
    // 이 구조체는 문자열 데이터를 소유하지 않고 빌려서 보관합니다.
    // 따라서 Config 인스턴스는 raw가 가리키는 데이터보다 오래 살 수 없습니다.
    raw: &'a str,
}
