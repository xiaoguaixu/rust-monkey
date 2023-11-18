use std::fmt::Debug;
use std::any::Any;
use std::rc::Rc;

use crate::{macro_node_trait_impl, token};
use crate::ast::base::*;

macro_rules! express_trait_impl {
    ($impl_name:ident) => {
        impl Expression for $impl_name {
            fn expression_node(&self) {
            }

            fn upcast(&self) ->&dyn Node {
                self
            }
        }


        impl $impl_name {
            #[allow(dead_code)]
            pub fn from_expression<'a>(expression: &'a Rc<dyn Expression> ) -> Option<& 'a $impl_name> {
                match expression.as_any().downcast_ref::<$impl_name>() {
                    None => {None}
                    Some(v) => {Some(v)}
                }
            }
        }
    }
}


#[derive(Debug, Default, Clone)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Node for Identifier {
    macro_node_trait_impl!(Identifier);
    fn string(&self) -> String {
        self.value.clone()
    }
}

express_trait_impl!(Identifier);


pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

impl Node for Boolean {
    macro_node_trait_impl!(Boolean);

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

express_trait_impl!(Boolean);


pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    macro_node_trait_impl!(IntegerLiteral);

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

express_trait_impl!(IntegerLiteral);


pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Rc<dyn Expression>,
}

impl Node for PrefixExpression {
    macro_node_trait_impl!(PrefixExpression);

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

express_trait_impl!(PrefixExpression);

pub struct InfixExpression {
    pub token: token::Token,
    // The operator token, e.g. +
    pub left: Rc<dyn Expression>,
    pub operator: String,
    pub right: Rc<dyn Expression>,
}

impl Node for InfixExpression {
    macro_node_trait_impl!(InfixExpression);

    fn string(&self) -> String {
        format!("({} {} {})", self.left.string(), self.operator, self.right.string())
    }
}

express_trait_impl!(InfixExpression);


pub struct IfExpression {
    pub token: token::Token,
    pub condition: Rc<dyn Expression>,
    pub consequence: Rc<dyn Statement>,
    pub alternative: Option<Rc<dyn Statement>>,
}

impl Node for IfExpression {
    macro_node_trait_impl!(IfExpression);

    fn string(&self) -> String {
        let mut rlt = format!("if {} {}", self.condition.string(), self.consequence.string());
        match &self.alternative {
            None => {}
            Some(v) => {
                rlt = format!("{} else {}", rlt, v.string());
            }
        }
        rlt
    }
}

express_trait_impl!(IfExpression);

#[derive(Clone)]
pub struct FunctionLiteral {
    pub token: token::Token,
    pub parameters: Vec<Identifier>,
    pub body: Rc<dyn Statement>,
}

impl Node for FunctionLiteral {
    macro_node_trait_impl!(FunctionLiteral);

    fn string(&self) -> String {
        let mut params: Vec<String> = Vec::new();
        for v in &self.parameters {
            params.push(v.string());
        }
        format!("{}({}){}", self.token_literal(), params.join(","), self.body.string())
    }
}

express_trait_impl!(FunctionLiteral);

pub struct CallExpression {
    pub token: token::Token,
    pub function: Rc<dyn Expression>,
    pub arguments: Vec<Rc<dyn Expression>>,
}

impl Node for CallExpression {
    macro_node_trait_impl!(CallExpression);
    fn string(&self) -> String {
        let mut args: Vec<String> = Vec::new();
        for v in &self.arguments {
            args.push(v.string());
        }

        format!("{}({})", self.function.string(), args.join(", "))
    }
}

express_trait_impl!(CallExpression);

pub struct StringLiteral {
    pub token: token::Token,
    pub value: String,
}

impl Node for StringLiteral {
    macro_node_trait_impl!(StringLiteral);
    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

express_trait_impl!(StringLiteral);

pub struct ArrayLiteral {
    pub token: token::Token,
    pub elements: Vec<Rc<dyn Expression>>,
}

impl Node for ArrayLiteral {
    macro_node_trait_impl!(ArrayLiteral);
    fn string(&self) -> String {
        let mut elements: Vec<String> = Vec::new();
        for v in &self.elements {
            elements.push(v.string());
        }
        format!("[{}]", elements.join(", "))
    }
}

express_trait_impl!(ArrayLiteral);

pub struct IndexExpression {
    pub token: token::Token,
    pub left: Rc<dyn Expression>,
    pub index: Rc<dyn Expression>,
}

impl Node for IndexExpression {
    macro_node_trait_impl!(IndexExpression);
    fn string(&self) -> String {
        format!("({}[{}])", self.left.string(), self.index.string())
    }
}

express_trait_impl!(IndexExpression);

pub struct HashLiteral {
    pub token: token::Token,
    pub pairs: Vec<(Rc<dyn Expression>, Rc<dyn Expression>)>,
}

impl Node for HashLiteral {
    macro_node_trait_impl!(HashLiteral);
    fn string(&self) -> String {
        let mut pairs: Vec<String> = Vec::new();
        for (key, value) in self.pairs.iter() {
            pairs.push(key.string() + ":" + value.string().as_str());
        }

        format!("{}{}{}", "{", pairs.join(","), "}")
    }
}

express_trait_impl!(HashLiteral);