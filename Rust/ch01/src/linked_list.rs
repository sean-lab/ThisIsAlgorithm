// 단순 연결 리스트 (Singly Linked List)
// Clang/01/LinkedList 포팅. C의 포인터 기반 구조를 raw pointer로 충실히 옮겼다.
use std::ptr;

pub type ElementType = i32;

pub struct Node {
    pub data: ElementType,
    pub next: *mut Node,
}

pub unsafe fn sll_create_node(new_data: ElementType) -> *mut Node {
    Box::into_raw(Box::new(Node {
        data: new_data,
        next: ptr::null_mut(),
    }))
}

pub unsafe fn sll_destroy_node(node: *mut Node) {
    drop(Box::from_raw(node));
}

pub unsafe fn sll_append_node(head: &mut *mut Node, new_node: *mut Node) {
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

pub unsafe fn sll_insert_after(current: *mut Node, new_node: *mut Node) {
    (*new_node).next = (*current).next;
    (*current).next = new_node;
}

pub unsafe fn sll_insert_new_head(head: &mut *mut Node, new_head: *mut Node) {
    if head.is_null() {
        *head = new_head;
    } else {
        (*new_head).next = *head;
        *head = new_head;
    }
}

pub unsafe fn sll_remove_node(head: &mut *mut Node, remove: *mut Node) {
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

pub unsafe fn sll_get_node_at(head: *mut Node, mut location: i32) -> *mut Node {
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

pub unsafe fn sll_get_node_count(head: *mut Node) -> i32 {
    let mut count = 0;
    let mut current = head;
    while !current.is_null() {
        current = (*current).next;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // 테스트 헬퍼: 리스트를 Vec로 수집한다.
    unsafe fn to_vec(head: *mut Node) -> Vec<ElementType> {
        let mut out = Vec::new();
        let mut current = head;
        while !current.is_null() {
            out.push((*current).data);
            current = (*current).next;
        }
        out
    }

    unsafe fn destroy(head: &mut *mut Node) {
        while !head.is_null() {
            let node = *head;
            *head = (*node).next;
            sll_destroy_node(node);
        }
    }

    #[test]
    fn append_builds_in_order() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..5 {
                sll_append_node(&mut list, sll_create_node(i));
            }
            assert_eq!(sll_get_node_count(list), 5);
            assert_eq!(to_vec(list), vec![0, 1, 2, 3, 4]);
            destroy(&mut list);
        }
    }

    #[test]
    fn insert_new_head_prepends() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..3 {
                sll_append_node(&mut list, sll_create_node(i));
            }
            sll_insert_new_head(&mut list, sll_create_node(-1));
            assert_eq!(to_vec(list), vec![-1, 0, 1, 2]);
            destroy(&mut list);
        }
    }

    #[test]
    fn insert_after_middle() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..3 {
                sll_append_node(&mut list, sll_create_node(i));
            }
            let second = sll_get_node_at(list, 1);
            sll_insert_after(second, sll_create_node(99));
            assert_eq!(to_vec(list), vec![0, 1, 99, 2]);
            destroy(&mut list);
        }
    }

    #[test]
    fn remove_head_and_middle() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..4 {
                sll_append_node(&mut list, sll_create_node(i));
            }
            // 헤드 제거
            let head = sll_get_node_at(list, 0);
            sll_remove_node(&mut list, head);
            sll_destroy_node(head);
            assert_eq!(to_vec(list), vec![1, 2, 3]);
            // 가운데 제거
            let mid = sll_get_node_at(list, 1);
            sll_remove_node(&mut list, mid);
            sll_destroy_node(mid);
            assert_eq!(to_vec(list), vec![1, 3]);
            destroy(&mut list);
        }
    }

    #[test]
    fn get_node_at_out_of_range() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            sll_append_node(&mut list, sll_create_node(7));
            assert!(sll_get_node_at(list, -1).is_null());
            assert!(sll_get_node_at(list, 5).is_null());
            destroy(&mut list);
        }
    }
}
