// 단일 연결 리스트 (위상 정렬용) - Data 는 Vertex 포인터
use crate::graph::Vertex;
use std::ptr;

pub type ElementType = *mut Vertex;

pub struct Node {
    pub data: ElementType,
    pub next_node: *mut Node,
}

pub unsafe fn sll_create_node(new_data: ElementType) -> *mut Node {
    Box::into_raw(Box::new(Node {
        data: new_data,
        next_node: ptr::null_mut(),
    }))
}

pub unsafe fn sll_insert_new_head(head: &mut *mut Node, new_head: *mut Node) {
    (*new_head).next_node = *head;
    *head = new_head;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{create_vertex, destroy_vertex};

    #[test]
    fn insert_new_head_prepends() {
        unsafe {
            let v1 = create_vertex(b'A' as i32);
            let v2 = create_vertex(b'B' as i32);

            let mut head: *mut Node = ptr::null_mut();
            sll_insert_new_head(&mut head, sll_create_node(v1));
            sll_insert_new_head(&mut head, sll_create_node(v2));

            // 마지막에 넣은 v2 가 헤드.
            assert_eq!((*(*head).data).data, b'B' as i32);
            assert_eq!((*(*(*head).next_node).data).data, b'A' as i32);

            // 노드 정리
            let mut cur = head;
            while !cur.is_null() {
                let next = (*cur).next_node;
                drop(Box::from_raw(cur));
                cur = next;
            }
            destroy_vertex(v1);
            destroy_vertex(v2);
        }
    }
}
