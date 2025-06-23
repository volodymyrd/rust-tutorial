use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, data: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(data);
        drop(inner);
        self.shared.available.notify_one();
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.shared.inner.lock().unwrap().senders -= 1;
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        //drop(inner);
        Sender {
            shared: self.shared.clone(),
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                None => {
                    if inner.senders == 0 {
                        return None;
                    }
                    inner = self.shared.available.wait(inner).unwrap();
                }
                Some(data) => return Some(data),
            }
        }
    }
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared {
        inner: Mutex::new(Inner {
            queue: VecDeque::default(),
            senders: 1,
        }),
        available: Condvar::new(),
    });
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);

        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn closed() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);

        assert_eq!(rx.recv(), None);
    }
}
