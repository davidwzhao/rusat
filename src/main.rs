mod ast_node;
use ast_node::*;

fn main() {
    let mut node = AstNode::new(AstNodeType::DisjunctionClause);
    let mut val = AstNode::new(AstNodeType::Value);
    val.add_child(AstNode::new(AstNodeType::Variable("hello".to_string())));
    node.add_child(val);

    let s = node.to_string();
    println!("{}", s);
}
