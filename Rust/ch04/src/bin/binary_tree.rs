// 이진 트리 (Binary Tree)
// Clang/04/BinaryTree 포팅. 로직은 ch04::binary_tree 모듈에 있다.
use ch04::binary_tree::{inorder_print_tree, postorder_print_tree, preorder_print_tree, SBTNode};

fn main() {
    // 노드 생성 및 트리 구성 (C 테스트와 동일한 구조)
    //        A
    //      /   \
    //     B     E
    //    / \   / \
    //   C   D F   G
    let mut b = SBTNode::new('B');
    b.left = Some(SBTNode::new('C'));
    b.right = Some(SBTNode::new('D'));

    let mut e = SBTNode::new('E');
    e.left = Some(SBTNode::new('F'));
    e.right = Some(SBTNode::new('G'));

    let mut a = SBTNode::new('A');
    a.left = Some(b);
    a.right = Some(e);

    let root = Some(a);

    // 트리 출력
    println!("Preorder ...");
    preorder_print_tree(&root);
    println!("\n");

    println!("Inorder ... ");
    inorder_print_tree(&root);
    println!("\n");

    println!("Postorder ... ");
    postorder_print_tree(&root);
    println!();
}
