use std::{task::{Waker, Context, Poll}, pin::Pin, sync::{Mutex, Arc}, time::{Duration, self}, thread};

use futures::{Future, executor::block_on};



pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在 Future 和等待线程间共享状态
struct SharedState {
    /// 睡眠时间是否已结束
    completed: bool,
    /// TimerFuture 所运行于的任务的 Waker
    /// 在设置 completed = true 之后，线程可以使用它来告诉
    /// TimerFuture 的任务可以唤醒，看到 completed = true 并前进
    waker: Option<Waker>,
}


impl Future for TimerFuture {
    type Output = ();

    // 查看 shared state，看下 timer 是否已经结束
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        println!("poll running... now:{:?}", time::SystemTime::now());
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置 waker 以便当 timer 结束时，线程可以唤醒当前任务，保证
            // Future 可以在再次被 poll，并看到 compelted = true
            //
            // 相比每次都克隆 waker，如果只做一次线程更有诱惑力，但是
            // TimerFuture 可在执行之的任务间移动，这会导致过期的 waker
            // 指向错误的任务，从而阻止了 TimerFuture 正确的唤醒
            //
            // 注意：可以使用 Waker::will_wake 函数来检查这一点
            // 但为了简单起见，我们省略了这一点
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration/2);

            println!("task running, now:{:?}", time::SystemTime::now());

            thread::sleep(duration/2);

            let mut shared_state = thread_shared_state.lock().unwrap();
            // 发出信号，计时器已停止并唤醒 Future 被 poll 的最后一个任务
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                // 如果注释掉这一行，线程就不会被唤醒，一直被阻塞，为什么呢？
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

async fn async_main() {
    let duration = Duration::new(10, 0);
    println!("begin");
    println!("{:?}", time::SystemTime::now());
    let t = TimerFuture::new(duration);
    t.await;
    println!("{:?}", time::SystemTime::now());
    println!("end");
}

fn main() {
    block_on(async_main());
}