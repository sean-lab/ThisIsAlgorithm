// 이중 연결 리스트 (Doubly Linked List)
// Clang/01/DoublyLinkedList 포팅. 로직은 ch01::doubly_linked_list 모듈에 있다.
use ch01::doubly_linked_list::*;
use std::ptr;

fn main() {
    unsafe {
        let mut list: *mut Node = ptr::null_mut();

        // 노드 5개 추가
        for i in 0..5 {
            let new_node = dll_create_node(i);
            dll_append_node(&mut list, new_node);
        }

        // 리스트 출력
        let count = dll_get_node_count(list);
        for i in 0..count {
            let current = dll_get_node_at(list, i);
            println!("List[{}] : {}", i, (*current).data);
        }

        // 리스트의 세번째 칸 뒤에 노드 삽입
        println!("\nInserting 3000 After [2]...\n");

        let current = dll_get_node_at(list, 2);
        let new_node = dll_create_node(3000);
        dll_insert_after(current, new_node);

        // 리스트 출력
        let count = dll_get_node_count(list);
        for i in 0..count {
            let current = dll_get_node_at(list, i);
            println!("List[{}] : {}", i, (*current).data);
        }

        // 모든 노드를 메모리에서 제거
        println!("\nDestroying List...");
        let count = dll_get_node_count(list);
        for _ in 0..count {
            let current = dll_get_node_at(list, 0);
            if !current.is_null() {
                dll_remove_node(&mut list, current);
                dll_destroy_node(current);
            }
        }
    }
}
