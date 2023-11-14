use crate::{ast, token};
use crate::ast::ASTNode;
use crate::parser::base::{ParseContext, Precedence};
use crate::parser::func_base::{parse_expression, parse_expression_list};

pub fn parse_infix_expression(context: &mut Box<ParseContext>, left_expression: ASTNode) -> ASTNode {
    let left = left_expression;
    let precedence = context.cur_precedence();
    let operator = context.cur_token.literal.clone();
    let token = context.cur_token.clone();

    context.next_token();
    let right = parse_expression(context, precedence);
    if let ASTNode::None = right {
        return ASTNode::None;
    };

    ASTNode::InfixExpression(ast::InfixExpression {
        token,
        left: Box::new(left),
        operator,
        right: Box::new(right),
    })
}

pub fn parse_call_expression(context: &mut Box<ParseContext>, left_expression: ASTNode) -> ASTNode {
    let arguments = parse_expression_list(context, token::RPAREN);
    ASTNode::CallExpression(ast::CallExpression {
        token: context.cur_token.clone(),
        function: Box::new(left_expression),
        arguments,
    })
}

pub fn parse_index_expression(context: &mut Box<ParseContext>, left_expression: ASTNode) -> ASTNode {
    let token = context.cur_token.clone();
    context.next_token();

    let index = parse_expression(context, Precedence::LOWEST as i32);
    if let ASTNode::None = index {
        return ASTNode::None;
    };

    if !context.expect_peek(token::RBRACKET) {
        return ASTNode::None;
    }

    ASTNode::IndexExpression(ast::IndexExpression {
        token,
        left: Box::new(left_expression),
        index: Box::new(index),
    })
}

