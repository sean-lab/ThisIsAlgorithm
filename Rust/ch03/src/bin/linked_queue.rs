// 연결 리스트 기반 큐 (Linked Queue)
// Clang/03/LinkedQueue 포팅. C는 char* 데이터를 저장하므로 String 사용.
use std::collections::VecDeque;

struct LinkedQueue {
    items: VecDeque<String>,
    count: i32,
}

impl LinkedQueue {
    fn create() -> LinkedQueue {
        LinkedQueue {
            items: VecDeque::new(),
            count: 0,
        }
    }

    fn enqueue(&mut self, data: String) {
        self.items.push_back(data);
        self.count += 1;
    }

    fn dequeue(&mut self) -> Option<String> {
        let front = self.items.pop_front();
        if front.is_some() {
            self.count -= 1;
        }
        front
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn main() {
    let mut queue = LinkedQueue::create();

    queue.enqueue("abc".to_string());
    queue.enqueue("def".to_string());
    queue.enqueue("efg".to_string());
    queue.enqueue("hij".to_string());

    println!("Queue Size : {}", queue.count);

    while !queue.is_empty() {
        let popped = queue.dequeue().unwrap();
        println!("Dequeue: {} ", popped);
    }
}
