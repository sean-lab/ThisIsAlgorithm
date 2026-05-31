// 순차 탐색 + 자기구성 탐색 (Sequential Search / Move-To-Front / Transpose)
// Clang/06/SequentialSearch 포팅. 로직은 ch06::sequential_search 모듈에 있다.
use ch06::sequential_search::{
    sll_append_node, sll_create_node, sll_destroy_node, sll_get_node_at, sll_move_to_front,
    sll_remove_node, sll_sequential_search, sll_transpose, Node,
};
use std::ptr;

fn main() {
    unsafe {
        let count = 10;
        let mut list: *mut Node = ptr::null_mut();
        let init_value = [1, 2, 6, 10, 4, 9, 5, 3, 8, 7];

        for &v in init_value.iter() {
            let new_node = sll_create_node(v);
            sll_append_node(&mut list, new_node);
        }

        // 순차 탐색
        println!("Simple Sequential Search...");
        let matched = sll_sequential_search(list, 9);
        if !matched.is_null() {
            println!("Found : {}", (*matched).data);
        } else {
            println!("Not Found");
        }

        // 전진 이동법
        println!("Move To Front...");
        let matched = sll_move_to_front(&mut list, 4);
        if !matched.is_null() {
            println!("Found : {}", (*matched).data);
        } else {
            println!("Not Found");
        }

        // 전위법
        println!("Transpose...");
        let matched = sll_transpose(&mut list, 7);
        if !matched.is_null() {
            println!("Found : {}", (*matched).data);
        } else {
            println!("Not Found");
        }

        // 모든 노드를 메모리에서 제거
        println!("Destroying List...");
        for _ in 0..count {
            let current = sll_get_node_at(list, 0);
            if !current.is_null() {
                sll_remove_node(&mut list, current);
                sll_destroy_node(current);
            }
        }
    }
}
