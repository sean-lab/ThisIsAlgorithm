// LCRS 트리 (Left-Child Right-Sibling Tree)
// Clang/04/LCRSTree 포팅. 로직은 ch04::lcrs_tree 모듈에 있다.
use ch04::lcrs_tree::{lcrs_add_child_node, lcrs_create_node, lcrs_destroy_tree, lcrs_print_tree};

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
