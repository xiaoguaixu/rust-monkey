pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
}

#[macro_export]
macro_rules! macro_node_trait_impl {
    ($impl_name:ident) => {
        fn token_literal(&self) -> &str {
            self.token.literal.as_str()
        }
    }
}