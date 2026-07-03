#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    email: Option<String>,
}

#[derive(Debug)]
enum LoginEvent {
    Success { user_id: u64 },
    Failure(String),
    Locked,
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("Kim"),
        email: Some(String::from("kim@example.com")),
    };

    // 구조체 업데이트 문법: 일부 필드만 바꾸고 나머지는 기존 값을 재사용합니다.
    let renamed = User {
        name: String::from("Lee"),
        ..user
    };
    println!("{renamed:?}");
    println!("변경된 이름: {}", renamed.name);

    // user.name은 String이라 renamed로 이동되었습니다.
    // user.id처럼 Copy 타입인 필드는 여전히 복사되어 사용할 수 있습니다.
    println!("기존 user id는 Copy라 사용 가능: {}", user.id);

    match &renamed.email {
        Some(email) => println!("이메일 있음: {email}"),
        None => println!("이메일 없음"),
    }

    let events = [
        LoginEvent::Success { user_id: 1 },
        LoginEvent::Failure(String::from("잘못된 비밀번호")),
        LoginEvent::Locked,
    ];

    for event in events {
        handle_event(event);
    }

    let maybe_score = Some(95);
    // if let은 match가 한 패턴만 중요할 때 간결합니다.
    if let Some(score) = maybe_score {
        println!("점수: {score}");
    }
}

fn handle_event(event: LoginEvent) {
    match event {
        LoginEvent::Success { user_id } => println!("로그인 성공: user_id={user_id}"),
        LoginEvent::Failure(reason) => println!("로그인 실패: {reason}"),
        LoginEvent::Locked => println!("계정 잠김"),
    }
}
