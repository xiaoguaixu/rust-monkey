use crate::{ast, token};
use crate::ast::ASTNode;
use crate::parser::base::{ParseContext, Precedence};
use crate::parser::func::parse_block_statement;
use crate::parser::func_base::{parse_expression, parse_expression_list, parse_function_parameters};

pub fn parse_identifier(context: &mut Box<ParseContext>) -> ASTNode {
    let expression = ast::Identifier {
        token: context.cur_token.clone(),
        value: context.cur_token.literal.clone(),
    };

    ASTNode::Identifier(expression)
}

pub fn parse_integer_literal(context: &mut Box<ParseContext>) -> ASTNode {
    let mut expression = ast::IntegerLiteral {
        token: context.cur_token.clone(),
        value: 0,
    };


    if let Ok(v) = context.cur_token.literal.parse() {
        expression.value = v;
    } else {
        let msg = format!("could not parse {} as integer", context.cur_token.literal);
        context.add_err_msg(msg.as_str());
        return ASTNode::None;
    }

    ASTNode::IntegerLiteral(expression)
}

pub fn parse_string_literal(context: &mut Box<ParseContext>) -> ASTNode {
    let expression = ast::StringLiteral {
        token: context.cur_token.clone(),
        value: context.cur_token.literal.clone(),
    };

    ASTNode::StringLiteral(expression)
}

pub fn parse_prefix_expression(context: &mut Box<ParseContext>) -> ASTNode {
    let token = context.cur_token.clone();
    let operator = context.cur_token.literal.clone();
    context.next_token();
    let right = parse_expression(context, Precedence::PREFIX as i32);
    if right.is_none() {
        return ASTNode::None;
    };

    let expression = ast::PrefixExpression {
        token,
        operator,
        right: Box::new(right),
    };

    ASTNode::PrefixExpression(expression)
}

pub fn parse_boolean(context: &mut Box<ParseContext>) -> ASTNode {
    let expression = ast::Boolean {
        token: context.cur_token.clone(),
        value: context.cur_token_is(token::TRUE),
    };

    ASTNode::Boolean(expression)
}

pub fn parse_grouped_expression(context: &mut Box<ParseContext>) -> ASTNode {
    context.next_token();
    let expression = parse_expression(context, Precedence::LOWEST as i32);
    if !context.expect_peek(token::RPAREN) {
        return ASTNode::None;
    }

    expression
}

pub fn parse_if_expression(context: &mut Box<ParseContext>) -> ASTNode {
    if !context.expect_peek(token::LPAREN) {
        return ASTNode::None;
    }

    let tok = context.cur_token.clone();
    context.next_token();

    let condition = parse_expression(context, Precedence::LOWEST as i32);
    if condition.is_none() {
        return ASTNode::None;
    };

    if !context.expect_peek(token::RPAREN) {
        return ASTNode::None;
    }

    if !context.expect_peek(token::LBRACE) {
        return ASTNode::None;
    }

    let consequence = parse_block_statement(context);
    if consequence.is_none() {
        return ASTNode::None;
    };

    let mut expression = ast::IfExpression {
        token: tok,
        condition: Box::new(condition),
        consequence: Box::new(consequence),
        alternative: Box::new(ASTNode::None),
    };

    if context.peek_token_is(token::ELSE) {
        context.next_token();
        if !context.expect_peek(token::LBRACE) {
            return ASTNode::None;
        }

        expression.alternative = Box::new(parse_block_statement(context));
    }

    ASTNode::IfExpression(expression)
}

pub fn parse_function_literal(context: &mut Box<ParseContext>) -> ASTNode {
    let token = context.cur_token.clone();
    if !context.expect_peek(token::LPAREN) {
        return ASTNode::None;
    }

    let parameters = parse_function_parameters(context);
    if !context.expect_peek(token::LBRACE) {
        return ASTNode::None;
    }
    let body = parse_block_statement(context);
    if body.is_none() {
        return ASTNode::None;
    };

    let expression = ast::FunctionLiteral {
        token,
        parameters,
        body: Box::new(body),
    };

    ASTNode::FunctionLiteral(expression)
}

pub fn parse_array_literal(context: &mut Box<ParseContext>) -> ASTNode {
    let mut expression = ast::ArrayLiteral {
        token: context.cur_token.clone(),
        elements: vec![],
    };

    expression.elements = parse_expression_list(context, token::RBRACKET);

    ASTNode::ArrayLiteral(expression)
}

pub fn parse_hash_literal(context: &mut Box<ParseContext>) -> ASTNode {
    let mut expression = ast::HashLiteral {
        token: context.cur_token.clone(),
        pairs: vec![],
    };

    while !context.peek_token_is(token::RBRACE) {
        context.next_token();
        let key = parse_expression(context, Precedence::LOWEST as i32);
        if key.is_none() {
            return ASTNode::None;
        }

        if !context.expect_peek(token::COLON) {
            return ASTNode::None;
        }

        context.next_token();
        let value = parse_expression(context, Precedence::LOWEST as i32);
        if value.is_none() {
            return ASTNode::None;
        }

        expression.pairs.push((Box::new(key), Box::new(value)));

        if !context.peek_token_is(token::RBRACE) && !context.expect_peek(token::COMMA) {
            return ASTNode::None;
        }
    }
    context.next_token();

    ASTNode::HashLiteral(expression)
}