use core::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast;
use crate::ast::Node;
pub use crate::object::base::*;
pub use crate::object::environment::Environment;

mod environment;
mod base;
mod object_test;


macro_rules! downcast_trait_impl {
    ($impl_name:ident) => {
        impl Downcast for $impl_name {
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
}

pub struct Integer {
    pub value: i64,
}

downcast_trait_impl!(Integer);

impl Object for Integer {
    fn object_type(&self) -> &'static ObjectType {
        return INTEGER_OBJ;
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn is_hash(&self) -> bool { true }

    fn hash_key(&self) -> Option<HashKey> {
        Some(HashKey {
            object_type: "Integer".to_string(),
            value: format!("{}", self.value),
        })
    }
}

pub struct Boolean {
    pub value: bool,
}

downcast_trait_impl!(Boolean);

impl Object for Boolean {
    fn object_type(&self) -> &'static ObjectType {
        return BOOLEAN_OBJ;
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn is_hash(&self) -> bool { true }

    fn hash_key(&self) -> Option<HashKey> {
        Some(HashKey {
            object_type: "Boolean".to_string(),
            value: format!("{}", self.value),
        })
    }
}

pub struct NULL {}

downcast_trait_impl!(NULL);

impl Object for NULL {
    fn object_type(&self) -> &'static ObjectType {
        return NULL_OBJ;
    }

    fn inspect(&self) -> String {
        "NULL".to_string()
    }
}

pub struct ReturnValue {
    pub value: Rc<dyn Object>,
}

downcast_trait_impl!(ReturnValue);

impl Object for ReturnValue {
    fn object_type(&self) -> &'static ObjectType {
        return RETURN_VALUE_OBJ;
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

pub struct Error {
    pub message: String,
}

downcast_trait_impl!(Error);

impl Object for Error {
    fn object_type(&self) -> &'static ObjectType {
        return ERROR_OBJ;
    }

    fn inspect(&self) -> String {
        self.message.clone()
    }
}

pub struct Function {
    pub parameters: Vec<ast::Identifier>,
    pub body: Rc<dyn ast::Statement>,
    pub env: Rc<Environment>,
}

downcast_trait_impl!(Function);

impl Object for Function {
    fn object_type(&self) -> &'static ObjectType {
        return FUNCTION_OBJ;
    }

    fn inspect(&self) -> String {
        let mut params: Vec<String> = Vec::new();
        for v in &self.parameters {
            params.push(v.string());
        }
        format!("fn({}) {}{}{}", params.join(","), "{\n", self.body.string(), "\n}")
    }
}

pub struct StringValue {
    pub value: String,
}

downcast_trait_impl!(StringValue);

impl Object for StringValue {
    fn object_type(&self) -> &'static ObjectType {
        return STRING_OBJ;
    }

    fn inspect(&self) -> String {
        self.value.clone()
    }

    fn is_hash(&self) -> bool { true }

    fn hash_key(&self) -> Option<HashKey> {
        Some(HashKey {
            object_type: "String".to_string(),
            value: format!("{}", self.value),
        })
    }
}

pub struct Builtin {
    pub func: Rc<BuiltinFunction>,
}

downcast_trait_impl!(Builtin);

impl Object for Builtin {
    fn object_type(&self) -> &'static ObjectType {
        return BUILTIN_OBJ;
    }

    fn inspect(&self) -> String {
        "builtin function".to_string()
    }
}

pub struct Array {
    pub elements: Vec<Rc<dyn Object>>,
}

downcast_trait_impl!(Array);

impl Object for Array {
    fn object_type(&self) -> &'static ObjectType {
        return ARRAY_OBJ;
    }

    fn inspect(&self) -> String {
        let mut elements: Vec<String> = Vec::new();
        for v in &self.elements {
            elements.push(v.inspect());
        }

        format!("[{}]", elements.join(","))
    }
}

pub struct HashPair {
    pub key: Rc<dyn Object>,
    pub value: Rc<dyn Object>,
}

pub struct Hash {
    pub pairs: HashMap<HashKey, HashPair>,
}

downcast_trait_impl!(Hash);

impl Object for Hash {
    fn object_type(&self) -> &'static ObjectType {
        return HASH_OBJ;
    }

    fn inspect(&self) -> String {
        let mut pairs: Vec<String> = Vec::new();
        for (_, value) in self.pairs.iter() {
            pairs.push(value.key.inspect() + ":" + value.value.inspect().as_str());
        }

        format!("{{{}}}", pairs.join(","))
    }
}