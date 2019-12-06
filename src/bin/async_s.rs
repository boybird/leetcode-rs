#![feature(async_closure)]

use std::future::Future;
async fn compute() {
    println!("async->");
}

fn compute_01() -> impl Future<Output = Result<(), String>> {
    async move {
        for i in 1..10000 {
            let _ = i * 2;
        }
        println!("=> 01 it is over!");
        Ok(())
    }
}

async fn compute_03(i: i32) -> i32 {
    dbg!(i + 1)
}

fn compute_05(value: i32) -> impl Future<Output = i32> {
    let closure = async move |v: i32| dbg!(v + compute_03(v).await);
    closure(value)
}

fn main() {
    dbg!(async_std::task::block_on(compute_05(1)));
}
