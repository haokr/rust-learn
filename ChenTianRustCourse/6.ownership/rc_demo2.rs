use std::sync::Arc;

fn main() {
    let s = "hello world!".to_string();

    let arc = Arc::new(s);

    let arc_copy = arc.clone();
    let t = std::thread::spawn(move || {
        println!("sub thread: {:?}", arc_copy);
    });
    t.join().unwrap();
    println!("main thread: {:?}", arc);
}