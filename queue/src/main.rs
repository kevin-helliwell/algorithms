pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    pub fn length(&self) -> usize {
        self.queue.len()
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    pub fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

fn main() {
    let mut queue: Queue<isize> = Queue::new();
    queue.enqueue(100);
    let item = queue.dequeue();
    assert_eq!(item, 100);
    assert_eq!(queue.is_empty(), true);
}
