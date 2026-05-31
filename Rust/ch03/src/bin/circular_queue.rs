// 환형 큐 (Circular Queue)
// Clang/03/CircularQueue 포팅. Capacity+1 개의 슬롯과 Front/Rear 인덱스를 사용.

type ElementType = i32;

struct CircularQueue {
    capacity: i32,
    front: i32,
    rear: i32,
    nodes: Vec<ElementType>,
}

impl CircularQueue {
    fn create(capacity: i32) -> CircularQueue {
        CircularQueue {
            capacity,
            front: 0,
            rear: 0,
            nodes: vec![0; (capacity + 1) as usize],
        }
    }

    fn enqueue(&mut self, data: ElementType) {
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

    fn dequeue(&mut self) -> ElementType {
        let position = self.front;

        if self.front == self.capacity {
            self.front = 0;
        } else {
            self.front += 1;
        }

        self.nodes[position as usize]
    }

    fn get_size(&self) -> i32 {
        if self.front <= self.rear {
            self.rear - self.front
        } else {
            self.rear + (self.capacity - self.front) + 1
        }
    }

    fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    fn is_full(&self) -> bool {
        if self.front < self.rear {
            (self.rear - self.front) == self.capacity
        } else {
            (self.rear + 1) == self.front
        }
    }
}

fn main() {
    let mut queue = CircularQueue::create(10);

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.enqueue(4);

    for _ in 0..3 {
        print!("Dequeue: {}, ", queue.dequeue());
        println!("Front:{}, Rear:{}", queue.front, queue.rear);
    }

    let mut i = 100;
    while !queue.is_full() {
        queue.enqueue(i);
        i += 1;
    }

    println!(
        "Capacity: {}, Size: {}\n",
        queue.capacity,
        queue.get_size()
    );

    while !queue.is_empty() {
        print!("Dequeue: {}, ", queue.dequeue());
        println!("Front:{}, Rear:{}", queue.front, queue.rear);
    }
}
