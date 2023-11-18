#[cfg(test)]
mod object_test {
    use crate::object::StringValue;

    #[allow(unused_variables)]
    #[test]
    fn test_string_hash_key() {
        let hello1 = StringValue {
            value: "Hello World".to_string(),
        };

        let hello2 = StringValue {
            value: "Hello World".to_string(),
        };
    }
}