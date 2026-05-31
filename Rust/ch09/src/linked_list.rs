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
