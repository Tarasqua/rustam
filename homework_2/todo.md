Here's a focused reading list, grouped by concept. Each item maps directly to something in the codebase.

---

**1. Trait objects and dynamic dispatch**
The foundation of the whole design. `Box<dyn SmartDevice>` is a fat pointer — a data pointer + a vtable pointer. Understanding this explains why you can't just call `turn_on()` on a trait object.
- The Rust Book, ch. 17.2 — "Using Trait Objects That Allow for Values of Different Types"
- Rustonomicon — "Trait Objects" (for the vtable layout internals)

**2. The `Any` downcast pattern (`as_any`)**
`std::any::Any` is a special trait that stores a runtime type ID alongside the value. `downcast_ref::<T>()` compares that ID against `TypeId::of::<T>()`. The reason we need `as_any(&self) -> &dyn Any` as a helper is that you can't directly cast `&dyn SmartDevice` to `&dyn Any` — Rust doesn't support multi-trait upcasting (yet, it's stabilizing in 1.76+).
- `std::any` module docs: https://doc.rust-lang.org/std/any/
- Blog post: "Rust Any and Downcasting" by Amos Wenger (fasterthanlime)

**3. `async`/`await` fundamentals**
Every `async fn` returns an opaque `Future`. Nothing runs until you `.await` it. The executor (Tokio) polls futures and parks them when they're waiting on I/O.
- The Rust Book, ch. 17.1 (new async chapter in edition 2024)
- "Async Rust" book: https://rust-lang.github.io/async-book/

**4. Tokio runtime and task model**
`tokio::spawn` creates a green thread (task). Tasks are cheap — you can have thousands. `#[tokio::main]` wraps your `main` in a multi-threaded executor. Key concepts: the scheduler, work-stealing, and why `Send` bounds matter for spawned tasks.
- Tokio tutorial: https://tokio.rs/tokio/tutorial
- Specifically the "Spawning" and "Shared state" chapters

**5. `tokio::select!`**
Used in `SmartThermometer`'s background loop to race two futures: a UDP receive vs. a cancellation signal. The first one to complete wins; the other is dropped.
- Tokio docs: https://docs.rs/tokio/latest/tokio/macro.select.html
- Tokio tutorial chapter: "Select"

**6. `Arc<Mutex<T>>` — shared state across async tasks**
`Arc` gives shared ownership across threads. `Mutex` serializes access. The pattern `Arc<Mutex<Option<f32>>>` in `SmartThermometer` is the standard way to share mutable state between a spawned task and the owner. Important gotcha: never hold a `MutexGuard` across an `.await` point — it won't compile with `tokio::sync::Mutex` and will deadlock with `std::sync::Mutex`.
- Tokio tutorial: "Shared State" chapter
- The Rust Book, ch. 16.3

**7. `CancellationToken` (tokio-util)**
A cooperative cancellation primitive. The background task checks `cancel.cancelled()` in `select!`. When the thermometer is dropped, `Drop` calls `cancel.cancel()`, which resolves that future and lets the loop exit cleanly. Alternative: `tokio::sync::watch` channel, or `JoinHandle::abort()` (forceful).
- tokio-util docs: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html

**8. `async-trait` crate**
`async fn` in traits isn't natively object-safe because each impl returns a different opaque `Future` type. `async-trait` rewrites them to return `Pin<Box<dyn Future>>`, which is a concrete type that fits in a vtable. This is the cost: a heap allocation per call. The native RPITIT (`-> impl Future`) is landing in stable Rust but isn't yet object-safe.
- crate docs: https://docs.rs/async-trait
- RFC 3185 (native async fn in traits) for context on where this is heading

**9. Tokio networking — `TcpListener`, `TcpStream`, `UdpSocket`**
The async equivalents of `std::net`. Key pattern in `socket_simulator`: `loop { listener.accept().await → tokio::spawn(handle_client) }`. Each client gets its own task. `into_split()` gives you independent read/write halves so you can pass them to different parts of the code.
- Tokio docs: https://docs.rs/tokio/latest/tokio/net/index.html
- Tokio tutorial: "I/O" chapter

**10. `BufReader` + `read_line` for framing**
Raw TCP is a byte stream with no message boundaries. `BufReader::read_line` reads until `\n`, which is the framing strategy used in the TCP protocol here. Alternative framing strategies: length-prefix, `\0` delimiter, or a proper codec via `tokio-util::codec`.
- `tokio::io::AsyncBufReadExt` docs

**11. `thiserror` and the error design pattern**
`#[derive(Error)]` generates `Display` and `From` impls. The `#[from]` attribute on `Io(#[from] std::io::Error)` means `?` on any `io::Error` automatically converts it to `SmartHomeError::Io`. This is the idiomatic alternative to `anyhow` for library code.
- thiserror docs: https://docs.rs/thiserror
- "Error Handling in Rust" — Nick Cameron's blog post

**12. `clap` derive API**
`#[derive(Parser)]` + `#[arg(...)]` generates a full CLI parser from struct fields. The `default_value` attribute sets the fallback when the flag isn't provided.
- clap docs: https://docs.rs/clap — "Derive" section

**13. `serde` + `toml` for config deserialization**
`#[derive(Deserialize)]` generates a parser for any format. `toml::from_str` drives it with TOML input. The struct fields map directly to TOML keys.
- serde docs: https://serde.rs
- toml crate: https://docs.rs/toml

**14. Property-based testing with `proptest`**
Instead of writing specific inputs, you define a *strategy* (a generator) and a *property* (an assertion that must hold for all generated inputs). `proptest!` macro runs 100+ random cases and shrinks failures to the minimal counterexample.
- proptest book: https://proptest-rs.github.io/proptest/intro.html

---

The single most impactful thing to read first is the **Tokio tutorial** (tokio.rs/tokio/tutorial) — it covers tasks, shared state, channels, and I/O in one place and directly maps to ~70% of this codebase.
