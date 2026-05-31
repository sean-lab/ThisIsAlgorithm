// 이중 연결 리스트 (Doubly Linked List)
// Clang/01/DoublyLinkedList 포팅.
use std::ptr;

type ElementType = i32;

struct Node {
    data: ElementType,
    prev: *mut Node,
    next: *mut Node,
}

unsafe fn dll_create_node(new_data: ElementType) -> *mut Node {
    Box::into_raw(Box::new(Node {
        data: new_data,
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
    }))
}

unsafe fn dll_destroy_node(node: *mut Node) {
    drop(Box::from_raw(node));
}

unsafe fn dll_append_node(head: &mut *mut Node, new_node: *mut Node) {
    if head.is_null() {
        *head = new_node;
    } else {
        let mut tail = *head;
        while !(*tail).next.is_null() {
            tail = (*tail).next;
        }
        (*tail).next = new_node;
        (*new_node).prev = tail;
    }
}

unsafe fn dll_insert_after(current: *mut Node, new_node: *mut Node) {
    (*new_node).next = (*current).next;
    (*new_node).prev = current;

    if !(*current).next.is_null() {
        (*(*current).next).prev = new_node;
    }
    (*current).next = new_node;
}

unsafe fn dll_remove_node(head: &mut *mut Node, remove: *mut Node) {
    if *head == remove {
        *head = (*remove).next;
        if !head.is_null() {
            (**head).prev = ptr::null_mut();
        }
        (*remove).prev = ptr::null_mut();
        (*remove).next = ptr::null_mut();
    } else {
        if !(*remove).prev.is_null() {
            (*(*remove).prev).next = (*remove).next;
        }
        if !(*remove).next.is_null() {
            (*(*remove).next).prev = (*remove).prev;
        }
        (*remove).prev = ptr::null_mut();
        (*remove).next = ptr::null_mut();
    }
}

unsafe fn dll_get_node_at(head: *mut Node, mut location: i32) -> *mut Node {
    let mut current = head;
    if location < 0 {
        return ptr::null_mut();
    }
    while !current.is_null() && location > 0 {
        current = (*current).next;
        location -= 1;
    }
    current
}

unsafe fn dll_get_node_count(head: *mut Node) -> i32 {
    let mut count = 0;
    let mut current = head;
    while !current.is_null() {
        current = (*current).next;
        count += 1;
    }
    count
}

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
