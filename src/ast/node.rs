use paste::paste;

use crate::ast::{ArrayLiteral, BlockStatement, Boolean, CallExpression, ExpressionStatement, FunctionLiteral, HashLiteral, Identifier, IfExpression, IndexExpression, InfixExpression, IntegerLiteral, LetStatement, Node, PrefixExpression, Program, ReturnStatement, StringLiteral};

#[derive(Clone)]
pub enum ASTNode {
    None,
    Identifier(Identifier),
    Boolean(Boolean),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    IndexExpression(IndexExpression),
    HashLiteral(HashLiteral),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
    BlockStatement(BlockStatement),
    Program(Program),
}


macro_rules! macro_ast_node_some_fun {
    ($func_name:ident, $node_name:ident) => {
        paste! {
            pub fn [<is_ $func_name>] (&self) -> bool {
                return match self {
                    ASTNode::$node_name => { true }
                    _ => { false }
                }
            }
        }
    }
}


impl ASTNode {
    macro_ast_node_some_fun!(none, None);
    pub fn string(&self) -> String {
        match self {
            ASTNode::None => { "".to_string() }
            ASTNode::Identifier(v) => { v.string() }
            ASTNode::Boolean(v) => { v.string() }
            ASTNode::IntegerLiteral(v) => { v.string() }
            ASTNode::PrefixExpression(v) => { v.string() }
            ASTNode::InfixExpression(v) => { v.string() }
            ASTNode::IfExpression(v) => { v.string() }
            ASTNode::FunctionLiteral(v) => { v.string() }
            ASTNode::CallExpression(v) => { v.string() }
            ASTNode::StringLiteral(v) => { v.string() }
            ASTNode::ArrayLiteral(v) => { v.string() }
            ASTNode::IndexExpression(v) => { v.string() }
            ASTNode::HashLiteral(v) => { v.string() }
            ASTNode::LetStatement(v) => { v.string() }
            ASTNode::ReturnStatement(v) => { v.string() }
            ASTNode::ExpressionStatement(v) => { v.string() }
            ASTNode::BlockStatement(v) => { v.string() }
            ASTNode::Program(v) => { v.string() }
        }
    }

    pub fn token_literal(&self) -> &str {
        match self {
            ASTNode::None => { "" }
            ASTNode::Identifier(v) => { v.token_literal() }
            ASTNode::Boolean(v) => { v.token_literal() }
            ASTNode::IntegerLiteral(v) => { v.token_literal() }
            ASTNode::PrefixExpression(v) => { v.token_literal() }
            ASTNode::InfixExpression(v) => { v.token_literal() }
            ASTNode::IfExpression(v) => { v.token_literal() }
            ASTNode::FunctionLiteral(v) => { v.token_literal() }
            ASTNode::CallExpression(v) => { v.token_literal() }
            ASTNode::StringLiteral(v) => { v.token_literal() }
            ASTNode::ArrayLiteral(v) => { v.token_literal() }
            ASTNode::IndexExpression(v) => { v.token_literal() }
            ASTNode::HashLiteral(v) => { v.token_literal() }
            ASTNode::LetStatement(v) => { v.token_literal() }
            ASTNode::ReturnStatement(v) => { v.token_literal() }
            ASTNode::ExpressionStatement(v) => { v.token_literal() }
            ASTNode::BlockStatement(v) => { v.token_literal() }
            ASTNode::Program(v) => { v.token_literal() }
        }
    }
}
