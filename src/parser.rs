/**
 * The grammar is as follows:
 * 
 * Conjunct -> ( '(' Disjunct ')' 'and' )* '(' Disjunct ')'
 * Disjunct -> ( Val 'or' )* Val
 * Val -> 'not' Var | Var
 * Var -> id
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
        Ok((tree, _pos)) => Ok(tree),
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
                // start a disjunction inside parentheses
                let (disjunction_node, next_pos) = parse_disjunction(tokens, position + 1)?;
                conjunction_node.add_child(disjunction_node);
                position = next_pos;
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

    Ok((conjunction_node, position + 1))
}

fn parse_disjunction(tokens: &Vec<Token>, pos: usize) -> Result<(AstNode, usize), String> {
    let mut disjunction_node = AstNode::new(AstNodeType::DisjunctionClause);
    let mut position = pos;

    while position < tokens.len() {
        match tokens.get(position) {
            Some(Token::Negation) | Some(Token::Variable(_)) => {
                let (value_node, next_pos) = parse_value(tokens, position)?;
                disjunction_node.add_child(value_node);
                position = next_pos + 1;
            }

            Some(Token::Disjunction) => {
                // continue to next
                position = position + 1;
            }

            _ => {
                return Ok((disjunction_node, position + 1));
            }
        }
    }

    Ok((disjunction_node, position + 1))
}

fn parse_value(tokens: &Vec<Token>, pos: usize) -> Result<(AstNode, usize), String> {
    let mut value_node = AstNode::new(AstNodeType::Value);
    let position;

    match tokens.get(pos) {
        Some(Token::Negation) => {
            let mut negation_node = AstNode::new(AstNodeType::Negation);

            // parse the contained variable
            let (variable_node, next_pos) = parse_variable(tokens, pos + 1)?;
            negation_node.add_child(variable_node);

            // add to value_node
            value_node.add_child(negation_node);
            position = next_pos;
        }

        Some(Token::Variable(_)) => {
            let (variable_node, next_pos) = parse_variable(tokens, pos)?;
            value_node.add_child(variable_node);
            position = next_pos;
        }

        _ => {
            return Err(format!("Unexpected token: {}, in position: {}", tokens.get(pos).unwrap(), pos));
        }
    }

    Ok((value_node, position))

    // Err(format!("Unexpected token: {}, in position: {}", tokens.get(pos).unwrap(), pos))
}

fn parse_variable(tokens: &Vec<Token>, pos: usize) -> Result<(AstNode, usize), String> {
    if pos >= tokens.len() {
        return Err("Error: reached end of input".to_string());
    }

    if let Some(Token::Variable(var)) = tokens.get(pos) {
        return Ok((AstNode::new(AstNodeType::Variable(var.to_string())), pos + 1));
    }

    Err(format!("Unexpected token: {}, in position: {}", tokens.get(pos).unwrap(), pos))
}
