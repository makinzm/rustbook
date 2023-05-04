use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    static N: usize = 10;

    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; N]));

    for x in 0..N {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
