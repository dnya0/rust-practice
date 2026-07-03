trait Summarize {
    fn summary(&self) -> String;

    // trait은 기본 구현을 가질 수 있습니다.
    fn short_summary(&self) -> String {
        String::from("(요약 없음)")
    }
}

struct Article {
    title: String,
    author: String,
}

struct PullRequest {
    number: u32,
    title: String,
}

impl Summarize for Article {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summarize for PullRequest {
    fn summary(&self) -> String {
        format!("#{} {}", self.number, self.title)
    }

    fn short_summary(&self) -> String {
        format!("PR #{}", self.number)
    }
}

fn main() {
    let article = Article {
        title: String::from("Rust ownership"),
        author: String::from("Kim"),
    };
    let pr = PullRequest {
        number: 15,
        title: String::from("Add parser"),
    };

    notify(&article);
    notify(&pr);

    let numbers = vec![3, 8, 2, 10];
    println!("가장 큰 값: {:?}", largest(&numbers));

    let words = vec!["rust", "go", "typescript"];
    println!("가장 큰 값: {:?}", largest(&words));
}

fn notify(item: &impl Summarize) {
    // impl Trait은 간단한 trait bound 문법입니다.
    println!("summary={}", item.summary());
    println!("short={}", item.short_summary());
}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    // 빈 슬라이스를 고려해 Option으로 반환합니다.
    let mut current = *items.first()?;

    for &item in &items[1..] {
        if item > current {
            current = item;
        }
    }

    Some(current)
}
