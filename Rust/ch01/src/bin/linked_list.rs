// 단순 연결 리스트 (Singly Linked List)
// Clang/01/LinkedList 포팅. C의 포인터 기반 구조를 raw pointer로 충실히 옮겼다.
use std::ptr;

type ElementType = i32;

struct Node {
    data: ElementType,
    next: *mut Node,
}

unsafe fn sll_create_node(new_data: ElementType) -> *mut Node {
    Box::into_raw(Box::new(Node {
        data: new_data,
        next: ptr::null_mut(),
    }))
}

unsafe fn sll_destroy_node(node: *mut Node) {
    drop(Box::from_raw(node));
}

unsafe fn sll_append_node(head: &mut *mut Node, new_node: *mut Node) {
    if head.is_null() {
        *head = new_node;
    } else {
        let mut tail = *head;
        while !(*tail).next.is_null() {
            tail = (*tail).next;
        }
        (*tail).next = new_node;
    }
}

unsafe fn sll_insert_after(current: *mut Node, new_node: *mut Node) {
    (*new_node).next = (*current).next;
    (*current).next = new_node;
}

unsafe fn sll_insert_new_head(head: &mut *mut Node, new_head: *mut Node) {
    if head.is_null() {
        *head = new_head;
    } else {
        (*new_head).next = *head;
        *head = new_head;
    }
}

unsafe fn sll_remove_node(head: &mut *mut Node, remove: *mut Node) {
    if *head == remove {
        *head = (*remove).next;
    } else {
        let mut current = *head;
        while !current.is_null() && (*current).next != remove {
            current = (*current).next;
        }
        if !current.is_null() {
            (*current).next = (*remove).next;
        }
    }
}

unsafe fn sll_get_node_at(head: *mut Node, mut location: i32) -> *mut Node {
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

unsafe fn sll_get_node_count(head: *mut Node) -> i32 {
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
            let new_node = sll_create_node(i);
            sll_append_node(&mut list, new_node);
        }

        let new_node = sll_create_node(-1);
        sll_insert_new_head(&mut list, new_node);

        let new_node = sll_create_node(-2);
        sll_insert_new_head(&mut list, new_node);

        // 리스트 출력
        let count = sll_get_node_count(list);
        for i in 0..count {
            let current = sll_get_node_at(list, i);
            println!("List[{}] : {}", i, (*current).data);
        }

        // 리스트의 세번째 노드 뒤에 새 노드 삽입
        println!("\nInserting 3000 After [2]...\n");

        let current = sll_get_node_at(list, 2);
        let new_node = sll_create_node(3000);
        sll_insert_after(current, new_node);

        // 리스트 출력
        let count = sll_get_node_count(list);
        for i in 0..count {
            let current = sll_get_node_at(list, i);
            println!("List[{}] : {}", i, (*current).data);
        }

        // 모든 노드를 메모리에서 제거
        println!("\nDestroying List...");
        let count = sll_get_node_count(list);
        for _ in 0..count {
            let current = sll_get_node_at(list, 0);
            if !current.is_null() {
                sll_remove_node(&mut list, current);
                sll_destroy_node(current);
            }
        }
    }
}
