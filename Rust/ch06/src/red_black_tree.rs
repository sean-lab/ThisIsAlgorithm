// 레드 블랙 트리 (Red-Black Tree)
// Clang/06/RedBlackTree 포팅. Nil 센티넬과 부모 포인터, 회전이 필요하므로 raw pointer 사용.
use std::ptr;

pub type ElementType = i32;

pub const RED: i32 = 0;
pub const BLACK: i32 = 1;

pub struct RBTNode {
    pub parent: *mut RBTNode,
    pub left: *mut RBTNode,
    pub right: *mut RBTNode,
    pub color: i32,
    pub data: ElementType,
}

// 전역 Nil 센티넬 (원본 C의 `RBTNode* Nil;` 에 대응)
static mut NIL: *mut RBTNode = ptr::null_mut();

pub unsafe fn nil() -> *mut RBTNode {
    NIL
}

// 원본 C main 의 `Nil = RBT_CreateNode(0); Nil->Color = BLACK;` 에 대응.
pub unsafe fn rbt_init_nil() {
    NIL = rbt_create_node(0);
    (*NIL).color = BLACK;
}

pub unsafe fn rbt_create_node(new_data: ElementType) -> *mut RBTNode {
    Box::into_raw(Box::new(RBTNode {
        parent: ptr::null_mut(),
        left: ptr::null_mut(),
        right: ptr::null_mut(),
        color: BLACK,
        data: new_data,
    }))
}

pub unsafe fn rbt_destroy_node(node: *mut RBTNode) {
    drop(Box::from_raw(node));
}

pub unsafe fn rbt_destroy_tree(tree: *mut RBTNode) {
    if tree.is_null() || tree == nil() {
        return;
    }
    if (*tree).right != nil() {
        rbt_destroy_tree((*tree).right);
    }
    if (*tree).left != nil() {
        rbt_destroy_tree((*tree).left);
    }
    (*tree).left = nil();
    (*tree).right = nil();
    rbt_destroy_node(tree);
}

pub unsafe fn rbt_search_node(tree: *mut RBTNode, target: ElementType) -> *mut RBTNode {
    if tree == nil() {
        return ptr::null_mut();
    }
    if (*tree).data > target {
        rbt_search_node((*tree).left, target)
    } else if (*tree).data < target {
        rbt_search_node((*tree).right, target)
    } else {
        tree
    }
}

pub unsafe fn rbt_search_min_node(tree: *mut RBTNode) -> *mut RBTNode {
    if tree == nil() {
        return nil();
    }
    if (*tree).left == nil() {
        tree
    } else {
        rbt_search_min_node((*tree).left)
    }
}

pub unsafe fn rbt_insert_node(tree: &mut *mut RBTNode, new_node: *mut RBTNode) {
    rbt_insert_node_helper(tree, new_node);

    (*new_node).color = RED;
    (*new_node).left = nil();
    (*new_node).right = nil();

    rbt_rebuild_after_insert(tree, new_node);
}

unsafe fn rbt_insert_node_helper(tree: &mut *mut RBTNode, new_node: *mut RBTNode) {
    if (*tree).is_null() {
        *tree = new_node;
    }

    if (*(*tree)).data < (*new_node).data {
        if (*(*tree)).right == nil() {
            (*(*tree)).right = new_node;
            (*new_node).parent = *tree;
        } else {
            rbt_insert_node_helper(&mut (*(*tree)).right, new_node);
        }
    } else if (*(*tree)).data > (*new_node).data {
        if (*(*tree)).left == nil() {
            (*(*tree)).left = new_node;
            (*new_node).parent = *tree;
        } else {
            rbt_insert_node_helper(&mut (*(*tree)).left, new_node);
        }
    }
}

unsafe fn rbt_rotate_right(root: &mut *mut RBTNode, parent: *mut RBTNode) {
    let left_child = (*parent).left;

    (*parent).left = (*left_child).right;

    if (*left_child).right != nil() {
        (*(*left_child).right).parent = parent;
    }

    (*left_child).parent = (*parent).parent;

    if (*parent).parent.is_null() {
        *root = left_child;
    } else if parent == (*(*parent).parent).left {
        (*(*parent).parent).left = left_child;
    } else {
        (*(*parent).parent).right = left_child;
    }

    (*left_child).right = parent;
    (*parent).parent = left_child;
}

unsafe fn rbt_rotate_left(root: &mut *mut RBTNode, parent: *mut RBTNode) {
    let right_child = (*parent).right;

    (*parent).right = (*right_child).left;

    if (*right_child).left != nil() {
        (*(*right_child).left).parent = parent;
    }

    (*right_child).parent = (*parent).parent;

    if (*parent).parent.is_null() {
        *root = right_child;
    } else if parent == (*(*parent).parent).left {
        (*(*parent).parent).left = right_child;
    } else {
        (*(*parent).parent).right = right_child;
    }

    (*right_child).left = parent;
    (*parent).parent = right_child;
}

unsafe fn rbt_rebuild_after_insert(root: &mut *mut RBTNode, mut x: *mut RBTNode) {
    while x != *root && (*(*x).parent).color == RED {
        if (*x).parent == (*(*(*x).parent).parent).left {
            let uncle = (*(*(*x).parent).parent).right;
            if (*uncle).color == RED {
                (*(*x).parent).color = BLACK;
                (*uncle).color = BLACK;
                (*(*(*x).parent).parent).color = RED;

                x = (*(*x).parent).parent;
            } else {
                if x == (*(*x).parent).right {
                    x = (*x).parent;
                    rbt_rotate_left(root, x);
                }

                (*(*x).parent).color = BLACK;
                (*(*(*x).parent).parent).color = RED;

                rbt_rotate_right(root, (*(*x).parent).parent);
            }
        } else {
            let uncle = (*(*(*x).parent).parent).left;
            if (*uncle).color == RED {
                (*(*x).parent).color = BLACK;
                (*uncle).color = BLACK;
                (*(*(*x).parent).parent).color = RED;

                x = (*(*x).parent).parent;
            } else {
                if x == (*(*x).parent).left {
                    x = (*x).parent;
                    rbt_rotate_right(root, x);
                }

                (*(*x).parent).color = BLACK;
                (*(*(*x).parent).parent).color = RED;
                rbt_rotate_left(root, (*(*x).parent).parent);
            }
        }
    }

    (*(*root)).color = BLACK;
}

pub unsafe fn rbt_remove_node(root: &mut *mut RBTNode, data: ElementType) -> *mut RBTNode {
    let target = rbt_search_node(*root, data);

    if target.is_null() {
        return ptr::null_mut();
    }

    let removed: *mut RBTNode;
    if (*target).left == nil() || (*target).right == nil() {
        removed = target;
    } else {
        removed = rbt_search_min_node((*target).right);
        (*target).data = (*removed).data;
    }

    let successor = if (*removed).left != nil() {
        (*removed).left
    } else {
        (*removed).right
    };

    (*successor).parent = (*removed).parent;

    if (*removed).parent.is_null() {
        *root = successor;
    } else if removed == (*(*removed).parent).left {
        (*(*removed).parent).left = successor;
    } else {
        (*(*removed).parent).right = successor;
    }

    if (*removed).color == BLACK {
        rbt_rebuild_after_remove(root, successor);
    }

    removed
}

unsafe fn rbt_rebuild_after_remove(root: &mut *mut RBTNode, mut successor: *mut RBTNode) {
    while !(*successor).parent.is_null() && (*successor).color == BLACK {
        if successor == (*(*successor).parent).left {
            let mut sibling = (*(*successor).parent).right;

            if (*sibling).color == RED {
                (*sibling).color = BLACK;
                (*(*successor).parent).color = RED;
                rbt_rotate_left(root, (*successor).parent);
            } else if (*(*sibling).left).color == BLACK && (*(*sibling).right).color == BLACK {
                (*sibling).color = RED;
                successor = (*successor).parent;
            } else {
                if (*(*sibling).left).color == RED {
                    (*(*sibling).left).color = BLACK;
                    (*sibling).color = RED;

                    rbt_rotate_right(root, sibling);
                    sibling = (*(*successor).parent).right;
                }

                (*sibling).color = (*(*successor).parent).color;
                (*(*successor).parent).color = BLACK;
                (*(*sibling).right).color = BLACK;
                rbt_rotate_left(root, (*successor).parent);
                successor = *root;
            }
        } else {
            let mut sibling = (*(*successor).parent).left;

            if (*sibling).color == RED {
                (*sibling).color = BLACK;
                (*(*successor).parent).color = RED;
                rbt_rotate_right(root, (*successor).parent);
            } else if (*(*sibling).right).color == BLACK && (*(*sibling).left).color == BLACK {
                (*sibling).color = RED;
                successor = (*successor).parent;
            } else {
                if (*(*sibling).right).color == RED {
                    (*(*sibling).right).color = BLACK;
                    (*sibling).color = RED;

                    rbt_rotate_left(root, sibling);
                    sibling = (*(*successor).parent).left;
                }

                (*sibling).color = (*(*successor).parent).color;
                (*(*successor).parent).color = BLACK;
                (*(*sibling).left).color = BLACK;
                rbt_rotate_right(root, (*successor).parent);
                successor = *root;
            }
        }
    }

    (*successor).color = BLACK;
}

pub unsafe fn rbt_print_tree(node: *mut RBTNode, depth: i32, mut black_count: i32) {
    if node.is_null() || node == nil() {
        return;
    }

    if (*node).color == BLACK {
        black_count += 1;
    }

    let mut c = 'X';
    let mut v = -1;

    if !(*node).parent.is_null() {
        v = (*(*node).parent).data;

        if (*(*node).parent).left == node {
            c = 'L';
        } else {
            c = 'R';
        }
    }

    let cnt = if (*node).left == nil() && (*node).right == nil() {
        format!("--------- {}", black_count)
    } else {
        String::new()
    };

    for _ in 0..depth {
        print!("  ");
    }

    println!(
        "{} {} [{},{}] {}",
        (*node).data,
        if (*node).color == RED { "RED" } else { "BLACK" },
        c,
        v,
        cnt
    );

    rbt_print_tree((*node).left, depth + 1, black_count);
    rbt_print_tree((*node).right, depth + 1, black_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    // 전역 NIL 센티넬을 사용하므로 모든 검증을 하나의 테스트에 모아 경합을 피한다.
    #[test]
    fn insert_search_remove_maintains_invariants() {
        unsafe {
            rbt_init_nil();
            let mut tree: *mut RBTNode = ptr::null_mut();

            for v in [10, 20, 30, 15, 25, 5, 1, 40, 35, 50] {
                rbt_insert_node(&mut tree, rbt_create_node(v));
            }

            // 루트는 항상 검정색이어야 한다.
            assert_eq!((*tree).color, BLACK);

            // 중위 순회는 정렬된 순서여야 한다.
            let mut out = Vec::new();
            inorder(tree, &mut out);
            assert_eq!(out, vec![1, 5, 10, 15, 20, 25, 30, 35, 40, 50]);

            // 검색이 동작한다.
            assert_eq!((*rbt_search_node(tree, 25)).data, 25);
            assert!(rbt_search_node(tree, 999).is_null());

            // 삭제 후에도 정렬/루트 검정 불변식이 유지된다.
            let node = rbt_remove_node(&mut tree, 20);
            assert!(!node.is_null());
            rbt_destroy_node(node);
            assert_eq!((*tree).color, BLACK);

            let mut out2 = Vec::new();
            inorder(tree, &mut out2);
            assert_eq!(out2, vec![1, 5, 10, 15, 25, 30, 35, 40, 50]);

            rbt_destroy_tree(tree);
        }
    }

    unsafe fn inorder(node: *mut RBTNode, out: &mut Vec<i32>) {
        if node.is_null() || node == nil() {
            return;
        }
        inorder((*node).left, out);
        out.push((*node).data);
        inorder((*node).right, out);
    }
}
