fn main() {
    let mut repo = user::Repository::new();
    repo.save(user::User::new(1, "Kim"));
    repo.save(user::User::new(2, "Lee"));

    for user in repo.all() {
        println!("user id={}, name={}", user.id(), user.name());
    }
}

mod user {
    #[derive(Debug)]
    pub struct User {
        id: u64,
        name: String,
    }

    impl User {
        pub fn new(id: u64, name: impl Into<String>) -> Self {
            Self {
                id,
                name: name.into(),
            }
        }

        pub fn id(&self) -> u64 {
            self.id
        }

        pub fn name(&self) -> &str {
            &self.name
        }
    }

    pub struct Repository {
        users: Vec<User>,
    }

    impl Repository {
        pub fn new() -> Self {
            Self { users: Vec::new() }
        }

        pub fn save(&mut self, user: User) {
            self.users.push(user);
        }

        pub fn all(&self) -> &[User] {
            // 내부 Vec를 직접 노출하지 않고 슬라이스로 읽기 전용 접근을 제공합니다.
            &self.users
        }
    }
}
