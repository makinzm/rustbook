use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
struct CountDown(u32,u32);

impl Future for CountDown {
    type Output = String;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        if self.0 == 0 {
            Poll::Ready(format!("{}:Zero!!!",self.1))
        } else {
            println!("{}::{}", self.1, self.0);
            self.0 -= 1;
            cx.waker().clone().wake();
            // 現在のtaskのcontext
            // https://doc.rust-lang.org/std/task/struct.Waker.html
            Poll::Pending
        }
    }
}

fn main() {
    let countdown_future1 = CountDown(10,0);
    let countdown_future2 = CountDown(20,1);
    // https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/future/fn.join_all.html
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    // https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/executor/fn.block_on.html
    println!("before executor");
    let res = executor::block_on(cd_set);
    println!("after executor");
    println!("res: {:?}", res);
    // `Vec<String>` is not an iterator; try calling `.into_iter()` or `.iter()` -> we need iter()
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s);
    }
}
