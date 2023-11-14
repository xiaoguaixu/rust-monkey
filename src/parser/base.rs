use std::collections::HashMap;
use std::rc::Rc;

use phf::phf_map;

use crate::{lexer, token};
use crate::ast::ASTNode;


#[repr(i32)]
pub enum Precedence {
    LOWEST = 1,
    EQUALS,
    // ==
    LESSGREATER,
    // > or <
    SUM,
    // +
    PRODUCT,
    // *
    PREFIX,
    // -X or !X
    #[allow(dead_code)]
    CALL,
    // myFunction(X)
    INDEX,       // array[index]
}

#[allow(dead_code)]
static PRECEDENCES: phf::Map<&str, i32> = phf_map! {
    "==" =>       Precedence::EQUALS as i32,
	"!=" =>   Precedence::EQUALS as i32,
	"<" =>       Precedence::LESSGREATER as i32,
	">" =>       Precedence::LESSGREATER as i32,
	"+" =>     Precedence::SUM as i32,
	"-" =>    Precedence::SUM as i32,
	"/" =>    Precedence::PRODUCT as i32,
	"*" => Precedence::PRODUCT as i32,
	"(" =>   Precedence::PRODUCT as i32,
	"[" => Precedence::INDEX as i32,
};


pub type PrefixParseFn = dyn Fn(&mut Box<ParseContext>) -> ASTNode;
pub type InfixParseFn = dyn Fn(&mut Box<ParseContext>, ASTNode) -> ASTNode;

pub struct ParseContext {
    pub l: Box<lexer::Lexer>,
    pub errors: Vec<String>,
    pub cur_token: token::Token,
    pub peek_token: token::Token,
    pub prefix_parse_fns: HashMap<token::TokenType, Rc<PrefixParseFn>>,
    pub infix_parse_fns: HashMap<token::TokenType, Rc<InfixParseFn>>,
}

impl ParseContext {
    pub fn new(l: Box<lexer::Lexer>) -> Self {
        let mut rlt = Self {
            l,
            errors: vec![],
            cur_token: Default::default(),
            peek_token: Default::default(),
            prefix_parse_fns: Default::default(),
            infix_parse_fns: Default::default(),
        };
        rlt.next_token();
        rlt.next_token();
        rlt
    }

    pub fn register_prefix<F>(&mut self, key: &str, func: F)
        where
            F: Fn(&mut Box<ParseContext>) -> ASTNode + 'static,
    {
        self.prefix_parse_fns.insert(key.to_string(), Rc::new(func));
    }

    pub fn register_infix<F>(&mut self, key: &str, func: F)
        where
            F: Fn(&mut Box<ParseContext>, ASTNode) -> ASTNode + 'static,
    {
        self.infix_parse_fns.insert(key.to_string(), Rc::new(func));
    }

    pub fn get_prefix_fn(&self, key: &str) -> Option<Rc<PrefixParseFn>> {
        match self.prefix_parse_fns.get(key) {
            None => { None }
            Some(v) => { Some(v.clone()) }
        }
    }

    pub fn get_infix_fn(&self, key: &str) -> Option<Rc<InfixParseFn>> {
        match self.infix_parse_fns.get(key) {
            None => { None }
            Some(v) => { Some(v.clone()) }
        }
    }
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn cur_token_is(&self, t: &str) -> bool {
        self.cur_token.token_type.as_str() == t
    }

    pub fn peek_token_is(&self, t: &str) -> bool {
        self.peek_token.token_type == t
    }

    pub fn peek_precedence(&self) -> i32 {
        match PRECEDENCES.get(self.peek_token.token_type.as_str()) {
            None => {
                Precedence::LOWEST as i32
            }
            Some(v) => {
                *v
            }
        }
    }

    pub fn cur_precedence(&self) -> i32 {
        match PRECEDENCES.get(self.cur_token.token_type.as_str()) {
            None => {
                Precedence::LOWEST as i32
            }
            Some(v) => {
                *v
            }
        }
    }

    pub fn expect_peek(&mut self, t: &str) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(&t);
            false
        }
    }

    pub fn peek_error(&mut self, t: &str) {
        let msg = format!("expected next token to be {}, got {} instead",
                          t, self.peek_token.token_type);

        self.errors.push(msg);
    }

    pub fn add_err_msg(&mut self, msg: &str) {
        self.errors.push(msg.to_string());
    }

    #[allow(dead_code)]
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}




