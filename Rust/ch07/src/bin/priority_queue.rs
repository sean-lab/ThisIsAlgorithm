// 우선순위 큐 (Priority Queue)
// Clang/07/PriorityQueue 포팅. 로직은 ch07::priority_queue 모듈에 있다.
use ch07::priority_queue::{PQNode, PriorityQueue};

fn print_node(node: &PQNode) {
    println!("작업명 : {} (우선순위:{})", node.data, node.priority);
}

fn main() {
    let mut pq = PriorityQueue::create(3);
    let mut popped = PQNode::default();

    let nodes = [
        PQNode { priority: 34, data: "코딩".to_string() },
        PQNode { priority: 12, data: "고객미팅".to_string() },
        PQNode { priority: 87, data: "커피타기".to_string() },
        PQNode { priority: 45, data: "문서작성".to_string() },
        PQNode { priority: 35, data: "디버깅".to_string() },
        PQNode { priority: 66, data: "이닦기".to_string() },
    ];

    for n in nodes.iter() {
        pq.enqueue(n.clone());
    }

    println!("큐에 남아 있는 작업의 수 : {}", pq.used_size);

    while !pq.is_empty() {
        pq.dequeue(&mut popped);
        print_node(&popped);
    }
}
