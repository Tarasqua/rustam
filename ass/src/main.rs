use std::{thread, time::Duration};

use ass::{
    chan, multi_sender, multi_sender_pin, race, slow_timedout, spawn_join, timeout, title,
    yielding_control,
};

// INFO: to call a method defined in a Trait, that Trait must be in scope
// so we have to import StreamExt so that we can use .next for the trpl::stream_from_iter
use trpl::StreamExt;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // ERROR: async code needs a runtime; main can initialize a runtime, but it’s not a runtime itself
    // match title(url).await {
    //     Some(title) => println!("Page title is {title}"),
    //     None => eprintln!("{url} had no title"),
    // }

    // trpl::block_on(async {
    //     let url = &args[1]; // WARNING: url inside of the async block (!)
    //     match title(url).await {
    //         Some(title) => println!("Page title is {title}"),
    //         None => eprintln!("{url} had no title"),
    //     }
    // });

    // NOTE: ======================== trpl select ====================================

    // trpl::block_on(async {
    //     race(&args[1], &args[2]).await;
    // });

    // NOTE: ============================ trpl join ================================

    // trpl::block_on(async {
    //     // spawn().await;
    //     spawn_join().await;
    // });

    // NOTE:  ========================== Sending Data Between Tasks ========================

    // trpl::block_on(async {
    //     // chan().await;
    //     // multi_sender().await;
    //     multi_sender_pin().await; // works with pin/unpin
    // });

    // NOTE: ======================= yielding control =========================

    // trpl::block_on(async {
    //     yielding_control().await;
    // });

    // NOTE: ======================= async abstractions =========================

    // trpl::block_on(async {
    //     let slow = async {
    //         _ = trpl::sleep(Duration::from_secs(5)).await;
    //         "Finally finished slow future!"
    //     };

    //     match timeout(slow, Duration::from_secs(2)).await {
    //         Ok(message) => println!("Succeeded with '{message}'"),
    //         Err(duration) => {
    //             println!("Failed after {} seconds", duration.as_secs())
    //         }
    //     };

    //     slow_timedout().await;
    // });

    // NOTE: ======================= stream from iter =========================
    // trpl::block_on(async {
    //     let arr: [i32; 10] = std::array::from_fn(|i| (i + 1) as i32);
    //     let iter = arr.iter().map(|n| n * 2);
    //     let mut stream = trpl::stream_from_iter(iter);

    //     while let Some(value) = stream.next().await {
    //         println!("The value was: {value}");
    //     }
    // })

    // NOTE: ======================= threads alongside with async =========================
    let (tx, mut rx) = trpl::channel::<i32>();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
