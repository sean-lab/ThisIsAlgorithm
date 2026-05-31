// 환형 큐 (Circular Queue)
// Clang/03/CircularQueue 포팅. 로직은 ch03::circular_queue 모듈에 있다.
use ch03::circular_queue::CircularQueue;

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
