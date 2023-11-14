

#[macro_export]
macro_rules! macro_node_trait_impl {
    ($impl_name:ident) => {
        #[allow(dead_code)]
        pub fn token_literal(&self) -> &str {
            self.token.literal.as_str()
        }
    }
}