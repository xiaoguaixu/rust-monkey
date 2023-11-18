use std::any::Any;
use std::rc::Rc;

use crate::{macro_node_trait_impl, token};
use crate::ast::{Expression, Node, Statement};
use crate::ast::expression::Identifier;

macro_rules! macro_statement_trait_impl {
    ($impl_name:ident) => {
        impl Statement for $impl_name {
            fn statement_node(&self) {
            }

            fn upcast(&self) ->&dyn Node {
                self
            }
        }

        impl $impl_name {
            #[allow(dead_code)]
            pub fn from_statement<'a>(statement: &'a Rc<dyn Statement> ) -> Option<& 'a $impl_name> {
                match statement.as_any().downcast_ref::<$impl_name>() {
                    None => {None}
                    Some(v) => {Some(v)}
                }
            }

            pub fn from_node<'a>(node: &'a Rc<dyn Node>) -> Option<& 'a $impl_name> {
                match node.as_any().downcast_ref::<$impl_name>() {
                    None => {None}
                    Some(v) => {Some(v)}
                }
            }
        }
    }
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Rc<Identifier>,
    pub value: Option<Rc<dyn Expression>>,
}

impl Node for LetStatement {
    macro_node_trait_impl!(LetStatement);
    fn string(&self) -> String {
        let mut rlt = format!("{} {}", self.token.literal, self.name.string());
        match &self.value {
            None => {}
            Some(v) => {
                rlt = rlt + " = " + v.token_literal();
            }
        }
        rlt
    }
}

macro_statement_trait_impl!(LetStatement);

pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Option<Rc<dyn Expression>>,
}

impl Node for ReturnStatement {
    macro_node_trait_impl!(ReturnStatement);
    fn string(&self) -> String {
        match &self.return_value {
            None => {
                format!("{} ;", self.token.literal)
            }
            Some(v) => {
                format!("{} {} ", self.token.literal, v.token_literal())
            }
        }
    }
}

macro_statement_trait_impl!(ReturnStatement);

pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Rc<dyn Expression>,
}

impl Node for ExpressionStatement {
    macro_node_trait_impl!(ExpressionStatement);
    fn string(&self) -> String {
        self.expression.string()
    }
}

macro_statement_trait_impl!(ExpressionStatement);

pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Rc<dyn Statement>>,
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