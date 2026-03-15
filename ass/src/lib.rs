use std::{process::Output, thread, time::Duration};

use trpl::{Either, Html};

pub async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

// INFO: the difference here is in:
// 1. Lifetime: in the previous function Future holds the url in the whole function, while here it holds in async block only (regardless of the url's lifetime).
// 2. Trait Bounds: here we can specify the exact returning type so that making a function more flexible.
// 3. Laziness: in the previous function code would not compile if we don't use .await, while here we can use non-async code before making the Future.
pub fn title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        // INFO: async move is the expression. the whole block is the expression returned from the function
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

async fn print_upper(s: &str) {
    // INFO: no move is needed here because the async block is borrowing s, not taking ownership of it. The async block will only live as long as the function call, so it can safely borrow s without needing to move it.
    let future = async {
        println!("{}", s.to_uppercase());
    };
    future.await;
}

pub fn race(url1: &str, url2: &str) -> impl Future<Output = ()> {
    async move {
        let title_fut_1 = page_title(&url1);
        let title_fut_2 = page_title(&url2);

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        };
    }
}

pub fn spawn() -> impl Future<Output = ()> {
    async move {
        // INFO: spawn_task is a function that takes an async block and runs it in the background, allowing the main task to continue running concurrently.
        // WARNING: if the main task finishes before the spawned task, the spawned task will be terminated before it can complete its work.
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(100)).await;
        }

        handle.await.unwrap(); // to prevent the main task from finishing before the spawned task
    }
}

// INFO: we can use trpl::join instead of spawn to run multiple tasks concurrently and wait for all of them to complete before proceeding.
pub fn spawn_join() -> impl Future<Output = ()> {
    async move {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    }
}

pub fn chan() -> impl Future<Output = ()> {
    async move {
        let (tx, mut rx) = trpl::channel();

        // INFO: the future to send values through the channel with a delay between sends.
        // WARNING: move is needed here to tx to be dropped at the end of the async block, which will allow the receiver to know when there are no more values to receive and exit the while let loop.
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap(); // INFO: send is a synchronous operation that sends a value through the channel and does not block the sender task.
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // INFO: the future to receive values from the channel and print them as they arrive.
        let rx_fut = async {
            // WARNING: while let loop won’t end until awaiting rx.recv produces None. This will only happen only once the other end of the channel is closed. The channel will close only if we call rx.close or when the sender side, tx, is dropped.
            // INFO: recv waits for a value to be sent through the channel. It does not block the receiver task.
            while let Some(val) = rx.recv().await {
                println!("Received {val}");
            }
        };

        trpl::join(tx_fut, rx_fut).await; // INFO: join is needed to run both the sender and receiver futures concurrently and wait for both of them to complete before proceeding. If we awaited the individual futures in sequence, we would just end up back in a sequential flow — exactly what we’re trying not to do.
    }
}

pub fn multi_sender() -> impl Future<Output = ()> {
    async move {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        // WARNING: we need to use async move as above to ensure that tx1 is dropped at the end of the async block.
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // INFO: there is no matter in which order we run the sender futures, because they will run concurrently and send their values to the receiver as they are ready.
        // WARNING: async move again.
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // INFO: the join! macro awaits an arbitrary number of futures where we know the number of futures at compile time.
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    }
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

// INFO: this function simulates a block of code that takes some time to run and yields control back to the executor at certain points, allowing other tasks to run concurrently.
pub fn yielding_control() -> impl Future<Output = ()> {
    async move {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await; // INFO: making a race
    }
}

pub fn timeout<F: Future>(
    fut: F,
    max_time: Duration,
) -> impl Future<Output = Result<F::Output, Duration>> {
    async move {
        match trpl::select(fut, trpl::sleep(max_time)).await {
            Either::Left(output) => Ok(output),
            Either::Right(_) => Err(max_time),
        }
    }
}

// INFO: self-written timer with trait-based implementation
trait MyTimeoutExt: Future + Sized {
    fn with_timeout(
        self,
        duration: Duration,
    ) -> impl Future<Output = Result<Self::Output, Duration>> {
        async move {
            match trpl::select(self, trpl::sleep(duration)).await {
                trpl::Either::Left(res) => Ok(res),
                trpl::Either::Right(_) => Err(duration),
            }
        }
    }
}

impl<F: Future> MyTimeoutExt for F {}

pub async fn slow_timedout() {
    let slow_task = async {
        _ = trpl::sleep(Duration::from_secs(5)).await;
        "Finally finished slow future!"
    };

    match slow_task.with_timeout(Duration::from_secs(2)).await {
        Ok(message) => println!("{message}"),
        Err(duration) => println!("Failed in {}", duration.as_secs()),
    };
}
