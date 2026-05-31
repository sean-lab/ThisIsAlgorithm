// 레드 블랙 트리 (Red-Black Tree)
// Clang/06/RedBlackTree 포팅. 로직은 ch06::red_black_tree 모듈에 있다.
use ch06::red_black_tree::{
    rbt_create_node, rbt_destroy_node, rbt_destroy_tree, rbt_init_nil, rbt_insert_node,
    rbt_print_tree, rbt_remove_node, rbt_search_node, RBTNode, RED,
};
use std::io::{self, BufRead, Write};
use std::ptr;

fn main() {
    unsafe {
        rbt_init_nil();

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
