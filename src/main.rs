extern crate mpsc_use;
use mpsc_use::queue::{Deadline, DeadlineQueue};

use fibers::sync::oneshot;
use futures::{Future, Poll};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

struct Command {
    value: u128,
    chan: AsyncReply<u128>,
}

#[derive(Debug)]
pub struct AsyncResult<T>(oneshot::Monitor<T, ()>);
impl<T> AsyncResult<T> {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> (AsyncReply<T>, Self) {
        let (tx, rx) = oneshot::monitor();
        (AsyncReply(tx), AsyncResult(rx))
    }
}
impl<T> Future for AsyncResult<T> {
    type Item = T;
    type Error = String;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll().map_err(|e| format!("{:?}", e))
    }
}

#[derive(Debug)]
struct AsyncReply<T>(oneshot::Monitored<T, ()>);
impl<T> AsyncReply<T> {
    fn send(self, result: Result<T, ()>) {
        self.0.exit(result);
    }
}

fn wait<E, F: Future<Error = E>>(mut f: F) -> Result<(), E> {
    while !(f.poll())?.is_ready() {}
    Ok(())
}

type MySender = Sender<Command>;
type MyReceiver = Receiver<Command>;

fn main() {
    const NUM: u128 = 2587_8531;

    let mut queue = DeadlineQueue::new();
    let (command_tx, command_rx): (MySender, MyReceiver) = std::sync::mpsc::channel();

    thread::spawn(move || loop {
        if let Ok(com) = command_rx.try_recv() {
            queue.push(com, Deadline::Infinity);
        } else if let Some(com) = queue.pop() {
            com.chan.send(Ok(com.value));
        }
    });

    println!("start");

    for i in 0..NUM {
        let (reply, result) = AsyncResult::new();
        command_tx
            .send(Command {
                value: i,
                chan: reply,
            })
            .unwrap();
        wait(result).unwrap();
    }

    println!("end");
}
