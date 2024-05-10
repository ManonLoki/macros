use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;

#[tokio::main]
async fn main() {
    let fut = MyFut::new(10);
    let ret = fut.await;
    println!("{:?}", ret);
}

struct MyFut {
    pooled: bool,
    data: usize,
}

impl MyFut {
    fn new(data: usize) -> Self {
        Self {
            pooled: false,
            data,
        }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.pooled {
            Poll::Ready(self.data)
        } else {
            self.pooled = true;
            // 这里需要wakeup
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
