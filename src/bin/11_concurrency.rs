use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    thread_example();
    channel_example();
    shared_state_example();
}

fn thread_example() {
    let values = vec![1, 2, 3];

    // move는 클로저가 values의 소유권을 가져가게 합니다.
    let handle = thread::spawn(move || {
        let sum: i32 = values.iter().sum();
        println!("스레드 내부 합계: {sum}");
        sum
    });

    let result = handle.join().expect("스레드가 panic 없이 종료되어야 합니다");
    println!("join 결과: {result}");
}

fn channel_example() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        for message in ["compile", "test", "deploy"] {
            sender.send(message.to_string()).expect("수신자가 살아 있어야 합니다");
        }
    });

    // sender가 drop되면 receiver 반복이 종료됩니다.
    for message in receiver {
        println!("channel 메시지: {message}");
    }
}

fn shared_state_example() {
    // Arc는 여러 스레드에서 소유권을 공유하는 참조 카운트 포인터입니다.
    // Mutex는 한 번에 하나의 스레드만 내부 값을 수정하도록 보호합니다.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut value = counter.lock().expect("mutex lock 획득");
            *value += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("스레드 종료");
    }

    println!("공유 카운터: {}", *counter.lock().expect("mutex lock 획득"));
}
