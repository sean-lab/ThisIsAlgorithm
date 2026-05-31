// 수식 트리 (Expression Tree)
// Clang/04/ExpressionTree 포팅. 후위표기식으로부터 트리를 구성하고 평가한다.

type ElementType = char;

struct ETNode {
    left: Option<Box<ETNode>>,
    right: Option<Box<ETNode>>,
    data: ElementType,
}

impl ETNode {
    fn new(data: ElementType) -> Box<ETNode> {
        Box::new(ETNode {
            left: None,
            right: None,
            data,
        })
    }
}

fn preorder_print_tree(node: &Option<Box<ETNode>>) {
    if let Some(n) = node {
        print!(" {}", n.data);
        preorder_print_tree(&n.left);
        preorder_print_tree(&n.right);
    }
}

fn inorder_print_tree(node: &Option<Box<ETNode>>) {
    if let Some(n) = node {
        print!("(");
        inorder_print_tree(&n.left);
        print!("{}", n.data);
        inorder_print_tree(&n.right);
        print!(")");
    }
}

fn postorder_print_tree(node: &Option<Box<ETNode>>) {
    if let Some(n) = node {
        postorder_print_tree(&n.left);
        postorder_print_tree(&n.right);
        print!(" {}", n.data);
    }
}

// C의 ET_BuildExpressionTree: 후위표기식의 끝에서부터 소비하며 트리를 구성한다.
fn build_expression_tree(postfix: &mut Vec<char>) -> Option<Box<ETNode>> {
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

fn evaluate(tree: &Option<Box<ETNode>>) -> f64 {
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

fn main() {
    let mut postfix: Vec<char> = "71*52-/".chars().collect();
    let root = build_expression_tree(&mut postfix);

    println!("Preorder ...");
    preorder_print_tree(&root);
    println!("\n");

    println!("Inorder ... ");
    inorder_print_tree(&root);
    println!("\n");

    println!("Postorder ... ");
    postorder_print_tree(&root);
    println!();

    println!("Evaulation Result : {:.6} ", evaluate(&root));
}
