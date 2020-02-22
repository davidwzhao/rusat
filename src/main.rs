mod ast_node;
use ast_node::*;

mod lexer;
use lexer::*;

fn main() {
    let mut node = AstNode::new(AstNodeType::DisjunctionClause);
    let mut val = AstNode::new(AstNodeType::Value);
    val.add_child(AstNode::new(AstNodeType::Variable("hello".to_string())));
    node.add_child(val);

    let s = node.to_string();
    println!("{}", s);

    let l = lex(&"(asdf& ~b)".to_string());
    if l.is_ok() {
        println!("{:?}", l.unwrap());
    }
}
