use std::{fs, thread, time::Duration};

use tokio::{runtime::Builder, time::sleep};

fn main() {
    let handle = thread::spawn(|| {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        rt.spawn(async {
            println!("Hello, world!");
            let content = fs::read_to_string("Cargo.toml").unwrap();
            println!("Content: {}", content);
        });

        rt.spawn(async {
            println!("Hello, world 2!");
            let ret = expensive_blocking_operation();
            println!("ret 2: {}", ret);
        });

        rt.block_on(async {
            sleep(Duration::from_secs(5)).await;
            println!("Hello, world 3!");
        });
    });

    handle.join().unwrap();
}

fn expensive_blocking_operation() -> String {
    thread::sleep(std::time::Duration::from_secs(2));
    fs::read_to_string("Cargo.toml").unwrap()
}
