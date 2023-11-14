use crate::token;

mod lexer_test;

#[derive(Debug, Default, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: &String) -> Self {
        let mut l = Self {
            input: input.clone(),
            ..Default::default()
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> token::Token {
        let tok: token::Token;

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    tok = token::Token {
                        token_type: token::EQ.to_string(),
                        literal,
                    };
                } else {
                    tok = Lexer::new_token(&token::ASSIGN.to_string(), self.ch);
                }
            }
            '+' => {
                tok = Lexer::new_token(&token::PLUS.to_string(), self.ch);
            }
            '-' => {
                tok = Lexer::new_token(&token::MINUS.to_string(), self.ch);
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    tok = token::Token {
                        token_type: token::NOT_EQ.to_string(),
                        literal,
                    };
                } else {
                    tok = Lexer::new_token(&token::BANG.to_string(), self.ch);
                }
            }
            '/' => {
                tok = Lexer::new_token(&token::SLASH.to_string(), self.ch);
            }
            '*' => {
                tok = Lexer::new_token(&token::ASTERISK.to_string(), self.ch);
            }
            '<' => {
                tok = Lexer::new_token(&token::LT.to_string(), self.ch);
            }
            '>' => {
                tok = Lexer::new_token(&token::GT.to_string(), self.ch);
            }
            ';' => {
                tok = Lexer::new_token(&token::SEMICOLON.to_string(), self.ch);
            }
            ':' => {
                tok = Lexer::new_token(&token::COLON.to_string(), self.ch);
            }
            ',' => {
                tok = Lexer::new_token(&token::COMMA.to_string(), self.ch);
            }
            '{' => {
                tok = Lexer::new_token(&token::LBRACE.to_string(), self.ch);
            }
            '}' => {
                tok = Lexer::new_token(&token::RBRACE.to_string(), self.ch);
            }
            '(' => {
                tok = Lexer::new_token(&token::LPAREN.to_string(), self.ch);
            }
            ')' => {
                tok = Lexer::new_token(&token::RPAREN.to_string(), self.ch);
            }
            '"' => {
                tok = Lexer::new_token_ex(&token::STRING.to_string(), &self.read_string());
            }
            '[' => {
                tok = Lexer::new_token(&token::LBRACKET.to_string(), self.ch);
            }
            ']' => {
                tok = Lexer::new_token(&token::RBRACKET.to_string(), self.ch);
            }
            '\0' => {
                tok = Lexer::new_token(&token::EOF.to_string(), ' ');
            }
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    tok = Lexer::new_token_ex(&token::lookup_ident(&literal), &literal);
                    return tok;
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    tok = Lexer::new_token_ex(&token::INT.to_string(), &literal);
                    return tok;
                } else {
                    tok = Lexer::new_token(&token::ILLEGAL.to_string(), self.ch);
                }
            }
        }

        self.read_char();

        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    fn peek_char(&self) -> char {
        return if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        };
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char()
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char()
        }
        self.input[position..self.position].to_string()
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
        }
        self.input[position..self.position].to_string()
    }


    fn new_token(token_type: &token::TokenType, ch: char) -> token::Token {
        return token::Token {
            token_type: token_type.clone(),
            literal: ch.to_string(),
        };
    }

    fn new_token_ex(token_type: &token::TokenType, literal: &String) -> token::Token {
        return token::Token {
            token_type: token_type.clone(),
            literal: literal.clone(),
        };
    }

    fn is_letter(ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
    }

    fn is_digit(ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }
}



