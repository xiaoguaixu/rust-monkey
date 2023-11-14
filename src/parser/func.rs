use crate::{ast, token};
use crate::ast::ASTNode;
use crate::parser::base::{ParseContext, Precedence};
use crate::parser::func_base::parse_expression;

#[allow(dead_code)]
pub fn parse_program(context: &mut Box<ParseContext>) -> ASTNode {
    let mut program = ast::Program {
        statements: vec![],
    };

    while !context.cur_token_is(token::EOF) {
        let statement = parse_statement(context);
        match statement {
            ASTNode::None => {}
            _ => { program.statements.push(statement); }
        }
        context.next_token();
    }
    ASTNode::Program(program)
}

pub fn parse_statement(context: &mut Box<ParseContext>) -> ASTNode {
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

pub fn parse_let_statement(context: &mut Box<ParseContext>) -> ASTNode {
    let mut statement = ast::LetStatement {
        token: context.cur_token.clone(),
        name: Box::new(ASTNode::None),
        value: Box::new(ASTNode::None),
    };

    if !context.expect_peek(token::IDENT) {
        return ASTNode::None;
    }

    statement.name = Box::new(ASTNode::Identifier(ast::Identifier {
        token: context.cur_token.clone(),
        value: context.cur_token.literal.clone(),
    }));

    if !context.expect_peek(token::ASSIGN) {
        return ASTNode::None;
    }

    context.next_token();

    statement.value = Box::new(parse_expression(context, Precedence::LOWEST as i32));

    if context.peek_token_is(token::SEMICOLON) {
        context.next_token();
    }

    ASTNode::LetStatement(statement)
}

pub fn parse_return_statement(context: &mut Box<ParseContext>) -> ASTNode {
    let mut statement = ast::ReturnStatement {
        token: context.cur_token.clone(),
        return_value: Box::new(ASTNode::None),
    };

    context.next_token();

    statement.return_value = Box::new(parse_expression(context, Precedence::LOWEST as i32));
    if context.peek_token_is(token::SEMICOLON) {
        context.next_token();
    }

    ASTNode::ReturnStatement(statement)
}

pub fn parse_expression_statement(context: &mut Box<ParseContext>) -> ASTNode {
    let token = context.cur_token.clone();
    let expression = parse_expression(context, Precedence::LOWEST as i32);

    match expression {
        ASTNode::None => { return ASTNode::None; }
        _ => {}
    }

    let statement = ast::ExpressionStatement {
        token,
        expression: Box::new(expression),
    };

    if context.peek_token_is(token::SEMICOLON) {
        context.next_token();
    }

    ASTNode::ExpressionStatement(statement)
}

pub fn parse_block_statement(context: &mut Box<ParseContext>) -> ASTNode {
    let mut statement = ast::BlockStatement {
        token: context.cur_token.clone(),
        statements: vec![],
    };

    context.next_token();

    while !context.cur_token_is(token::RBRACE) && !context.cur_token_is(token::EOF) {
        let sub_statement = parse_statement(context);
        match sub_statement {
            ASTNode::None => {}
            _ => {
                statement.statements.push(sub_statement);
            }
        }
        context.next_token();
    }

    ASTNode::BlockStatement(statement)
}

