use crate::ast::ASTNode;

#[derive(Clone)]
pub struct Program {
    pub statements: Vec<ASTNode>,
}

macro_rules! macro_program_trait_impl {
    ($impl_name:ident) => {

        impl $impl_name {
        }
    }
}

macro_program_trait_impl!(Program);

impl Program {
    #[allow(dead_code)]
    pub fn token_literal(&self) -> &str {
        self.statements[0].token_literal()
    }

    pub fn string(&self) -> String {
        let mut rlt = "".to_string();
        for v in &self.statements {
            rlt = rlt + v.string().as_str();
        }
        rlt
    }
}