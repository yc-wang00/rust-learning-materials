use concurrency::metrics::AMapMetrics as Metrics;
use rand::Rng as _;
use std::thread;

use anyhow::{Ok, Result};

const N: usize = 2;
const M: usize = 4;

fn main() {
    let metrics = Metrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
    ]);

    println!("metrics: {:?}", metrics);

    // START N workers and M request workers
    for i in 0..N {
        worker(i, metrics.clone()); // Arc::clone(&metrics)
    }

    for _ in 0..M {
        request_worker(metrics.clone()).unwrap();
    }

    loop {
        thread::sleep(std::time::Duration::from_secs(1));

        // print metrics
        println!("{}", metrics);
    }
}

fn worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || {
        loop {
            // do some work
            let mut rng = rand::thread_rng();

            thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }

        #[allow(unreachable_code)]
        Ok(())
    });
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do some work
            let mut rng = rand::thread_rng();

            thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..=4);
            metrics.inc(format!("req.page.{}", page))?;
        }

        #[allow(unreachable_code)]
        Ok(())
    });

    Ok(())
}
