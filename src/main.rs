use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here is the vector {:?}", v);
    });

    handle.join().unwrap();
}
