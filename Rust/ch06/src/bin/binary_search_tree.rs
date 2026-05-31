// 이진 탐색 트리 (Binary Search Tree)
// Clang/06/BinarySearchTree 포팅. 로직은 ch06::binary_search_tree 모듈에 있다.
use ch06::binary_search_tree::{
    bst_create_node, bst_destroy_node, bst_destroy_tree, bst_inorder_print_tree, bst_insert_node,
    bst_remove_node, bst_search_node, BSTNode,
};
use std::ptr;

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
