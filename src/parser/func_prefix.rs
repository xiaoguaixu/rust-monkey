use std::rc::Rc;

use crate::{ast, token};
use crate::parser::base::{ParseContext, Precedence};
use crate::parser::func::parse_block_statement;
use crate::parser::func_base::{parse_expression, parse_expression_list, parse_function_parameters};

pub fn parse_identifier(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    let expression = ast::Identifier {
        token: context.cur_token.clone(),
        value: context.cur_token.literal.clone(),
    };

    Some(Rc::new(expression))
}

pub fn parse_integer_literal(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    let mut expression = ast::IntegerLiteral {
        token: context.cur_token.clone(),
        value: 0,
    };


    if let Ok(v) = context.cur_token.literal.parse() {
        expression.value = v;
    } else {
        let msg = format!("could not parse {} as integer", context.cur_token.literal);
        context.add_err_msg(msg.as_str());
        return None;
    }

    Some(Rc::new(expression))
}

pub fn parse_string_literal(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    let expression = ast::StringLiteral {
        token: context.cur_token.clone(),
        value: context.cur_token.literal.clone(),
    };

    Some(Rc::new(expression))
}

pub fn parse_prefix_expression(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    let token = context.cur_token.clone();
    let operator = context.cur_token.literal.clone();
    context.next_token();
    let right = match parse_expression(context, Precedence::PREFIX as i32) {
        None => {
            return None;
        }
        Some(v) => {
            v
        }
    };

    let expression = ast::PrefixExpression {
        token,
        operator,
        right,
    };

    Some(Rc::new(expression))
}

pub fn parse_boolean(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    let expression = ast::Boolean {
        token: context.cur_token.clone(),
        value: context.cur_token_is(token::TRUE),
    };

    Some(Rc::new(expression))
}

pub fn parse_grouped_expression(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    context.next_token();
    let expression = parse_expression(context, Precedence::LOWEST as i32);
    if !context.expect_peek(token::RPAREN) {
        return None;
    }

    expression
}

pub fn parse_if_expression(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    if !context.expect_peek(token::LPAREN) {
        return None;
    }

    let tok = context.cur_token.clone();
    context.next_token();

    let condition = match parse_expression(context, Precedence::LOWEST as i32) {
        None => {
            return None;
        }
        Some(v) => {
            v
        }
    };

    if !context.expect_peek(token::RPAREN) {
        return None;
    }

    if !context.expect_peek(token::LBRACE) {
        return None;
    }

    let consequence = match parse_block_statement(context) {
        None => {
            return None;
        }
        Some(v) => {
            v
        }
    };

    let mut expression = ast::IfExpression {
        token: tok,
        condition,
        consequence,
        alternative: None,
    };

    if context.peek_token_is(token::ELSE) {
        context.next_token();
        if !context.expect_peek(token::LBRACE) {
            return None;
        }

        expression.alternative = parse_block_statement(context);
    }

    Some(Rc::new(expression))
}

pub fn parse_function_literal(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    let token = context.cur_token.clone();
    if !context.expect_peek(token::LPAREN) {
        return None;
    }

    let parameters = parse_function_parameters(context);
    if !context.expect_peek(token::LBRACE) {
        return None;
    }
    let body = match parse_block_statement(context) {
        None => {
            return None;
        }
        Some(v) => { v }
    };

    let expression = ast::FunctionLiteral {
        token,
        parameters,
        body,
    };

    Some(Rc::new(expression))
}

pub fn parse_array_literal(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    let mut expression = ast::ArrayLiteral {
        token: context.cur_token.clone(),
        elements: vec![],
    };

    expression.elements = parse_expression_list(context, token::RBRACKET);

    Some(Rc::new(expression))
}

pub fn parse_hash_literal(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Expression>> {
    let mut expression = ast::HashLiteral {
        token: context.cur_token.clone(),
        pairs: vec![],
    };

    while !context.expect_peek(token::RBRACE) {
        context.next_token();
        let key = parse_expression(context, Precedence::LOWEST as i32);
        if key.is_none() {
            return None;
        }

        let key = key.unwrap();

        if !context.expect_peek(token::COLON) {
            return None;
        }

        context.next_token();
        let value = parse_expression(context, Precedence::LOWEST as i32);
        if value.is_none() {
            return None;
        }

        let value = value.unwrap();
        expression.pairs.push((key, value));

        if !context.peek_token_is(token::RBRACE) && !context.expect_peek(token::COMMA) {
            return None;
        }
    }

    Some(Rc::new(expression))
}