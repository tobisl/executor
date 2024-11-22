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

#[executor::main]
async fn async_main() -> std::io::Result<()> {
    let res = Foo {}.await;
    assert_eq!(res, 42);
    Ok(())
}

#[cfg_attr(test, test)]
fn main() -> () {
    async_main().unwrap();
}
