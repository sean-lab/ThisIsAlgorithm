// LCRS 트리 (Left-Child Right-Sibling Tree)
// Clang/04/LCRSTree 포팅. 노드 별칭(aliasing)이 필요하므로 raw pointer로 옮겼다.
use std::ptr;

pub type ElementType = char;

pub struct LCRSNode {
    pub left_child: *mut LCRSNode,
    pub right_sibling: *mut LCRSNode,
    pub data: ElementType,
}

pub unsafe fn lcrs_create_node(new_data: ElementType) -> *mut LCRSNode {
    Box::into_raw(Box::new(LCRSNode {
        left_child: ptr::null_mut(),
        right_sibling: ptr::null_mut(),
        data: new_data,
    }))
}

pub unsafe fn lcrs_destroy_node(node: *mut LCRSNode) {
    drop(Box::from_raw(node));
}

pub unsafe fn lcrs_destroy_tree(root: *mut LCRSNode) {
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

pub unsafe fn lcrs_add_child_node(parent: *mut LCRSNode, child: *mut LCRSNode) {
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

pub unsafe fn lcrs_print_tree(node: *mut LCRSNode, depth: i32) {
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

#[cfg(test)]
mod tests {
    use super::*;

    // 부모의 자식들을 좌->우 형제 순서로 수집한다.
    unsafe fn children(parent: *mut LCRSNode) -> Vec<ElementType> {
        let mut out = Vec::new();
        let mut c = (*parent).left_child;
        while !c.is_null() {
            out.push((*c).data);
            c = (*c).right_sibling;
        }
        out
    }

    #[test]
    fn add_child_appends_as_sibling() {
        unsafe {
            let root = lcrs_create_node('A');
            let b = lcrs_create_node('B');
            let g = lcrs_create_node('G');
            let i = lcrs_create_node('I');
            lcrs_add_child_node(root, b);
            lcrs_add_child_node(root, g);
            lcrs_add_child_node(root, i);
            assert_eq!(children(root), vec!['B', 'G', 'I']);
            lcrs_destroy_tree(root);
        }
    }

    #[test]
    fn nested_children() {
        unsafe {
            let root = lcrs_create_node('A');
            let b = lcrs_create_node('B');
            let c = lcrs_create_node('C');
            let d = lcrs_create_node('D');
            lcrs_add_child_node(root, b);
            lcrs_add_child_node(b, c);
            lcrs_add_child_node(b, d);
            assert_eq!(children(root), vec!['B']);
            assert_eq!(children(b), vec!['C', 'D']);
            lcrs_destroy_tree(root);
        }
    }
}
