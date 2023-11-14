use crate::{macro_node_trait_impl, token};
use crate::ast::ASTNode;

macro_rules! express_trait_impl {
    ($impl_name:ident) => {
    }
}


#[derive(Debug, Default, Clone)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Identifier {
    macro_node_trait_impl!(Identifier);
    pub fn string(&self) -> String {
        self.value.clone()
    }
}

express_trait_impl!(Identifier);


#[derive(Clone)]
pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

impl Boolean {
    macro_node_trait_impl!(Boolean);

    pub fn string(&self) -> String {
        self.token.literal.clone()
    }
}

express_trait_impl!(Boolean);

#[derive(Clone)]
pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl IntegerLiteral {
    macro_node_trait_impl!(IntegerLiteral);

    pub fn string(&self) -> String {
        self.token.literal.clone()
    }
}

express_trait_impl!(IntegerLiteral);

#[derive(Clone)]
pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Box<ASTNode>,
}

impl PrefixExpression {
    macro_node_trait_impl!(PrefixExpression);

    pub fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

express_trait_impl!(PrefixExpression);

#[derive(Clone)]
pub struct InfixExpression {
    pub token: token::Token,
    // The operator token, e.g. +
    pub left: Box<ASTNode>,
    pub operator: String,
    pub right: Box<ASTNode>,
}

impl InfixExpression {
    macro_node_trait_impl!(InfixExpression);

    pub fn string(&self) -> String {
        format!("({} {} {})", self.left.string(), self.operator, self.right.string())
    }
}

express_trait_impl!(InfixExpression);

#[derive(Clone)]
pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<ASTNode>,
    pub consequence: Box<ASTNode>,
    pub alternative: Box<ASTNode>,
}

impl IfExpression {
    macro_node_trait_impl!(IfExpression);

    pub fn string(&self) -> String {
        let mut rlt = format!("if {} {}", self.condition.string(), self.consequence.string());
        match *self.alternative {
            ASTNode::None => {}
            _ => {
                rlt = format!("{} else {}", rlt, self.alternative.string());
            }
        }
        rlt
    }
}

express_trait_impl!(IfExpression);

#[derive(Clone)]
pub struct FunctionLiteral {
    pub token: token::Token,
    pub parameters: Vec<ASTNode>,
    pub body: Box<ASTNode>,
}

impl FunctionLiteral {
    macro_node_trait_impl!(FunctionLiteral);

    pub fn string(&self) -> String {
        let mut params: Vec<String> = Vec::new();
        for v in &self.parameters {
            params.push(v.string());
        }
        format!("{}({}){}", self.token_literal(), params.join(","), self.body.string())
    }
}

express_trait_impl!(FunctionLiteral);

#[derive(Clone)]
pub struct CallExpression {
    pub token: token::Token,
    pub function: Box<ASTNode>,
    pub arguments: Vec<ASTNode>,
}


impl CallExpression {
    macro_node_trait_impl!(CallExpression);
    pub fn string(&self) -> String {
        let mut args: Vec<String> = Vec::new();
        for v in &self.arguments {
            args.push(v.string());
        }

        format!("{}({})", self.function.string(), args.join(", "))
    }
}

express_trait_impl!(CallExpression);

#[derive(Clone)]
pub struct StringLiteral {
    pub token: token::Token,
    pub value: String,
}

impl StringLiteral {
    macro_node_trait_impl!(StringLiteral);
    pub fn string(&self) -> String {
        self.token.literal.clone()
    }
}

express_trait_impl!(StringLiteral);

#[derive(Clone)]
pub struct ArrayLiteral {
    pub token: token::Token,
    pub elements: Vec<ASTNode>,
}

impl ArrayLiteral {
    macro_node_trait_impl!(ArrayLiteral);
    pub fn string(&self) -> String {
        let mut elements: Vec<String> = Vec::new();
        for v in &self.elements {
            elements.push(v.string());
        }
        format!("[{}]", elements.join(", "))
    }
}

express_trait_impl!(ArrayLiteral);

#[derive(Clone)]
pub struct IndexExpression {
    pub token: token::Token,
    pub left: Box<ASTNode>,
    pub index: Box<ASTNode>,
}

impl IndexExpression {
    macro_node_trait_impl!(IndexExpression);
    pub fn string(&self) -> String {
        format!("({}[{}])", self.left.string(), self.index.string())
    }
}

express_trait_impl!(IndexExpression);

#[derive(Clone)]
pub struct HashLiteral {
    pub token: token::Token,
    pub pairs: Vec<(Box<ASTNode>, Box<ASTNode>)>,
}

impl HashLiteral {
    macro_node_trait_impl!(HashLiteral);
    pub fn string(&self) -> String {
        let mut pairs: Vec<String> = Vec::new();
        for (key, value) in self.pairs.iter() {
            pairs.push(key.string() + ":" + value.string().as_str());
        }

        format!("{}{}{}", "{", pairs.join(","), "}")
    }
}

express_trait_impl!(HashLiteral);