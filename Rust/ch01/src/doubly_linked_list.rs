// 이중 연결 리스트 (Doubly Linked List)
// Clang/01/DoublyLinkedList 포팅.
use std::ptr;

pub type ElementType = i32;

pub struct Node {
    pub data: ElementType,
    pub prev: *mut Node,
    pub next: *mut Node,
}

pub unsafe fn dll_create_node(new_data: ElementType) -> *mut Node {
    Box::into_raw(Box::new(Node {
        data: new_data,
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
    }))
}

pub unsafe fn dll_destroy_node(node: *mut Node) {
    drop(Box::from_raw(node));
}

pub unsafe fn dll_append_node(head: &mut *mut Node, new_node: *mut Node) {
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

pub unsafe fn dll_insert_after(current: *mut Node, new_node: *mut Node) {
    (*new_node).next = (*current).next;
    (*new_node).prev = current;

    if !(*current).next.is_null() {
        (*(*current).next).prev = new_node;
    }
    (*current).next = new_node;
}

pub unsafe fn dll_remove_node(head: &mut *mut Node, remove: *mut Node) {
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

pub unsafe fn dll_get_node_at(head: *mut Node, mut location: i32) -> *mut Node {
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

pub unsafe fn dll_get_node_count(head: *mut Node) -> i32 {
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

    unsafe fn to_vec(head: *mut Node) -> Vec<ElementType> {
        let mut out = Vec::new();
        let mut current = head;
        while !current.is_null() {
            out.push((*current).data);
            current = (*current).next;
        }
        out
    }

    // prev 링크가 올바르게 이어졌는지(뒤에서 앞으로 순회) 검증한다.
    unsafe fn to_vec_reverse(head: *mut Node) -> Vec<ElementType> {
        if head.is_null() {
            return Vec::new();
        }
        let mut tail = head;
        while !(*tail).next.is_null() {
            tail = (*tail).next;
        }
        let mut out = Vec::new();
        let mut current = tail;
        while !current.is_null() {
            out.push((*current).data);
            current = (*current).prev;
        }
        out
    }

    unsafe fn destroy(head: &mut *mut Node) {
        while !head.is_null() {
            let node = *head;
            *head = (*node).next;
            dll_destroy_node(node);
        }
    }

    #[test]
    fn append_links_both_directions() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..5 {
                dll_append_node(&mut list, dll_create_node(i));
            }
            assert_eq!(to_vec(list), vec![0, 1, 2, 3, 4]);
            assert_eq!(to_vec_reverse(list), vec![4, 3, 2, 1, 0]);
            destroy(&mut list);
        }
    }

    #[test]
    fn insert_after_fixes_prev() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..3 {
                dll_append_node(&mut list, dll_create_node(i));
            }
            let second = dll_get_node_at(list, 1);
            dll_insert_after(second, dll_create_node(99));
            assert_eq!(to_vec(list), vec![0, 1, 99, 2]);
            assert_eq!(to_vec_reverse(list), vec![2, 99, 1, 0]);
            destroy(&mut list);
        }
    }

    #[test]
    fn remove_head_keeps_links() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..4 {
                dll_append_node(&mut list, dll_create_node(i));
            }
            let head = dll_get_node_at(list, 0);
            dll_remove_node(&mut list, head);
            dll_destroy_node(head);
            assert_eq!(to_vec(list), vec![1, 2, 3]);
            assert_eq!(to_vec_reverse(list), vec![3, 2, 1]);
            destroy(&mut list);
        }
    }

    #[test]
    fn remove_middle_keeps_links() {
        unsafe {
            let mut list: *mut Node = ptr::null_mut();
            for i in 0..4 {
                dll_append_node(&mut list, dll_create_node(i));
            }
            let mid = dll_get_node_at(list, 1);
            dll_remove_node(&mut list, mid);
            dll_destroy_node(mid);
            assert_eq!(to_vec(list), vec![0, 2, 3]);
            assert_eq!(to_vec_reverse(list), vec![3, 2, 0]);
            destroy(&mut list);
        }
    }
}
