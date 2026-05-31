// LCRS 트리 (Left-Child Right-Sibling Tree)
// Clang/04/LCRSTree 포팅. 노드 별칭(aliasing)이 필요하므로 raw pointer로 옮겼다.
use std::ptr;

type ElementType = char;

struct LCRSNode {
    left_child: *mut LCRSNode,
    right_sibling: *mut LCRSNode,
    data: ElementType,
}

unsafe fn lcrs_create_node(new_data: ElementType) -> *mut LCRSNode {
    Box::into_raw(Box::new(LCRSNode {
        left_child: ptr::null_mut(),
        right_sibling: ptr::null_mut(),
        data: new_data,
    }))
}

unsafe fn lcrs_destroy_node(node: *mut LCRSNode) {
    drop(Box::from_raw(node));
}

unsafe fn lcrs_destroy_tree(root: *mut LCRSNode) {
    if !(*root).right_sibling.is_null() {
        lcrs_destroy_tree((*root).right_sibling);
    }
    if !(*root).left_child.is_null() {
        lcrs_destroy_tree((*root).left_child);
    }
    (*root).left_child = ptr::null_mut();
    (*root).right_sibling = ptr::null_mut();
    lcrs_destroy_node(root);
}

unsafe fn lcrs_add_child_node(parent: *mut LCRSNode, child: *mut LCRSNode) {
    if (*parent).left_child.is_null() {
        (*parent).left_child = child;
    } else {
        let mut temp = (*parent).left_child;
        while !(*temp).right_sibling.is_null() {
            temp = (*temp).right_sibling;
        }
        (*temp).right_sibling = child;
    }
}

unsafe fn lcrs_print_tree(node: *mut LCRSNode, depth: i32) {
    // 들여쓰기
    for _ in 0..depth - 1 {
        print!("   "); // 공백 3칸
    }

    if depth > 0 {
        print!("+--");
    }

    // 노드 데이터 출력
    println!("{}", (*node).data);

    if !(*node).left_child.is_null() {
        lcrs_print_tree((*node).left_child, depth + 1);
    }

    if !(*node).right_sibling.is_null() {
        lcrs_print_tree((*node).right_sibling, depth);
    }
}

fn main() {
    unsafe {
        let root = lcrs_create_node('A');

        let b = lcrs_create_node('B');
        let c = lcrs_create_node('C');
        let d = lcrs_create_node('D');
        let e = lcrs_create_node('E');
        let f = lcrs_create_node('F');
        let g = lcrs_create_node('G');
        let h = lcrs_create_node('H');
        let i = lcrs_create_node('I');
        let j = lcrs_create_node('J');
        let k = lcrs_create_node('K');

        // 트리에 노드 추가
        lcrs_add_child_node(root, b);
        lcrs_add_child_node(b, c);
        lcrs_add_child_node(b, d);
        lcrs_add_child_node(d, e);
        lcrs_add_child_node(d, f);

        lcrs_add_child_node(root, g);
        lcrs_add_child_node(g, h);

        lcrs_add_child_node(root, i);
        lcrs_add_child_node(i, j);
        lcrs_add_child_node(j, k);

        // 트리 출력
        lcrs_print_tree(root, 0);

        // 트리 소멸
        lcrs_destroy_tree(root);
    }
}
