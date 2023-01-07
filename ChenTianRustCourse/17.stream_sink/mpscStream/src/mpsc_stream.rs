
use std::{task::Poll};

use futures::{Stream, Sink, FutureExt, TryFutureExt};
use pin_project::pin_project;
use tokio::{sync::mpsc::{Sender, Receiver, channel}, runtime::Runtime, join};

#[pin_project]
pub struct MyReceiver<T> {
    #[pin]
    inner_receiver: Receiver<T>
}

#[pin_project]
pub struct MySender<T> {
    #[pin]
    inner_sender: Sender<T>,
    buf: Vec<T>,
}
impl<T> MyReceiver<T> {
    fn new(receiver: Receiver<T>) -> Self {
        Self {
            inner_receiver: receiver,
        }
    }
}

impl<T> MySender<T> {
    fn new(sender: Sender<T>) -> Self {
        Self {
            inner_sender: sender,
            buf: vec!(),
        }
    }
}

fn my_channel<T>() -> (MySender<T>, MyReceiver<T>) {
    let mpsc = channel::<T>(1024);
    (
        MySender::new(mpsc.0), MyReceiver::new(mpsc.1)
    )
}

// 为接收者实现 stream
impl<T> Stream for MyReceiver<T> {
    type Item = T;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().inner_receiver.poll_recv(cx)
    }
}

// 为发送者实现 sink
impl<T> Sink<T> for MySender<T> {
    type Error = std::io::Error;

    fn poll_ready(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: std::pin::Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        let this = self.project();
        this.buf.push(item);
        Ok(())
    }

    fn poll_flush(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.as_mut().project();
    
        let mut allf = async {
            let mut fl = vec![];
            while let Some(t) = this.buf.pop() {
                let f = this.inner_sender.send(t);
                fl.push(f);
            }
            for f in fl {
                f.await;
            }
        };
        let pr = Box::pin(allf).poll_unpin(cx);
        if (pr.is_pending()) {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn poll_close(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
        rt.block_on(async {
            this.inner_sender.closed().await;
        });
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod test {
    use std::{thread};

    use futures::{SinkExt, StreamExt, FutureExt};

    use super::my_channel;

    #[test]
    fn test() {
        let (mut send, mut receiver) = my_channel::<u32>();
        // send
        let t1 = thread::spawn(move || {
            println!("begin send.");
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            for i in 0..10 {
                rt.block_on(async {
                    println!("sended: {:?}", i);
                    send.send(i).await.unwrap();
                })
            }
        });

        // receiver
        let t2 = thread::spawn(move || {
            println!("begin receive.");
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on( async {
                while let Some(x) = receiver.next().await {
                    println!("receivered: {:?}", x);
                }
            });
        });
        t1.join();
        t2.join();
    }
}