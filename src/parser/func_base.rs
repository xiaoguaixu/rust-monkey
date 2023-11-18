use std::rc::Rc;

use crate::{ast, token};
use crate::parser::base::{ParseContext, Precedence};

pub fn parse_expression(context: &mut Box<ParseContext>, precedence: i32) -> Option<Rc<dyn ast::Expression>> {
    let prefix = context.get_prefix_fn(context.cur_token.token_type.as_str());
    let mut left_exp = match prefix {
        None => {
            return None;
        }
        Some(v) => {
            v(context)
        }
    };

    while left_exp.is_some() && !context.peek_token_is(token::SEMICOLON) && precedence < context.peek_precedence() {
        match context.get_infix_fn(context.peek_token.token_type.as_str()) {
            None => {
                return left_exp;
            }
            Some(v) => {
                context.next_token();
                left_exp = v(context, left_exp.unwrap());
            }
        }
    }

    left_exp
}

pub fn parse_function_parameters(context: &mut Box<ParseContext>) -> Vec<ast::Identifier> {
    let mut identifiers = vec![];

    if context.peek_token_is(token::RPAREN) {
        context.next_token();
        return identifiers;
    }

    context.next_token();

    let ident = ast::Identifier {
        token: context.cur_token.clone(),
        value: context.cur_token.literal.clone(),
    };

    identifiers.push(ident);

    while context.peek_token_is(token::COMMA) {
        context.next_token();
        context.next_token();

        let ident = ast::Identifier {
            token: context.cur_token.clone(),
            value: context.cur_token.literal.clone(),
        };
        identifiers.push(ident);
    }

    if !context.expect_peek(token::RPAREN) {
        return identifiers;
    }

    identifiers
}


pub fn parse_expression_list(context: &mut Box<ParseContext>, end: &str) -> Vec<Rc<dyn ast::Expression>> {
    let mut expressions = vec![];

    if context.peek_token_is(end) {
        context.next_token();
        return expressions;
    }

    context.next_token();

    let expression = parse_expression(context, Precedence::LOWEST as i32);
    if expression.is_none() {
        return expressions;
    }

    expressions.push(expression.unwrap());

    while context.peek_token_is(token::COMMA) {
        context.next_token();
        context.next_token();

        let expression = parse_expression(context, Precedence::LOWEST as i32);
        if expression.is_some() {
            expressions.push(expression.unwrap());
        }
    }

    if !context.expect_peek(end) {
        return expressions;
    }

    expressions
}