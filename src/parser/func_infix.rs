use std::rc::Rc;

use crate::{ast, token};
use crate::parser::base::{ParseContext, Precedence};
use crate::parser::func_base::{parse_expression, parse_expression_list};

pub fn parse_infix_expression(context: &mut Box<ParseContext>, left_expression: Rc<dyn ast::Expression>) -> Option<Rc<dyn ast::Expression>> {
    let left = left_expression;
    let precedence = context.cur_precedence();
    let operator = context.cur_token.literal.clone();
    let token = context.cur_token.clone();

    context.next_token();
    let right = match parse_expression(context, precedence) {
        None => {
            return None;
        }
        Some(v) => {
            v
        }
    };

    Some(Rc::new(ast::InfixExpression {
        token,
        left,
        operator,
        right,
    }))
}

pub fn parse_call_expression(context: &mut Box<ParseContext>, left_expression: Rc<dyn ast::Expression>) -> Option<Rc<dyn ast::Expression>> {
    let arguments = parse_expression_list(context, token::RPAREN);
    Some(Rc::new(ast::CallExpression {
        token: context.cur_token.clone(),
        function: left_expression,
        arguments,
    }))
}

pub fn parse_index_expression(context: &mut Box<ParseContext>, left_expression: Rc<dyn ast::Expression>) -> Option<Rc<dyn ast::Expression>> {
    let token = context.cur_token.clone();
    context.next_token();

    let index = match parse_expression(context, Precedence::LOWEST as i32) {
        None => {
            return None;
        }
        Some(v) => {
            v
        }
    };

    if !context.expect_peek(token::RBRACKET) {
        return None;
    }

    Some(Rc::new(ast::IndexExpression {
        token,
        left: left_expression,
        index,
    }))
}

