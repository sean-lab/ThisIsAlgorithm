// 이진 탐색 트리 (Binary Search Tree)
// Clang/06/BinarySearchTree 포팅. 부모 포인터 조작이 필요하므로 raw pointer 사용.
use std::ptr;

type ElementType = i32;

struct BSTNode {
    left: *mut BSTNode,
    right: *mut BSTNode,
    data: ElementType,
}

unsafe fn bst_create_node(new_data: ElementType) -> *mut BSTNode {
    Box::into_raw(Box::new(BSTNode {
        left: ptr::null_mut(),
        right: ptr::null_mut(),
        data: new_data,
    }))
}

unsafe fn bst_destroy_node(node: *mut BSTNode) {
    drop(Box::from_raw(node));
}

unsafe fn bst_destroy_tree(tree: *mut BSTNode) {
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

unsafe fn bst_search_node(tree: *mut BSTNode, target: ElementType) -> *mut BSTNode {
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

unsafe fn bst_search_min_node(tree: *mut BSTNode) -> *mut BSTNode {
    if tree.is_null() {
        return ptr::null_mut();
    }
    if (*tree).left.is_null() {
        tree
    } else {
        bst_search_min_node((*tree).left)
    }
}

unsafe fn bst_insert_node(tree: *mut BSTNode, child: *mut BSTNode) {
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

unsafe fn bst_remove_node(
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

unsafe fn bst_inorder_print_tree(node: *mut BSTNode) {
    if node.is_null() {
        return;
    }
    bst_inorder_print_tree((*node).left);
    print!("{} ", (*node).data);
    bst_inorder_print_tree((*node).right);
}

fn print_search_result(search_target: i32, result: *mut BSTNode) {
    unsafe {
        if !result.is_null() {
            println!("Found : {} ", (*result).data);
        } else {
            println!("Not Found:  {}", search_target);
        }
    }
}

fn main() {
    unsafe {
        let tree = bst_create_node(123);

        bst_insert_node(tree, bst_create_node(22));
        bst_insert_node(tree, bst_create_node(9918));
        bst_insert_node(tree, bst_create_node(424));
        bst_insert_node(tree, bst_create_node(17));
        bst_insert_node(tree, bst_create_node(3));

        bst_insert_node(tree, bst_create_node(98));
        bst_insert_node(tree, bst_create_node(34));

        bst_insert_node(tree, bst_create_node(760));
        bst_insert_node(tree, bst_create_node(317));
        bst_insert_node(tree, bst_create_node(1));

        let mut search_target = 17;
        let node = bst_search_node(tree, search_target);
        print_search_result(search_target, node);

        search_target = 117;
        let node = bst_search_node(tree, search_target);
        print_search_result(search_target, node);

        bst_inorder_print_tree(tree);
        println!();

        // 특정 노드 삭제
        println!("Removing 98...");
        let node = bst_remove_node(tree, ptr::null_mut(), 98);
        bst_destroy_node(node);

        bst_inorder_print_tree(tree);
        println!();

        // 새 노드 삽입
        println!("Inserting 111...");
        bst_insert_node(tree, bst_create_node(111));
        bst_inorder_print_tree(tree);
        println!();

        bst_destroy_tree(tree);
    }
}
