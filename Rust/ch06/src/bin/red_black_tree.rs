// 레드 블랙 트리 (Red-Black Tree)
// Clang/06/RedBlackTree 포팅. Nil 센티넬과 부모 포인터, 회전이 필요하므로 raw pointer 사용.
use std::io::{self, BufRead, Write};
use std::ptr;

type ElementType = i32;

const RED: i32 = 0;
const BLACK: i32 = 1;

struct RBTNode {
    parent: *mut RBTNode,
    left: *mut RBTNode,
    right: *mut RBTNode,
    color: i32,
    data: ElementType,
}

// 전역 Nil 센티넬 (원본 C의 `RBTNode* Nil;` 에 대응)
static mut NIL: *mut RBTNode = ptr::null_mut();

unsafe fn nil() -> *mut RBTNode {
    NIL
}

unsafe fn rbt_create_node(new_data: ElementType) -> *mut RBTNode {
    Box::into_raw(Box::new(RBTNode {
        parent: ptr::null_mut(),
        left: ptr::null_mut(),
        right: ptr::null_mut(),
        color: BLACK,
        data: new_data,
    }))
}

unsafe fn rbt_destroy_node(node: *mut RBTNode) {
    drop(Box::from_raw(node));
}

unsafe fn rbt_destroy_tree(tree: *mut RBTNode) {
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

unsafe fn rbt_search_node(tree: *mut RBTNode, target: ElementType) -> *mut RBTNode {
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

unsafe fn rbt_search_min_node(tree: *mut RBTNode) -> *mut RBTNode {
    if tree == nil() {
        return nil();
    }
    if (*tree).left == nil() {
        tree
    } else {
        rbt_search_min_node((*tree).left)
    }
}

unsafe fn rbt_insert_node(tree: &mut *mut RBTNode, new_node: *mut RBTNode) {
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

unsafe fn rbt_remove_node(root: &mut *mut RBTNode, data: ElementType) -> *mut RBTNode {
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

unsafe fn rbt_print_tree(node: *mut RBTNode, depth: i32, mut black_count: i32) {
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

fn main() {
    unsafe {
        NIL = rbt_create_node(0);
        (*NIL).color = BLACK;

        let mut tree: *mut RBTNode = ptr::null_mut();

        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        loop {
            println!("Enter command number :");
            println!("(1) Create a node, (2) Remove a node, (3) Search a Node");
            println!("(4) Display Tree (5) quit");
            print!("command number:");
            io::stdout().flush().ok();

            let line = match lines.next() {
                Some(Ok(l)) => l,
                _ => break,
            };
            let cmd: i32 = line.trim().parse().unwrap_or(0);

            if !(1..=5).contains(&cmd) {
                println!("Invalid command number.");
                continue;
            } else if cmd == 4 {
                rbt_print_tree(tree, 0, 0);
                println!();
                continue;
            } else if cmd == 5 {
                break;
            }

            println!("Enter parameter (1~200) :");

            let line = match lines.next() {
                Some(Ok(l)) => l,
                _ => break,
            };
            let param: i32 = line.trim().parse().unwrap_or(0);

            if !(1..=200).contains(&param) {
                println!("Invalid parameter.{}", param);
                continue;
            }

            match cmd {
                1 => rbt_insert_node(&mut tree, rbt_create_node(param)),
                2 => {
                    let node = rbt_remove_node(&mut tree, param);
                    if node.is_null() {
                        println!("Not found node to delete:{}", param);
                    } else {
                        rbt_destroy_node(node);
                    }
                }
                3 => {
                    let node = rbt_search_node(tree, param);
                    if node.is_null() {
                        println!("Not found node:{}", param);
                    } else {
                        println!(
                            "Found Node: {}(Color:{})",
                            (*node).data,
                            if (*node).color == RED { "RED" } else { "BLACK" }
                        );
                    }
                }
                _ => {}
            }

            println!();
        }

        rbt_destroy_tree(tree);
    }
}
