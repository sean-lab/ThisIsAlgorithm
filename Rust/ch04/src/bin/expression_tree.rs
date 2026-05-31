// 수식 트리 (Expression Tree)
// Clang/04/ExpressionTree 포팅. 로직은 ch04::expression_tree 모듈에 있다.
use ch04::expression_tree::{
    build_expression_tree, evaluate, inorder_print_tree, postorder_print_tree, preorder_print_tree,
};

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
