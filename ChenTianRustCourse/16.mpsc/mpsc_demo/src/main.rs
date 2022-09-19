use std::{sync::{Mutex, Arc, Condvar, atomic::{AtomicUsize, Ordering}}, collections::VecDeque};
use anyhow::Result;
use anyhow::anyhow;

fn main() {
    println!("Hello, world!");
}

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

pub struct Sender<T> {
    sharded: Arc<Shared<T>>,
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    // 缓存
    cache: VecDeque<T>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<()> {
        // 如果没有消费者，写入报错
        if self.total_receivers() == 0 {
            return Err(anyhow!("no receiver left"));
        }

        // 加锁，访问 vecdeque，压入数据，然后立即释放锁
        let was_empty = {
            let mut inner = self.sharded.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(t);
            empty
        };

        // 通知任意一个被挂起等待的消费者
        if was_empty {
            self.sharded.available.notify_one();
        }

        Ok(())
    }

    pub fn total_receivers(&self) -> usize {
        // 使用 SeqCst 保证所有线程看到同样顺序的对 receiver 的操作。
        // 这个值是最新的值
        self.sharded.receivers.load(Ordering::SeqCst)
    }

    pub fn total_queued_items(&self) -> usize {
        let queue = self.sharded.queue.lock().unwrap();
        queue.len()
    }
} 

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        if let Some(v) = self.cache.pop_front() {
            println!("消费者：读到数据(from cache)");
            return Ok(v);
        }

        // 拿到队列的锁
        let mut inner = self.shared.queue.lock().unwrap();
        loop {
            // 从队列里取出元素
            match inner.pop_front() {
                // 有元素，读到数据，锁被释放
                Some(t) => {
                    // 如果当前队列中还有数据，就把消费者的缓存队列和共享队列交换一下
                    // 这样之后再读取，就直接从缓存中读
                    if !inner.is_empty() {
                        std::mem::swap(&mut self.cache, &mut inner);
                    }
                    // 读到数据直接 return
                    println!("消费者：读到数据(from shared)");
                    return Ok(t);
                }
                // 元素为空，并且生产者数量为 0
                None if self.total_senders() == 0 => return Err(anyhow!("no sender left")),
                // 元素为空，阻塞等待，直到唤醒再次读取
                None => {
                    println!("消费者：未读到数据-开始阻塞");
                    inner = self.shared.available.wait(inner).map_err(|_| anyhow!("lock poisoned"))?;
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        // sender 数量 +1
        self.sharded.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            sharded: Arc::clone(&self.sharded),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // sender 数量 -1
        let old = self.sharded.senders.fetch_sub(1, Ordering::AcqRel);
        // sender 走光了，唤醒 receicer
        if old <= 1 {
            self.sharded.available.notify_all();
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // 消费者离开时，将 receiver 减一
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

const INITIAL_SIZE: usize = 32;
impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITIAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);
    (
        Sender {
            sharded: Clone::clone(&shared),
        },
        Receiver { 
            shared,
            cache: VecDeque::with_capacity(INITIAL_SIZE)
        }
    )
}

#[cfg(test)]
mod test {
    use std::{thread::{self, sleep}, time::Duration};

    use crate::unbounded;

 
    // 需求1: 生产者产生数据，消费者消费数据
    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world!".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello world!");
    }


    // 需求2: 允许多个 sender 往 channel 里发送数据
    #[test]
    fn multiple_senders_should_work() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();
        let t = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t1 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(3).unwrap();
        });

        for handle in [t, t1, t2] {
            handle.join().unwrap();
        }

        // 也许可以使用 iterator 实现
        let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
        result.sort();

        assert_eq!(result, [1,2,3]);
    }

    // 需求3: 当队列空的时候，receiver 所在的线程会被阻塞
    #[test]
    fn receiver_should_be_blocked_when_nothing_to_read() {
        let (mut s, r) = unbounded();
        let s1 = s.clone();
        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                // 如果读到数据，确保它和发送的数据一致
                assert_eq!(idx, i);
            }
            // 读不到应该休眠，正常应该不会执行到这一句，执行到这一句说明逻辑出错
            assert!(false);
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });

        // 1ms 足够让生产者发完 100 个消息，消费者消费完 100 个消息并阻塞
        thread::sleep(Duration::from_millis(1));

        // 如果 receiver 被正常唤醒处理，那么队列里的数据都会被读完
        assert_eq!(s1.total_queued_items(), 0);
    }

    // 需求4: 当所有 sender 都退出作用域时，让调用者知道另一侧已经没有生产者了
    #[test]
    fn last_sender_drop_should_error_when_receive() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();


        // send 即用即抛
        for mut sender in senders {
            thread::spawn(move || {
                sender.send("hello").unwrap();
                // send 在此被丢弃
            })
            .join()
            .unwrap();
        }

        // 虽然没有 sender 了，消费者依然可以接受已经在队列里的数据
        for _ in 0..total {
            r.recv().unwrap();
        }

        // 然而读更多数据时会出错
        assert!(r.recv().is_err());
    }
    

    // 需求5: 没有 receiver 时，sender 发送也应该报错
    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s1 = s.clone();
            let s2 = s.clone();
            (s1, s2)
        };

        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }
    
    #[test]
    fn receiver_shall_be_notified_when_all_senders_exit() {
        let (s, mut r) = unbounded::<usize>();
        let (mut sender, mut receiver) = unbounded::<usize>();

        let t1 = thread::spawn(move || {
            sender.send(0).unwrap();
            assert!(r.recv().is_err());
        });

        thread::spawn(move || {
            receiver.recv().unwrap();
            drop(s);
        });

        t1.join().unwrap();
    }

    #[test]
    fn channel_fast_path_should_work() {
        let (mut s, mut r) = unbounded();
        for i in 0..10usize {
            s.send(i).unwrap();
        }

        assert!(r.cache.is_empty());
        // 读取一个数据，此时应该会导致 swap，cache 中有数据
        assert_eq!(0, r.recv().unwrap());
        // 还有 9 个数据在 cache 中
        assert_eq!(r.cache.len(), 9);
        // 在 queue 里没有数据了
        assert_eq!(s.total_queued_items(), 0);

        // 从 cache 里读取剩下的数据
        for (idx, i) in r.into_iter().take(9).enumerate() {
            assert_eq!(idx + 1, i);
        }
    }

    #[test]
    fn receiver_number() {
        let (mut s, mut r) = unbounded();
        s.send(1).unwrap();

        let t1 = thread::spawn(move || {
            r.recv().unwrap();
            r.recv().unwrap();
        });

        sleep(Duration::new(1, 0));
        s.send(1).unwrap();
        t1.join();

        assert_eq!(s.total_receivers(), 0);
    }
}