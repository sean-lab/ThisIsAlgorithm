// 연결 리스트 기반 큐 (Linked Queue)
// Clang/03/LinkedQueue 포팅. C는 char* 데이터를 저장하므로 String 사용.
use std::collections::VecDeque;

pub struct LinkedQueue {
    pub items: VecDeque<String>,
    pub count: i32,
}

impl LinkedQueue {
    pub fn create() -> LinkedQueue {
        LinkedQueue {
            items: VecDeque::new(),
            count: 0,
        }
    }

    pub fn enqueue(&mut self, data: String) {
        self.items.push_back(data);
        self.count += 1;
    }

    pub fn dequeue(&mut self) -> Option<String> {
        let front = self.items.pop_front();
        if front.is_some() {
            self.count -= 1;
        }
        front
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for LinkedQueue {
    fn default() -> Self {
        Self::create()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_dequeue_is_fifo() {
        let mut q = LinkedQueue::create();
        q.enqueue("abc".to_string());
        q.enqueue("def".to_string());
        q.enqueue("efg".to_string());
        assert_eq!(q.count, 3);
        assert_eq!(q.dequeue().unwrap(), "abc");
        assert_eq!(q.dequeue().unwrap(), "def");
        assert_eq!(q.dequeue().unwrap(), "efg");
        assert!(q.is_empty());
        assert_eq!(q.count, 0);
    }

    #[test]
    fn dequeue_empty_is_none() {
        let mut q = LinkedQueue::create();
        assert!(q.is_empty());
        assert!(q.dequeue().is_none());
        assert_eq!(q.count, 0);
    }
}
