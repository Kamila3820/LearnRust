use std::{sync::{mpsc, Arc}, thread};

fn main() {
    let loot = vec![10, 20, 30];
    let mut gold_coin = 100; 

    let (sender, receiver) = mpsc::sync_channel(3);
    let bot_sender = Arc::new(sender);

    for value in loot.clone().into_iter() {
        thread::spawn({
            let bot_sender_clone = Arc::clone(&bot_sender);
            move || {
                bot_sender_clone.send(value).unwrap();
            }
        });
    }

    for _ in 0..loot.len()-1 {
        let bot_receiver = receiver.recv().unwrap();
        gold_coin += bot_receiver;
    }

    println!("Total gold coin: {}", gold_coin);

}
