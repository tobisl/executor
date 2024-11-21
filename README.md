# executor

<a href="https://docs.rs/executor"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
[![Tests](https://github.com/tobisl/executor/actions/workflows/test.yml/badge.svg)](https://github.com/tobisl/executor/actions/workflows/test.yml)

```toml
[dependencies]
executor = "0.9.0"
```
## Features
- [x] `#![no_std]` + `alloc`
- [x] simple enough to learn from! (~ 200 lines)
- [x] works with WebAssembly

## WebAssembly

```rust
use web::*;
use executor;

#[no_mangle]
fn main() {
    executor::add_async(async {
        loop {
            set_inner_html(DOM_BODY, "⏰ tic");
            sleep(1000).await;
            set_inner_html(DOM_BODY, "⏰ tock");
            sleep(1000).await;
        }
    });
    while !executor::is_done() {
        executor::update();
    }
}
```

See this working [here](https://richardanaya.github.io/executor/examples/timer/).

## CLI

Even `async-std` can be used if you add something to stop it from exiting too early.

```rust
use async_std::task::sleep;
use std::time::Duration;

fn main() {
    let complete = std::sync::Arc::new(core::sync::atomic::AtomicBool::new(false));
    let ender = complete.clone();
    executor::add_async(async move {
        println!("hello");
        sleep(Duration::from_secs(1)).await;
        println!("world!");
        ender.store(true, core::sync::atomic::Ordering::Release);
    });
    while !complete.load(core::sync::atomic::Ordering::Acquire) {executor::update();}
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `executor` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
