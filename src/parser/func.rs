use std::rc::Rc;

use crate::{ast, token};
use crate::parser::base::{ParseContext, Precedence};
use crate::parser::func_base::parse_expression;

#[allow(dead_code)]
pub fn parse_program(context: &mut Box<ParseContext>) -> Box<ast::Program> {
    let mut program = Box::new(ast::Program {
        statements: vec![],
    });

    while !context.cur_token_is(token::EOF) {
        let statement = parse_statement(context);
        if statement.is_some() {
            program.statements.push(statement.unwrap());
        }
        context.next_token();
    }
    program
}

pub fn parse_statement(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Statement>> {
    match context.cur_token.token_type.as_str() {
        token::LET => {
            parse_let_statement(context)
        }
        token::RETURN => {
            parse_return_statement(context)
        }
        _ => {
            parse_expression_statement(context)
        }
    }
}

pub fn parse_let_statement(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Statement>> {
    let mut statement = ast::LetStatement {
        token: context.cur_token.clone(),
        name: Rc::new(Default::default()),
        value: None,
    };

    if !context.expect_peek(token::IDENT) {
        return None;
    }

    statement.name = Rc::new(ast::Identifier {
        token: context.cur_token.clone(),
        value: context.cur_token.literal.clone(),
    });

    if !context.expect_peek(token::ASSIGN) {
        return None;
    }

    context.next_token();

    statement.value = parse_expression(context, Precedence::LOWEST as i32);

    if context.peek_token_is(token::SEMICOLON) {
        context.next_token();
    }

    Some(Rc::new(statement))
}

pub fn parse_return_statement(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Statement>> {
    let mut statement = ast::ReturnStatement {
        token: context.cur_token.clone(),
        return_value: None,
    };

    context.next_token();

    statement.return_value = parse_expression(context, Precedence::LOWEST as i32);
    if context.peek_token_is(token::SEMICOLON) {
        context.next_token();
    }

    Some(Rc::new(statement))
}

pub fn parse_expression_statement(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Statement>> {
    let token = context.cur_token.clone();
    let expression = match parse_expression(context, Precedence::LOWEST as i32) {
        None => {
            return None;
        }
        Some(v) => { v }
    };

    let statement = ast::ExpressionStatement {
        token,
        expression,
    };

    if context.peek_token_is(token::SEMICOLON) {
        context.next_token();
    }

    Some(Rc::new(statement))
}

pub fn parse_block_statement(context: &mut Box<ParseContext>) -> Option<Rc<dyn ast::Statement>> {
    let mut statement = ast::BlockStatement {
        token: context.cur_token.clone(),
        statements: vec![],
    };

    context.next_token();

    while !context.cur_token_is(token::RBRACE) && !context.cur_token_is(token::EOF) {
        match parse_statement(context) {
            None => {}
            Some(v) => {
                statement.statements.push(v);
            }
        }
        context.next_token();
    }

    Some(Rc::new(statement))
}

