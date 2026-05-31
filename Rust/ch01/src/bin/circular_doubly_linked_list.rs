// 원형 이중 연결 리스트 (Circular Doubly Linked List)
// Clang/01/CircularDoublyLinkedList 포팅.
use std::ptr;

type ElementType = i32;

struct Node {
    data: ElementType,
    prev: *mut Node,
    next: *mut Node,
}

unsafe fn cdll_create_node(new_data: ElementType) -> *mut Node {
    Box::into_raw(Box::new(Node {
        data: new_data,
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
    }))
}

unsafe fn cdll_destroy_node(node: *mut Node) {
    drop(Box::from_raw(node));
}

unsafe fn cdll_append_node(head: &mut *mut Node, new_node: *mut Node) {
    if head.is_null() {
        *head = new_node;
        (**head).next = *head;
        (**head).prev = *head;
    } else {
        // 테일과 헤드 사이에 NewNode를 삽입한다.
        let tail = (**head).prev;

        (*(*tail).next).prev = new_node;
        (*tail).next = new_node;

        (*new_node).next = *head;
        (*new_node).prev = tail;
    }
}

unsafe fn cdll_insert_after(current: *mut Node, new_node: *mut Node) {
    (*new_node).next = (*current).next;
    (*new_node).prev = current;

    if !(*current).next.is_null() {
        (*(*current).next).prev = new_node;
        (*current).next = new_node;
    }
}

unsafe fn cdll_remove_node(head: &mut *mut Node, remove: *mut Node) {
    if *head == remove {
        (*(**head).prev).next = (*remove).next;
        (*(**head).next).prev = (*remove).prev;

        *head = (*remove).next;

        (*remove).prev = ptr::null_mut();
        (*remove).next = ptr::null_mut();
    } else {
        (*(*remove).prev).next = (*remove).next;
        (*(*remove).next).prev = (*remove).prev;

        (*remove).prev = ptr::null_mut();
        (*remove).next = ptr::null_mut();
    }
}

unsafe fn cdll_get_node_at(head: *mut Node, location: i32) -> *mut Node {
    let mut current = head;
    let mut i = 0;

    if location < 0 || head.is_null() {
        return ptr::null_mut();
    }

    while i < location {
        current = (*current).next;
        i += 1;

        // 원형 리스트에서 한 바퀴 돌았다면 중지
        if current == head && i <= location {
            break;
        }
    }

    current
}

unsafe fn cdll_get_node_count(head: *mut Node) -> i32 {
    let mut count = 0;
    let mut current = head;

    if head.is_null() {
        return 0;
    }

    loop {
        count += 1;
        current = (*current).next;
        if current == head {
            break;
        }
    }

    count
}

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
