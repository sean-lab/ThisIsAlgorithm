// 원형 이중 연결 리스트 (Circular Doubly Linked List)
// Clang/01/CircularDoublyLinkedList 포팅. 로직은 ch01::circular_doubly_linked_list 모듈에 있다.
use ch01::circular_doubly_linked_list::*;
use std::ptr;

fn main() {
    unsafe {
        let mut list: *mut Node = ptr::null_mut();

        // 노드 5개 추가
        for i in 0..5 {
            let new_node = cdll_create_node(i);
            cdll_append_node(&mut list, new_node);
        }

        // 리스트 출력
        let count = cdll_get_node_count(list);
        for i in 0..count {
            let current = cdll_get_node_at(list, i);
            println!("List[{}] : {}", i, (*current).data);
        }

        // 리스트의 세번째 칸 뒤에 노드 삽입
        println!("\nInserting 3000 After [2]...\n");

        let current = cdll_get_node_at(list, 2);
        let new_node = cdll_create_node(3000);
        cdll_insert_after(current, new_node);

        println!("\nRemoving Node at 2...");
        let current = cdll_get_node_at(list, 2);
        cdll_remove_node(&mut list, current);
        cdll_destroy_node(current);

        // 리스트 출력 (노드 수의 2배만큼 루프를 돌며 환형임을 확인한다.)
        let count = cdll_get_node_count(list);
        let mut current: *mut Node = ptr::null_mut();
        for i in 0..count * 2 {
            if i == 0 {
                current = list;
            } else {
                current = (*current).next;
            }
            println!("List[{}] : {}", i, (*current).data);
        }

        // 모든 노드를 메모리에서 제거
        println!("\nDestroying List...");
        let count = cdll_get_node_count(list);
        for _ in 0..count {
            let current = cdll_get_node_at(list, 0);
            if !current.is_null() {
                cdll_remove_node(&mut list, current);
                cdll_destroy_node(current);
            }
        }
    }
}
