/**
 * The grammar is as follows:
 * 
 * Conjunct -> ( '(' Disjunct ')' 'and' )* '(' Disjunct ')'
 * Disjunct -> ( Val 'or' )* Val
 * Val -> 'not' id | id
 *
 * Disjunct -> Conjunct 'or' Disjunct | Conjunct
 * Conjunct -> Val 'and' Conjunct | Val
 * Val -> Neg Val' | Val' | ( Disjunct )
 * Neg -> 'not'
 * Val' -> id
 */

use crate::ast_node::{AstNode,AstNodeType};
use crate::lexer::*;

pub fn parse(tokens: &Vec<Token>) -> Result<AstNode, String> {
    // let tokens = lex(input)?;

    // The grammar start symbol represents a conjunction clause
    let result = parse_conjunction(&tokens, 0);
    match result {
        Ok((tree, pos)) => Ok(tree),
        Err(e) => Err(e)
    }
}

fn parse_conjunction(tokens: &Vec<Token>, pos: usize) -> Result<(AstNode, usize), String> {
    let mut conjunction_node = AstNode::new(AstNodeType::ConjunctionClause);
    let mut position = pos;

    // let (disjunction_node, mut next_pos) = parse_disjunction(tokens, position)?;

    // add the first conjunction node
    // disjunction_node.add_child(conjunction_node);

    while position < tokens.len() {
        match tokens.get(position) {
            Some(Token::LParen) => {
                let (disjunction_node, next_pos) = parse_disjunction(tokens, position + 1)?;
                conjunction_node.add_child(disjunction_node);
                position = next_pos;


                // position += 1;
                // println!("next_pos before: {}", next_pos);
                // let (conjunction_node_after, next_pos_after) = parse_conjunction(tokens, next_pos)?;
                // next_pos = next_pos_after;
                // println!("{}", next_pos);
            }

            Some(Token::RParen) => {
                // this is a little bit cheaty - this should be a new rule in the grammar
                // but doing it this way is more succinct and it still works
                if position == tokens.len() - 1 {
                    // we have reached the end
                    position = position + 1;
                    break;
                } else {
                    // check that the next token is a conjunction
                    if let Some(Token::Conjunction) = tokens.get(position + 1) {
                        position = position + 2;
                    } else {
                        return Err(format!("Unexpected token: {}, in position: {}", tokens.get(position + 1).unwrap(), position + 1))
                    }
                }
            }

            _ => {
                return Err(format!("Unexpected token: {}, in position: {}", tokens.get(position).unwrap(), position))
                // return Err("Unexpected token: ".to_string())
            }
        }
    }

    Ok((conjunction_node, position))
}

fn parse_disjunction(tokens: &Vec<Token>, position: usize) -> Result<(AstNode, usize), String> {
    println!("position: {}", position);
    Ok((AstNode::new(AstNodeType::DisjunctionClause), position + 1))
}
