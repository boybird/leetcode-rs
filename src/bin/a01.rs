use async_std::prelude::*;
use async_std::stream;
use async_std::task::{Context, Poll};
use std::pin::Pin;

struct Counter {
    count: usize,
}
impl Counter {
    pub fn new(n: usize) -> Self {
        Counter { count: n }
    }
}
impl Stream for Counter {
    type Item = usize;
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.count = self.count + 1;
        if self.count < 6 {
            Poll::Ready(Some(self.count))
        } else {
            Poll::Ready(None)
        }
    }
}

fn main() -> std::io::Result<()> {
    async_std::task::block_on(async {
        let mut counter = Counter::new(0);
        while let Some(c) = counter.next().await {
            print!("{} ", c);
        }
        println!();
        let mut repeat = stream::repeat(2).take(5);
        while let Some(c) = repeat.next().await {
            print!("{} ", c);
        }
        println!();
        let mut v = stream::repeat(1u8).take(5);
        while let Some(number) = v.next().await {
            print!("{} ", number);
        }
        println!();
        stream::repeat(3u8).take(5).map(|m| println!("{} ", m));
        println!();
        Ok(())
    })
}
