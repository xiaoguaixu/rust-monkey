use std::any::Any;
use std::rc::Rc;

use crate::ast::base::*;

pub struct Program {
    pub statements: Vec<Rc<dyn Statement>>,
}

macro_rules! macro_program_trait_impl {
    ($impl_name:ident) => {

        impl $impl_name {
            #[allow(dead_code)]
            fn upcast(&self) ->&dyn Node {
                self
            }
        }
    }
}

macro_program_trait_impl!(Program);

impl Node for Program {
    fn token_literal(&self) -> &str {
        self.statements[0].token_literal()
    }

    fn string(&self) -> String {
        let mut rlt = "".to_string();
        for v in &self.statements {
            rlt = rlt + v.string().as_str();
        }
        rlt
    }

    fn as_any(&self) -> &dyn Any
        where Self: Sized
    {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any
        where Self: Sized
    {
        self
    }
}