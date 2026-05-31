// 힙 (Heap) - 최소 힙
// Clang/07/Heap 포팅. 로직은 ch07::heap 모듈에 있다.
use ch07::heap::{Heap, HeapNode};

fn main() {
    let mut h = Heap::create(3);
    let mut min_node = HeapNode::default();

    h.insert(12);
    h.insert(87);
    h.insert(111);
    h.insert(34);
    h.insert(16);
    h.insert(75);
    h.print_nodes();

    for _ in 0..6 {
        h.delete_min(&mut min_node);
        h.print_nodes();
    }
}
