// 수식 트리 (Expression Tree)
// Clang/04/ExpressionTree 포팅. 후위표기식으로부터 트리를 구성하고 평가한다.

pub type ElementType = char;

pub struct ETNode {
    pub left: Option<Box<ETNode>>,
    pub right: Option<Box<ETNode>>,
    pub data: ElementType,
}

impl ETNode {
    pub fn new(data: ElementType) -> Box<ETNode> {
        Box::new(ETNode {
            left: None,
            right: None,
            data,
        })
    }
}

pub fn preorder_print_tree(node: &Option<Box<ETNode>>) {
    if let Some(n) = node {
        print!(" {}", n.data);
        preorder_print_tree(&n.left);
        preorder_print_tree(&n.right);
    }
}

pub fn inorder_print_tree(node: &Option<Box<ETNode>>) {
    if let Some(n) = node {
        print!("(");
        inorder_print_tree(&n.left);
        print!("{}", n.data);
        inorder_print_tree(&n.right);
        print!(")");
    }
}

pub fn postorder_print_tree(node: &Option<Box<ETNode>>) {
    if let Some(n) = node {
        postorder_print_tree(&n.left);
        postorder_print_tree(&n.right);
        print!(" {}", n.data);
    }
}

// C의 ET_BuildExpressionTree: 후위표기식의 끝에서부터 소비하며 트리를 구성한다.
pub fn build_expression_tree(postfix: &mut Vec<char>) -> Option<Box<ETNode>> {
    if postfix.is_empty() {
        return None;
    }
    let token = postfix.pop().unwrap();

    match token {
        '+' | '-' | '*' | '/' => {
            let mut node = ETNode::new(token);
            node.right = build_expression_tree(postfix);
            node.left = build_expression_tree(postfix);
            Some(node)
        }
        _ => Some(ETNode::new(token)),
    }
}

pub fn evaluate(tree: &Option<Box<ETNode>>) -> f64 {
    let node = match tree {
        Some(n) => n,
        None => return 0.0,
    };

    match node.data {
        '+' | '-' | '*' | '/' => {
            let left = evaluate(&node.left);
            let right = evaluate(&node.right);
            match node.data {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => 0.0,
            }
        }
        _ => node.data.to_string().parse().unwrap_or(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_sample() {
        // 후위표기 "71*52-/" = (7*1) / (5-2) = 7/3
        let mut postfix: Vec<char> = "71*52-/".chars().collect();
        let root = build_expression_tree(&mut postfix);
        assert!((evaluate(&root) - 7.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn evaluate_simple_ops() {
        let mut p: Vec<char> = "34+".chars().collect();
        assert_eq!(evaluate(&build_expression_tree(&mut p)), 7.0);
        let mut p: Vec<char> = "82-".chars().collect();
        assert_eq!(evaluate(&build_expression_tree(&mut p)), 6.0);
        let mut p: Vec<char> = "63*".chars().collect();
        assert_eq!(evaluate(&build_expression_tree(&mut p)), 18.0);
    }

    #[test]
    fn build_preorder_structure() {
        let mut postfix: Vec<char> = "71*52-/".chars().collect();
        let root = build_expression_tree(&mut postfix);
        // 전위 순회 결과를 수집해 트리 모양을 확인한다.
        fn pre(node: &Option<Box<ETNode>>, out: &mut Vec<char>) {
            if let Some(n) = node {
                out.push(n.data);
                pre(&n.left, out);
                pre(&n.right, out);
            }
        }
        let mut out = Vec::new();
        pre(&root, &mut out);
        assert_eq!(out, vec!['/', '*', '7', '1', '-', '5', '2']);
    }
}
