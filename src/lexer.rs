use crate::token::Token;
use crate::constants::{ASSIGN, SEMICOLON, LPAREN, RPAREN, COMMA, PLUS, LBRACE, RBRACE, EOF, INT, ILLEGAL};
use std::borrow::Borrow;
use std::ops::Deref;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8, //TODO think about using Vec<u8> as byte. So, in that case we need no function new_token_string
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    /// read char
    ///
    ///
    ///
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    ///
    ///
    ///
    ///
    pub fn next_token(&mut self) -> Token {
        let mut token: Token = Token::new();

        self.skip_whitespace();

        match self.ch as char {
            '=' => {
                token = Token::new_token(ASSIGN.parse::<String>().unwrap(), self.ch);
            }
            ';' => {
                token = Token::new_token(SEMICOLON.parse::<String>().unwrap(), self.ch);
            }
            '(' => {
                token = Token::new_token(LPAREN.parse::<String>().unwrap(), self.ch);
            }
            ')' => {
                token = Token::new_token(RPAREN.parse::<String>().unwrap(), self.ch);
            }
            ',' => {
                token = Token::new_token(COMMA.parse::<String>().unwrap(), self.ch);
            }
            '+' => {
                token = Token::new_token(PLUS.parse::<String>().unwrap(), self.ch);
            }
            '{' => {
                token = Token::new_token(LBRACE.parse::<String>().unwrap(), self.ch);
            }
            '}' => {
                token = Token::new_token(RBRACE.parse::<String>().unwrap(), self.ch);
            }
            '0' => {
                token = Token::new_token(EOF.parse::<String>().unwrap(), 0);
            }
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let ident = self.read_identifier();
                    let tok_type = Token::lookup_ident(ident.clone());
                    token = Token::new_token_string(ident.clone(), tok_type.to_string().clone());
                    return token;
                } else if self.ch.is_ascii_digit() {
                    let ident = INT.parse::<String>().unwrap();
                    let tok_type = self.read_number();
                    token = Token::new_token_string(ident.clone(), tok_type.clone());
                    return token;
                } else {
                    token = Token::new_token(ILLEGAL.parse::<String>().unwrap(), self.ch)
                }
            }
        }

        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        loop {
            if self.ch.is_ascii_alphabetic() {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        loop {
            if self.ch.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        match self.ch as char {
            ' ' => {
                self.read_char();
            }
            '\t' => {
                self.read_char();
            }
            '\n' => {
                self.read_char();
            }
            '\r' => {
                self.read_char();
            }
            _ => {

            }
        }
    }
}

#[cfg(test)]
mod lexer_test {
    use crate::lexer::Lexer;
    

    #[test]
    fn test_next_token() {
        let input: String =
            "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);".to_string();

        let mut l = Lexer::new(input);
        loop {
            let tok = l.next_token();
            println!("{:?}", tok);
        }
    }
}