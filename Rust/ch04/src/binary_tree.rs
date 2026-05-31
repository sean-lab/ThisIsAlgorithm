// 이진 트리 (Binary Tree)
// Clang/04/BinaryTree 포팅. Box 기반 안전한 소유 트리로 구성.

pub type ElementType = char;

pub struct SBTNode {
    pub left: Option<Box<SBTNode>>,
    pub right: Option<Box<SBTNode>>,
    pub data: ElementType,
}

impl SBTNode {
    pub fn new(data: ElementType) -> Box<SBTNode> {
        Box::new(SBTNode {
            left: None,
            right: None,
            data,
        })
    }
}

pub fn preorder_print_tree(node: &Option<Box<SBTNode>>) {
    if let Some(n) = node {
        print!(" {}", n.data);
        preorder_print_tree(&n.left);
        preorder_print_tree(&n.right);
    }
}

pub fn inorder_print_tree(node: &Option<Box<SBTNode>>) {
    if let Some(n) = node {
        inorder_print_tree(&n.left);
        print!(" {}", n.data);
        inorder_print_tree(&n.right);
    }
}

pub fn postorder_print_tree(node: &Option<Box<SBTNode>>) {
    if let Some(n) = node {
        postorder_print_tree(&n.left);
        postorder_print_tree(&n.right);
        print!(" {}", n.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn preorder(node: &Option<Box<SBTNode>>, out: &mut Vec<char>) {
        if let Some(n) = node {
            out.push(n.data);
            preorder(&n.left, out);
            preorder(&n.right, out);
        }
    }

    fn inorder(node: &Option<Box<SBTNode>>, out: &mut Vec<char>) {
        if let Some(n) = node {
            inorder(&n.left, out);
            out.push(n.data);
            inorder(&n.right, out);
        }
    }

    fn postorder(node: &Option<Box<SBTNode>>, out: &mut Vec<char>) {
        if let Some(n) = node {
            postorder(&n.left, out);
            postorder(&n.right, out);
            out.push(n.data);
        }
    }

    fn sample() -> Option<Box<SBTNode>> {
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
        Some(a)
    }

    #[test]
    fn preorder_order() {
        let mut out = Vec::new();
        preorder(&sample(), &mut out);
        assert_eq!(out, vec!['A', 'B', 'C', 'D', 'E', 'F', 'G']);
    }

    #[test]
    fn inorder_order() {
        let mut out = Vec::new();
        inorder(&sample(), &mut out);
        assert_eq!(out, vec!['C', 'B', 'D', 'A', 'F', 'E', 'G']);
    }

    #[test]
    fn postorder_order() {
        let mut out = Vec::new();
        postorder(&sample(), &mut out);
        assert_eq!(out, vec!['C', 'D', 'B', 'F', 'G', 'E', 'A']);
    }
}
