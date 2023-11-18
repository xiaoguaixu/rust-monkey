use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> &str;

    fn string(&self) -> String;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn upcast(&self) -> &dyn Node;
}

pub trait Expression: Node {
    fn expression_node(&self);
    fn upcast(&self) -> &dyn Node;
}

pub trait Upcast {
    fn upcast(&self) -> &dyn Node;
}

#[macro_export]
macro_rules! macro_node_trait_impl {
    ($impl_name:ident) => {
        fn token_literal(&self) -> &str {
            self.token.literal.as_str()
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
}
