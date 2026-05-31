// 이진 탐색 트리 (Binary Search Tree)
// Clang/06/BinarySearchTree 포팅. 부모 포인터 조작이 필요하므로 raw pointer 사용.
use std::ptr;

pub type ElementType = i32;

pub struct BSTNode {
    pub left: *mut BSTNode,
    pub right: *mut BSTNode,
    pub data: ElementType,
}

pub unsafe fn bst_create_node(new_data: ElementType) -> *mut BSTNode {
    Box::into_raw(Box::new(BSTNode {
        left: ptr::null_mut(),
        right: ptr::null_mut(),
        data: new_data,
    }))
}

pub unsafe fn bst_destroy_node(node: *mut BSTNode) {
    drop(Box::from_raw(node));
}

pub unsafe fn bst_destroy_tree(tree: *mut BSTNode) {
    if tree.is_null() {
        return;
    }
    if !(*tree).right.is_null() {
        bst_destroy_tree((*tree).right);
    }
    if !(*tree).left.is_null() {
        bst_destroy_tree((*tree).left);
    }
    (*tree).left = ptr::null_mut();
    (*tree).right = ptr::null_mut();
    bst_destroy_node(tree);
}

pub unsafe fn bst_search_node(tree: *mut BSTNode, target: ElementType) -> *mut BSTNode {
    if tree.is_null() {
        return ptr::null_mut();
    }
    if (*tree).data == target {
        tree
    } else if (*tree).data > target {
        bst_search_node((*tree).left, target)
    } else {
        bst_search_node((*tree).right, target)
    }
}

pub unsafe fn bst_search_min_node(tree: *mut BSTNode) -> *mut BSTNode {
    if tree.is_null() {
        return ptr::null_mut();
    }
    if (*tree).left.is_null() {
        tree
    } else {
        bst_search_min_node((*tree).left)
    }
}

pub unsafe fn bst_insert_node(tree: *mut BSTNode, child: *mut BSTNode) {
    if (*tree).data < (*child).data {
        if (*tree).right.is_null() {
            (*tree).right = child;
        } else {
            bst_insert_node((*tree).right, child);
        }
    } else if (*tree).data > (*child).data {
        if (*tree).left.is_null() {
            (*tree).left = child;
        } else {
            bst_insert_node((*tree).left, child);
        }
    }
}

pub unsafe fn bst_remove_node(
    tree: *mut BSTNode,
    parent: *mut BSTNode,
    target: ElementType,
) -> *mut BSTNode {
    let mut removed: *mut BSTNode;

    if tree.is_null() {
        return ptr::null_mut();
    }

    if (*tree).data > target {
        removed = bst_remove_node((*tree).left, tree, target);
    } else if (*tree).data < target {
        removed = bst_remove_node((*tree).right, tree, target);
    } else {
        // 목표 값을 찾은 경우.
        removed = tree;

        // 잎 노드인 경우 바로 삭제
        if (*tree).left.is_null() && (*tree).right.is_null() {
            if (*parent).left == tree {
                (*parent).left = ptr::null_mut();
            } else {
                (*parent).right = ptr::null_mut();
            }
        } else {
            // 자식이 양쪽 다 있는 경우
            if !(*tree).left.is_null() && !(*tree).right.is_null() {
                // 최소값 노드를 찾아 제거한 뒤 현재의 노드에 위치시킨다.
                let min_node = bst_search_min_node((*tree).right);
                let min_node = bst_remove_node(tree, ptr::null_mut(), (*min_node).data);
                (*tree).data = (*min_node).data;
                removed = min_node;
            } else {
                // 자식이 하나만 있는 경우
                let temp = if !(*tree).left.is_null() {
                    (*tree).left
                } else {
                    (*tree).right
                };

                if (*parent).left == tree {
                    (*parent).left = temp;
                } else {
                    (*parent).right = temp;
                }
            }
        }
    }

    removed
}

pub unsafe fn bst_inorder_print_tree(node: *mut BSTNode) {
    if node.is_null() {
        return;
    }
    bst_inorder_print_tree((*node).left);
    print!("{} ", (*node).data);
    bst_inorder_print_tree((*node).right);
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn inorder(node: *mut BSTNode, out: &mut Vec<i32>) {
        if node.is_null() {
            return;
        }
        inorder((*node).left, out);
        out.push((*node).data);
        inorder((*node).right, out);
    }

    unsafe fn build() -> *mut BSTNode {
        let tree = bst_create_node(123);
        for v in [22, 9918, 424, 17, 3, 98, 34, 760, 317, 1] {
            bst_insert_node(tree, bst_create_node(v));
        }
        tree
    }

    #[test]
    fn search_finds_and_misses() {
        unsafe {
            let tree = build();
            assert_eq!((*bst_search_node(tree, 17)).data, 17);
            assert!(bst_search_node(tree, 117).is_null());
            bst_destroy_tree(tree);
        }
    }

    #[test]
    fn inorder_is_sorted() {
        unsafe {
            let tree = build();
            let mut out = Vec::new();
            inorder(tree, &mut out);
            assert_eq!(out, vec![1, 3, 17, 22, 34, 98, 123, 317, 424, 760, 9918]);
            bst_destroy_tree(tree);
        }
    }

    #[test]
    fn remove_keeps_order() {
        unsafe {
            let tree = build();
            let node = bst_remove_node(tree, ptr::null_mut(), 98);
            bst_destroy_node(node);
            let mut out = Vec::new();
            inorder(tree, &mut out);
            assert_eq!(out, vec![1, 3, 17, 22, 34, 123, 317, 424, 760, 9918]);
            bst_destroy_tree(tree);
        }
    }
}
