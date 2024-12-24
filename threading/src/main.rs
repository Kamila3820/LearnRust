use std::{borrow::Borrow, sync::{Arc, Mutex}, thread};

fn main() {
    let threaded = thread::spawn(|| { 
        println!("Five is coding");
    });
    threaded.join().unwrap();

    // ex.
    // smart pointer for have many ownerships across the thread!!
    let coin = Arc::new(Mutex::new(100));

    let bot1 = thread::spawn({
        let coin_bot = Arc::clone(&coin);
        move || {
            let mut looting = coin_bot.lock().unwrap();
            *looting -= 10;
        }
    });

    let bot2 = thread::spawn({
        let coin_bot = Arc::clone(&coin);
        move || {
            let mut looting = coin_bot.lock().unwrap();
            *looting -= 10;
        }
    });

    let bot3 = thread::spawn({
        let coin_bot = Arc::clone(&coin);
        move || {
            let mut looting = coin_bot.lock().unwrap();
            *looting -= 30;
        }
    });

    println!("{:?}", coin);

    //
    bot1.join().unwrap();
    bot2.join().unwrap();
    bot3.join().unwrap();

    println!("{:?}", coin.lock().unwrap());
    // move is to make that thread being an owner for those variable cannot use outside the thread to prevent racing condition.
    // Mutex is like Refcell but lock for thread to use across.
}
