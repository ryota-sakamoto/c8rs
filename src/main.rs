use std::{
    env::args,
    collections::VecDeque,
};

fn main() {
    let arg: Vec<_> = args().collect();
    if arg.len() < 2 {
        panic!("Invalid args len");
    }

    let mut tokens = tokenize(&arg[1]).into_iter();

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    if let Some(token) = tokens.next() {
        match token.token_type {
            TokenType::TK_NUMBER(n) => {
                println!("    mov rax, {}", n);
            },
            _ => {
                panic!("Invalid first number");
            }
        }
    }

    while let Some(token) = tokens.next() {
        use TokenType::*;

        match token.token_type {
            TK_PLUS | TK_MINUS => {
                let slice = tokens.as_slice();
                if let Some(t) = slice.first() {
                    if let TK_NUMBER(n) = t.token_type {
                        match token.token_type {
                            TK_PLUS => println!("    add rax, {}", n),
                            TK_MINUS => println!("    sub rax, {}", n),
                            _ => {}
                        }
                    } else {
                        panic!("Invalid + after")                        
                    }
                } else {
                    panic!("Invalid + after")
                }
            },
            _ => {}
        }
    }

    println!("    ret");
}

#[derive(Debug)]
enum TokenType {
    TK_EOF,
    TK_NUMBER(u8),
    TK_PLUS,
    TK_MINUS,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
}

fn tokenize(line: &str) -> Vec<Token> {
    use TokenType::*;

    let mut result = Vec::new();
    let mut iter = line.chars().peekable();
    loop {
        match iter.peek().cloned() {
            Some(c) if c.is_digit(10) => {
                let mut vec: Vec<char> = Vec::new();
                loop {
                    match iter.by_ref().peek().cloned() {
                        Some(c) if c.is_digit(10) => {
                            iter.next();
                            vec.push(c);
                        }
                        _ => break,
                    }
                }
                let num: String = vec.iter().collect();
                result.push(Token {
                    token_type: TK_NUMBER(num.parse().unwrap())
                });
            },
            Some('+') => {
                iter.next();
                result.push(Token {
                    token_type: TK_PLUS,
                });
            },
            Some('-') => {
                iter.next();
                result.push(Token {
                    token_type: TK_MINUS,
                });
            },
            _ => return result,
        }
    }
}
