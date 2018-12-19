use std::cmp;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Deadline {
    Immediate,
    Within(Duration),
    Infinity,
}
impl Default for Deadline {
    fn default() -> Self {
        Deadline::Infinity
    }
}

#[derive(Debug)]
pub struct DeadlineQueue<T> {
    seqno: u64,
    heap: BinaryHeap<Item<T>>,
}
impl<T> DeadlineQueue<T> {
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// 新しい`DeadlineQueue`インスタンスを生成する.
    pub fn new() -> Self {
        DeadlineQueue {
            seqno: 0,
            heap: BinaryHeap::new(),
        }
    }

    /// 新しいコマンドをキューに追加する.
    pub fn push(&mut self, v: T, deadline: Deadline) {
        let deadline = AbsoluteDeadline::new(deadline);
        let item = Item {
            seqno: self.seqno,
            item: v,
            deadline,
        };
        self.heap.push(item);
        self.seqno += 1;
    }

    /// 次に処理するコマンドを取り出す.
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|t| t.item)
    }

    /// キューに格納されている要素数を返す.
    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

/// ヒープに格納する要素.
#[derive(Debug)]
struct Item<T> {
    seqno: u64, // デッドラインが同じ要素をFIFO順で扱うためのシーケンス番号
    item: T,
    deadline: AbsoluteDeadline,
}
impl<T> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.seqno == other.seqno
    }
}
impl<T> Eq for Item<T> {}
impl<T> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other
            .deadline
            .cmp(&self.deadline)
            .then_with(|| other.seqno.cmp(&self.seqno))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum AbsoluteDeadline {
    Immediate,
    Until(Instant),
    Infinity,
}
impl AbsoluteDeadline {
    fn new(relative: Deadline) -> Self {
        match relative {
            Deadline::Immediate => AbsoluteDeadline::Immediate,
            Deadline::Within(d) => AbsoluteDeadline::Until(Instant::now() + d),
            Deadline::Infinity => AbsoluteDeadline::Infinity,
        }
    }
}
