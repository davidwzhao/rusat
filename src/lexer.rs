use std::fmt;

#[derive(Clone,Debug)]
pub enum Token {
    // (
    LParen,

    // )
    RParen,

    // &
    Conjunction,

    // |
    Disjunction,

    // ~
    Negation,

    // variable id
    Variable(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Conjunction => write!(f, "&"),
            Token::Disjunction => write!(f, "|"),
            Token::Negation => write!(f, "~"),
            Token::Variable(s) => write!(f, "{}", s),
        }
    }
}

pub fn lex(input: &String) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();

    // iterate over characters in input
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        use Token::*;
        match c {
            '(' => {
                result.push(LParen);
                it.next();
            }

            ')' => {
                result.push(RParen);
                it.next();
            }

            '&' => {
                result.push(Conjunction);
                it.next();
            }

            '|' => {
                result.push(Disjunction);
                it.next();
            }

            '~' => {
                result.push(Negation);
                it.next();
            }

            'a' ... 'z' | 'A' ... 'Z' => {
                let mut var = String::new();
                
                loop {
                    let var_char = it.peek();
                    if var_char == None || !var_char.unwrap().is_alphabetic() {
                        break;
                    }
                    var.push(*var_char.unwrap());
                    it.next();
                }

                result.push(Variable(var));
            }

            ' ' | '\n' => {
                it.next();
            }

            _ => return Err(format!("unexpected character {}", c))
        }
    }

    Ok(result)
}
