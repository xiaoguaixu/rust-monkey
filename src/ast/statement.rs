use crate::{macro_node_trait_impl, token};
use crate::ast::{ASTNode, Node};

macro_rules! macro_statement_trait_impl {
    ($impl_name:ident) => {
        impl $impl_name {
        }
    }
}

#[derive(Clone)]
pub struct LetStatement {
    pub token: token::Token,
    pub name: Box<ASTNode>,
    pub value: Box<ASTNode>,
}

impl Node for LetStatement {
    macro_node_trait_impl!(LetStatement);
    fn string(&self) -> String {
        let mut rlt = format!("{} {}", self.token.literal, self.name.string());
        match &*self.value {
            ASTNode::Identifier(v) => {
                rlt = rlt + " = " + v.token_literal();
            }
            _ => {}
        }
        rlt
    }
}

macro_statement_trait_impl!(LetStatement);

#[derive(Clone)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Box<ASTNode>,
}

impl Node for ReturnStatement {
    macro_node_trait_impl!(ReturnStatement);
    fn string(&self) -> String {
        match *self.return_value {
            ASTNode::None => {
                format!("{} ;", self.token.literal)
            }
            _ => {
                format!("{} {} ", self.token.literal, self.token_literal())
            }
        }
    }
}

macro_statement_trait_impl!(ReturnStatement);

#[derive(Clone)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Box<ASTNode>,
}

impl Node for ExpressionStatement {
    macro_node_trait_impl!(ExpressionStatement);
    fn string(&self) -> String {
        self.expression.string()
    }
}

macro_statement_trait_impl!(ExpressionStatement);

#[derive(Clone)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<ASTNode>,
}

impl Node for BlockStatement {
    macro_node_trait_impl!(BlockStatement);
    fn string(&self) -> String {
        let mut rlt: String = "".to_string();
        for v in &self.statements {
            rlt = rlt + v.string().as_str();
        }

        rlt
    }
}

macro_statement_trait_impl!(BlockStatement);