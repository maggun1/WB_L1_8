use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use dashmap::DashMap;

fn main() {
    let mutex_map = Arc::new(Mutex::new(HashMap::new()));
    let dash_map = Arc::new(DashMap::new());

    let mut handles = vec![];

    for i in 0..10 {
        let mutex_map = Arc::clone(&mutex_map);
        let dash_map = Arc::clone(&dash_map);

        let handle = thread::spawn(move || {
            mutex_map.lock().unwrap().insert(i, i*i);
            dash_map.insert(i, i*i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


    println!("Mutex + HashMap:");
    for (key, value) in mutex_map.lock().unwrap().iter() {
        println!("{}: {}", key, value);
    }


    println!("\nDashMap:");
    for entry in dash_map.iter() {
        println!("{}: {}", entry.key(), entry.value());
    }
}
