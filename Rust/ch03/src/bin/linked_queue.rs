// 연결 리스트 기반 큐 (Linked Queue)
// Clang/03/LinkedQueue 포팅. 로직은 ch03::linked_queue 모듈에 있다.
use ch03::linked_queue::LinkedQueue;

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
