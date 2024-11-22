use core::future::Future;

use core::{
    pin::Pin,
    task::{Context, Poll},
};

struct Foo {}

impl Future for Foo {
    type Output = u8;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(42)
    }
}

async fn a() {
    println!("hello world");
    Foo {}.await;
    println!("goodbye world");
}

#[cfg_attr(test, test)]
fn main() -> () {
    executor::add_async(async {
        a().await;
    });

    let result = executor::block_on(Foo {});
    assert_eq!(result, 42);

    while !executor::is_done() {
        executor::update();
    }
}
