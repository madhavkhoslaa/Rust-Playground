use std::thread;

pub fn run() {
    thread::spawn(|| {
        for i in 0..30 {
            println!("in thread {:}", i);
        }
    });
    for i in 0..30 {
        println!("outside spawned thread {:}", i);
    }
}
pub fn joinrun() {    
    let join = thread::spawn(|| {
        for i in 0..30 {
            println!("jouined in thread {:}", i);
        }
    });
    for i in 0..30 {
        println!(" joined outside spawned thread {:}", i);
    }
    join.join().unwrap();    
}
