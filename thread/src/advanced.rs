use std::sync::{Arc, RwLock};
// use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn advanced_threading() {
    // Mutex for lock the resource for others while used by one thread
    // let data = Arc::new(Mutex::new(vec![1, 2, 3, 4]));

    // RwLock for lock the resourse while it is being write by any thread otherwise it can be access by any thread concurently
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4]));

    let mut handles = vec![];

    for i in 0..=3 {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            if i == 3 {
                let mut data = data_clone.write().unwrap();
                println!("Writing to data {:?} in thread {}", data, i);
                thread::sleep(Duration::from_secs(1));
                data.push(i);
                println!("done");
            } else {
                let data = data_clone.read().unwrap();
                println!("Print data in thread {}: {:?}", i, data);
                thread::sleep(Duration::from_secs(1));
                println!("done");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Print data in main: {:?}", data.read().unwrap());
}
