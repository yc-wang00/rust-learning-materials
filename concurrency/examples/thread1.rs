use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // create producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);
    
    // create consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }

        println!("Consumer stopped");
        42
    });

    let secret = consumer.join().map_err(|e| anyhow!("Consumer thread panicked: {:?}", e))?;
    println!("Consumer returned: {}", secret);
    Ok(())
}

fn producer(idx:usize, tx: mpsc::Sender<Msg>) -> Result<()>{
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time));

        // randomly stop the producer
        if rand::random::<u8>() % 5 == 0 {
            println!("Producer {} stopped", idx);
            break;
        }
    }

    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}