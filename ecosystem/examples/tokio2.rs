// tokio async task send message to send to the worker thread
use std::thread::{self, sleep};

use anyhow::Result;
use tokio::sync::mpsc;

fn worker(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Some(s) = rx.blocking_recv() {
            sleep(std::time::Duration::from_secs(1));
            let ret = s.to_uppercase();
            println!("ret: {}", ret);
        }
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(32);
    let handle = worker(rx);

    tokio::spawn(async move {
        let mut i = 0;
        loop {
            i += 1;
            tx.send(format!("task {}", i)).await?;
        }

        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    handle.join().unwrap();
    Ok(())
}
