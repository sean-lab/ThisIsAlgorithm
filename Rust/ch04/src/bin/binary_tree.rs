// 이진 트리 (Binary Tree)
// Clang/04/BinaryTree 포팅. Box 기반 안전한 소유 트리로 구성.

type ElementType = char;

struct SBTNode {
    left: Option<Box<SBTNode>>,
    right: Option<Box<SBTNode>>,
    data: ElementType,
}

impl SBTNode {
    fn new(data: ElementType) -> Box<SBTNode> {
        Box::new(SBTNode {
            left: None,
            right: None,
            data,
        })
    }
}

fn preorder_print_tree(node: &Option<Box<SBTNode>>) {
    if let Some(n) = node {
        print!(" {}", n.data);
        preorder_print_tree(&n.left);
        preorder_print_tree(&n.right);
    }
}

fn inorder_print_tree(node: &Option<Box<SBTNode>>) {
    if let Some(n) = node {
        inorder_print_tree(&n.left);
        print!(" {}", n.data);
        inorder_print_tree(&n.right);
    }
}

fn postorder_print_tree(node: &Option<Box<SBTNode>>) {
    if let Some(n) = node {
        postorder_print_tree(&n.left);
        postorder_print_tree(&n.right);
        print!(" {}", n.data);
    }
}

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
