// 순차 탐색 + 자기구성 탐색 (Sequential Search / Move-To-Front / Transpose)
// Clang/06/SequentialSearch 포팅. 단순 연결 리스트 기반이므로 raw pointer 사용.
use std::ptr;

pub struct Node {
    pub data: i32,
    pub next: *mut Node,
}

pub unsafe fn sll_create_node(new_data: i32) -> *mut Node {
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

pub unsafe fn sll_sequential_search(head: *mut Node, target: i32) -> *mut Node {
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

pub unsafe fn sll_move_to_front(head: &mut *mut Node, target: i32) -> *mut Node {
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

pub unsafe fn sll_transpose(head: &mut *mut Node, target: i32) -> *mut Node {
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

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn build(values: &[i32]) -> *mut Node {
        let mut head: *mut Node = ptr::null_mut();
        for &v in values {
            sll_append_node(&mut head, sll_create_node(v));
        }
        head
    }

    unsafe fn to_vec(head: *mut Node) -> Vec<i32> {
        let mut out = Vec::new();
        let mut cur = head;
        while !cur.is_null() {
            out.push((*cur).data);
            cur = (*cur).next;
        }
        out
    }

    unsafe fn destroy(head: &mut *mut Node) {
        while !(*head).is_null() {
            let node = sll_get_node_at(*head, 0);
            sll_remove_node(head, node);
            sll_destroy_node(node);
        }
    }

    #[test]
    fn sequential_search_finds_and_misses() {
        unsafe {
            let mut head = build(&[1, 2, 6, 10, 4]);
            assert_eq!((*sll_sequential_search(head, 6)).data, 6);
            assert!(sll_sequential_search(head, 99).is_null());
            destroy(&mut head);
        }
    }

    #[test]
    fn move_to_front_moves_match_to_head() {
        unsafe {
            let mut head = build(&[1, 2, 6, 10, 4]);
            let m = sll_move_to_front(&mut head, 10);
            assert_eq!((*m).data, 10);
            assert_eq!(to_vec(head), vec![10, 1, 2, 6, 4]);
            destroy(&mut head);
        }
    }

    #[test]
    fn transpose_swaps_with_previous() {
        unsafe {
            let mut head = build(&[1, 2, 6, 10, 4]);
            let m = sll_transpose(&mut head, 10);
            assert_eq!((*m).data, 10);
            assert_eq!(to_vec(head), vec![1, 2, 10, 6, 4]);
            destroy(&mut head);
        }
    }
}
