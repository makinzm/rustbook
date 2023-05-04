use std::thread;

fn main() {
    let mut handles = Vec::new();

    for x in 0..20 {
        handles.push(thread::spawn(move || {
            println!("Hello, world!: {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}
