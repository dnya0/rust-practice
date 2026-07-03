# Rust 문법 한눈에 보기

개발 경험이 있는 사람이 Rust의 핵심 문법과 사고방식을 빠르게 훑어볼 수 있도록 만든 예제 모음입니다.
각 파일은 독립 실행 가능한 바이너리 예제입니다.

## 프로젝트 소개

이 저장소는 Rust를 처음 접하거나 다른 언어 경험을 바탕으로 Rust 문법을 빠르게 비교하며 익히고 싶은 개발자를 위한 학습용 예제 프로젝트입니다.
문법 설명을 길게 나열하기보다, 실제로 컴파일하고 실행해볼 수 있는 작은 예제들을 통해 Rust의 핵심 개념을 확인할 수 있도록 구성했습니다.

특히 Rust에서 중요한 다음 개념들을 중심으로 다룹니다.

- 소유권, 이동, 빌림, 가변 참조
- 구조체, enum, 패턴 매칭
- trait, 제네릭, lifetime
- `Result`, `Option` 기반 에러 처리
- 컬렉션, iterator, 모듈, 동시성

## 필요 환경

- Rust
- Cargo

macOS에서 Homebrew를 사용한다면 다음 명령으로 설치할 수 있습니다.

```bash
brew install rust
```

## 실행 방법

```bash
cd rust-syntax-examples
cargo run --bin 01_basics
cargo run --bin 02_types
cargo run --bin 03_macros
cargo run --bin 04_ownership_borrowing
cargo run --bin 05_struct_enum_match
cargo run --bin 06_collections_iterators
cargo run --bin 07_error_handling
cargo run --bin 08_traits_generics
cargo run --bin 09_lifetimes
cargo run --bin 10_modules_visibility
cargo run --bin 11_concurrency
```

전체 컴파일 확인:

```bash
cargo check
```

## 구성

- `01_basics.rs`: 변수, 타입, 함수, 조건문, 반복문
- `02_types.rs`: 정수/실수/문자열/배열/튜플/`Option`/`Result` 등 Rust 자료형
- `03_macros.rs`: `println!`, `format!`, `vec!`처럼 `!`가 붙는 매크로 문법
- `04_ownership_borrowing.rs`: 소유권, 이동, 복사, 참조, 가변 참조
- `05_struct_enum_match.rs`: 구조체, enum, `Option`, `match`, `if let`
- `06_collections_iterators.rs`: `Vec`, `HashMap`, iterator 체인
- `07_error_handling.rs`: `Result`, `?`, 커스텀 에러 타입
- `08_traits_generics.rs`: trait, 제네릭, trait bound
- `09_lifetimes.rs`: lifetime 표기와 참조 반환
- `10_modules_visibility.rs`: 모듈, 공개 범위, 연관 함수
- `11_concurrency.rs`: thread, move 클로저, channel, `Arc<Mutex<T>>`
