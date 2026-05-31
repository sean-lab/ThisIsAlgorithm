// 원형 이중 연결 리스트 (Circular Doubly Linked List)
// Clang/01/CircularDoublyLinkedList 포팅.
use std::ptr;

pub type ElementType = i32;

pub struct Node {
    pub data: ElementType,
    pub prev: *mut Node,
    pub next: *mut Node,
}

pub unsafe fn cdll_create_node(new_data: ElementType) -> *mut Node {
    Box::into_raw(Box::new(Node {
        data: new_data,
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
    }))
}

pub unsafe fn cdll_destroy_node(node: *mut Node) {
    drop(Box::from_raw(node));
}

pub unsafe fn cdll_append_node(head: &mut *mut Node, new_node: *mut Node) {
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

pub unsafe fn cdll_insert_after(current: *mut Node, new_node: *mut Node) {
    (*new_node).next = (*current).next;
    (*new_node).prev = current;

    if !(*current).next.is_null() {
        (*(*current).next).prev = new_node;
        (*current).next = new_node;
    }
}

pub unsafe fn cdll_remove_node(head: &mut *mut Node, remove: *mut Node) {
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

pub unsafe fn cdll_get_node_at(head: *mut Node, location: i32) -> *mut Node {
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

pub unsafe fn cdll_get_node_count(head: *mut Node) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    // 헤드에서 count개 노드를 순방향으로 수집한다.
    unsafe fn forward(head: *mut Node, count: i32) -> Vec<ElementType> {
        let mut out = Vec::new();
        let mut current = head;
        for _ in 0..count {
            out.push((*current).data);
            current = (*current).next;
        }
        out
    }

    unsafe fn destroy(head: &mut *mut Node) {
        let count = cdll_get_node_count(*head);
        for _ in 0..count {
            let node = cdll_get_node_at(*head, 0);
            if !node.is_null() {
                cdll_remove_node(head, node);
                cdll_destroy_node(node);
            }
        }
    }

    #[test]
    fn append_is_circular() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..5 {
                cdll_append_node(&mut list, cdll_create_node(i));
            }
            assert_eq!(cdll_get_node_count(list), 5);
            // 한 바퀴 돌면 다시 헤드로 돌아온다.
            assert_eq!((*(*list).prev).data, 4);
            assert_eq!((*(*(*list).prev).next).data, 0);
            // 노드 수의 2배를 순회해도 환형으로 이어진다.
            assert_eq!(forward(list, 7), vec![0, 1, 2, 3, 4, 0, 1]);
            destroy(&mut list);
        }
    }

    #[test]
    fn remove_middle_keeps_circular() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..5 {
                cdll_append_node(&mut list, cdll_create_node(i));
            }
            let node = cdll_get_node_at(list, 2);
            cdll_remove_node(&mut list, node);
            cdll_destroy_node(node);
            assert_eq!(cdll_get_node_count(list), 4);
            assert_eq!(forward(list, 4), vec![0, 1, 3, 4]);
            destroy(&mut list);
        }
    }

    #[test]
    fn remove_head_advances_head() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..3 {
                cdll_append_node(&mut list, cdll_create_node(i));
            }
            let head = cdll_get_node_at(list, 0);
            cdll_remove_node(&mut list, head);
            cdll_destroy_node(head);
            assert_eq!(cdll_get_node_count(list), 2);
            assert_eq!((*list).data, 1);
            assert_eq!(forward(list, 3), vec![1, 2, 1]);
            destroy(&mut list);
        }
    }

    #[test]
    fn get_node_at_wraps_and_bounds() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..3 {
                cdll_append_node(&mut list, cdll_create_node(i));
            }
            assert!(cdll_get_node_at(list, -1).is_null());
            assert_eq!((*cdll_get_node_at(list, 1)).data, 1);
            destroy(&mut list);
        }
    }
}
