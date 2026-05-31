// 순차 탐색 + 자기구성 탐색 (Sequential Search / Move-To-Front / Transpose)
// Clang/06/SequentialSearch 포팅. 단순 연결 리스트 기반이므로 raw pointer 사용.
use std::ptr;

struct Node {
    data: i32,
    next: *mut Node,
}

unsafe fn sll_create_node(new_data: i32) -> *mut Node {
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

unsafe fn sll_sequential_search(head: *mut Node, target: i32) -> *mut Node {
    let mut current = head;
    let mut matched: *mut Node = ptr::null_mut();

    while !current.is_null() {
        if (*current).data == target {
            matched = current;
            break;
        } else {
            current = (*current).next;
        }
    }

    matched
}

unsafe fn sll_move_to_front(head: &mut *mut Node, target: i32) -> *mut Node {
    let mut current = *head;
    let mut previous: *mut Node = ptr::null_mut();
    let mut matched: *mut Node = ptr::null_mut();

    while !current.is_null() {
        if (*current).data == target {
            matched = current;
            if !previous.is_null() {
                // 자신의 앞 노드와 다음 노드를 연결
                (*previous).next = (*current).next;
                // 자신을 리스트의 가장 앞으로 옮기기
                (*current).next = *head;
                *head = current;
            }
            break;
        } else {
            previous = current;
            current = (*current).next;
        }
    }
    matched
}

unsafe fn sll_transpose(head: &mut *mut Node, target: i32) -> *mut Node {
    let mut current = *head;
    let mut pprevious: *mut Node = ptr::null_mut();
    let mut previous: *mut Node = ptr::null_mut();
    let mut matched: *mut Node = ptr::null_mut();

    while !current.is_null() {
        if (*current).data == target {
            matched = current;
            if !previous.is_null() {
                if !pprevious.is_null() {
                    (*pprevious).next = current;
                } else {
                    *head = current;
                }

                (*previous).next = (*current).next;
                (*current).next = previous;
            }
            break;
        } else {
            if !previous.is_null() {
                pprevious = previous;
            }
            previous = current;
            current = (*current).next;
        }
    }
    matched
}

fn main() {
    unsafe {
        let count = 10;
        let mut list: *mut Node = ptr::null_mut();
        let init_value = [1, 2, 6, 10, 4, 9, 5, 3, 8, 7];

        for &v in init_value.iter() {
            let new_node = sll_create_node(v);
            sll_append_node(&mut list, new_node);
        }

        // 순차 탐색
        println!("Simple Sequential Search...");
        let matched = sll_sequential_search(list, 9);
        if !matched.is_null() {
            println!("Found : {}", (*matched).data);
        } else {
            println!("Not Found");
        }

        // 전진 이동법
        println!("Move To Front...");
        let matched = sll_move_to_front(&mut list, 4);
        if !matched.is_null() {
            println!("Found : {}", (*matched).data);
        } else {
            println!("Not Found");
        }

        // 전위법
        println!("Transpose...");
        let matched = sll_transpose(&mut list, 7);
        if !matched.is_null() {
            println!("Found : {}", (*matched).data);
        } else {
            println!("Not Found");
        }

        // 모든 노드를 메모리에서 제거
        println!("Destroying List...");
        for _ in 0..count {
            let current = sll_get_node_at(list, 0);
            if !current.is_null() {
                sll_remove_node(&mut list, current);
                sll_destroy_node(current);
            }
        }
    }
}
