use std::time::Duration;
use tokio::{sync::mpsc, time::sleep};
use tokio_util::sync::CancellationToken;

// use futures::{future::FutureExt, join, select};

async fn wait_for(duration: Duration) {
    sleep(duration).await;
}

struct Song {
    name: String,
    duration: Duration,
}

async fn learn_song() -> Song {
    wait_for(Duration::from_millis(30)).await;
    Song {
        name: "Yellow Submarine".into(),
        duration: Duration::from_millis(50),
    }
}

async fn sing(song: Song) {
    println!("I'm singing the song \"{}\"!", song.name);
    wait_for(song.duration).await;
    println!("I finished singing!");
}

async fn dance() {
    println!("I'm dancing!");
    wait_for(std::time::Duration::from_millis(30)).await;
    println!("I finished dancing!");
}

async fn fut1() -> String {
    "Hello from fut1".to_string()
}

async fn fut2() -> String {
    "Hello from fut2".to_string()
}

fn foo() -> impl Future<Output = String> {
    async { "".to_string() }
}

// async fn worker(token: CancellationToken) {
//     loop {
//         tokio::select! {
//             // This is the "Stop Event" check
//             _ = token.cancelled() => {
//                 println!("Worker shutting down gracefully...");
//                 break;
//             }
//             // This waits forever (zero CPU waste) until a message arrives
//             msg = queue.recv() => {
//                 process(msg).await;
//             }
//         }
//     }
// }

#[tokio::main]
async fn main() {
    // let (x, y) = (fut1(), fut2());
    // let (x, y) = join!(x, y); // INFO: both futures will be finished
    // println!("x: {}", x);
    // println!("y: {}", y);

    // // 1. Create the futures and fuse them
    // let f1 = fut1().fuse();
    // let f2 = fut2().fuse();

    // // 2. Pin them to the stack so they can't move
    // tokio::pin!(f1);
    // tokio::pin!(f2);

    // // 3. Now select! can safely poll them
    // select! { // INFO: only one of them will be polled - the one that finished first
    //     res = f1 => println!("f1: {}", res),
    //     res = f2 => println!("f2: {}", res),
    // }

    let instant = std::time::Instant::now();
    let song = learn_song().await;
    tokio::join! { sing(song), dance() };
    println!("Program took: {}ms", instant.elapsed().as_millis());

    // NOTE: select! can be used to wait for multiple futures to complete: so here we wait for either the stop_signal (signals the program to stop) or a message from queue
    // tokio::select! {
    //     _ = stop_signal.changed() => println!("Stopping!"),
    //     msg = queue.recv() => println!("Got message: {:?}", msg),
    // }

    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..10 {
            if tx.send(i).await.is_err() {
                println!("receiver dropped");
                return;
            }
        }
    });

    while let Some(i) = rx.recv().await {
        println!("got = {}", i);
    }
}
