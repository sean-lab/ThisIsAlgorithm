// 환형 큐 (Circular Queue)
// Clang/03/CircularQueue 포팅. Capacity+1 개의 슬롯과 Front/Rear 인덱스를 사용.

pub type ElementType = i32;

pub struct CircularQueue {
    pub capacity: i32,
    pub front: i32,
    pub rear: i32,
    pub nodes: Vec<ElementType>,
}

impl CircularQueue {
    pub fn create(capacity: i32) -> CircularQueue {
        CircularQueue {
            capacity,
            front: 0,
            rear: 0,
            nodes: vec![0; (capacity + 1) as usize],
        }
    }

    pub fn enqueue(&mut self, data: ElementType) {
        let position;
        if self.rear == self.capacity {
            position = self.rear;
            self.rear = 0;
        } else {
            position = self.rear;
            self.rear += 1;
        }
        self.nodes[position as usize] = data;
    }

    pub fn dequeue(&mut self) -> ElementType {
        let position = self.front;

        if self.front == self.capacity {
            self.front = 0;
        } else {
            self.front += 1;
        }

        self.nodes[position as usize]
    }

    pub fn get_size(&self) -> i32 {
        if self.front <= self.rear {
            self.rear - self.front
        } else {
            self.rear + (self.capacity - self.front) + 1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    pub fn is_full(&self) -> bool {
        if self.front < self.rear {
            (self.rear - self.front) == self.capacity
        } else {
            (self.rear + 1) == self.front
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_dequeue_is_fifo() {
        let mut q = CircularQueue::create(10);
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.get_size(), 3);
        assert_eq!(q.dequeue(), 1);
        assert_eq!(q.dequeue(), 2);
        assert_eq!(q.dequeue(), 3);
        assert!(q.is_empty());
    }

    #[test]
    fn fills_to_capacity() {
        let mut q = CircularQueue::create(3);
        assert!(q.is_empty());
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);
        assert!(q.is_full());
        assert_eq!(q.get_size(), 3);
    }

    #[test]
    fn wraps_around() {
        let mut q = CircularQueue::create(3);
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.dequeue(), 1);
        assert_eq!(q.dequeue(), 2);
        // 두 칸 비었으니 다시 채워 넣으며 인덱스가 환형으로 감싼다.
        q.enqueue(4);
        q.enqueue(5);
        assert!(q.is_full());
        assert_eq!(q.dequeue(), 3);
        assert_eq!(q.dequeue(), 4);
        assert_eq!(q.dequeue(), 5);
        assert!(q.is_empty());
    }
}
