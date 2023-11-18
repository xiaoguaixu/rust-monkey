use crate::{ast, lexer, token};
use crate::parser::base::ParseContext;
use crate::parser::func::parse_program;
use crate::parser::func_infix::*;
use crate::parser::func_prefix::*;

pub mod base;
mod func;
mod func_infix;
mod func_base;
mod func_prefix;
mod parse_test;


pub struct Parser {
    parse_context: Box<ParseContext>,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(l: Box<lexer::Lexer>) -> Self {
        let parse_context = Box::new(ParseContext::new(l));
        let mut parser = Self {
            parse_context,
        };

        parser.register_prefix();
        parser.register_infix();

        parser
    }

    pub fn parse_program(&mut self) -> Box<ast::Program> {
        parse_program(&mut self.parse_context)
    }

    pub fn errors(&self) -> Vec<String> {
        self.parse_context.errors.clone()
    }

    fn register_prefix(&mut self) {
        macro_rules! macro_register_prefix {
            ($key:ident, $func:ident) => {
                self.parse_context.register_prefix(token::$key, $func);
            }
        }

        macro_register_prefix!(IDENT, parse_identifier);
        macro_register_prefix!(INT, parse_integer_literal);
        macro_register_prefix!(STRING, parse_string_literal);
        macro_register_prefix!(BANG, parse_prefix_expression);
        macro_register_prefix!(MINUS, parse_prefix_expression);
        macro_register_prefix!(TRUE, parse_boolean);
        macro_register_prefix!(FALSE, parse_boolean);
        macro_register_prefix!(LPAREN, parse_grouped_expression);
        macro_register_prefix!(IF, parse_if_expression);
        macro_register_prefix!(FUNCTION, parse_function_literal);
        macro_register_prefix!(LBRACKET, parse_array_literal);
        macro_register_prefix!(LBRACE, parse_hash_literal);
    }

    fn register_infix(&mut self) {
        macro_rules! macro_register_infix {
            ($key:ident, $func:ident) => {
                self.parse_context.register_infix(token::$key, $func);
            }
        }

        macro_register_infix!(PLUS, parse_infix_expression);
        macro_register_infix!(MINUS, parse_infix_expression);
        macro_register_infix!(SLASH, parse_infix_expression);
        macro_register_infix!(ASTERISK, parse_infix_expression);
        macro_register_infix!(EQ, parse_infix_expression);
        macro_register_infix!(NOT_EQ, parse_infix_expression);
        macro_register_infix!(LT, parse_infix_expression);
        macro_register_infix!(GT, parse_infix_expression);

        macro_register_infix!(LPAREN, parse_call_expression);
        macro_register_infix!(LBRACKET, parse_index_expression);
    }
}