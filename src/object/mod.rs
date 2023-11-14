use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::ASTNode;
pub use crate::object::base::*;
pub use crate::object::environment::Environment;

mod environment;
mod base;
mod object_test;


pub type BuiltinFunction = dyn Fn(&Vec<ValueObject>) -> ValueObject;


#[derive(Clone)]
pub enum ValueObject {
    Integer(i64),
    Boolean(bool),
    StringValue(String),
    ReturnValue(ReturnValue),
    Error(String),
    Function(Function),
    Builtin(Builtin),
    Array(Array),
    Hash(Hash),
    NULL,
    None,
}

impl ValueObject {
    pub fn is_none(&self) -> bool {
        return match self {
            ValueObject::None => { true }
            _ => { false }
        };
    }

    pub fn is_null(&self) -> bool {
        return match self {
            ValueObject::NULL => { true }
            _ => { false }
        };
    }

    #[allow(dead_code)]
    pub fn as_int(&self) -> i64 {
        return match self {
            ValueObject::Integer(v) => { *v }
            _ => { 0 }
        };
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        return match self {
            ValueObject::StringValue(v) => { v.clone() }
            _ => { "".to_string() }
        };
    }

    #[allow(dead_code)]
    pub fn as_bool(&self) -> bool {
        return match self {
            ValueObject::Boolean(v) => { *v }
            _ => { false }
        };
    }
    pub fn inspect(&self) -> String {
        match self {
            ValueObject::Integer(v) => { format!("{}", v) }
            ValueObject::Boolean(v) => { format!("{}", v) }
            ValueObject::StringValue(v) => { format!("{}", v) }
            ValueObject::NULL => { "NULL".to_string() }
            ValueObject::ReturnValue(v) => { v.inspect() }
            ValueObject::Error(v) => { format!("{}", v) }
            ValueObject::Function(v) => { v.inspect() }
            ValueObject::Builtin(v) => { v.inspect() }
            ValueObject::Array(v) => { v.inspect() }
            ValueObject::Hash(v) => { v.inspect() }
            ValueObject::None => { "None".to_string() }
        }
    }
    pub fn object_type(&self) -> &str {
        return match self {
            ValueObject::Integer(_) => { INTEGER_OBJ }
            ValueObject::Boolean(_) => { BOOLEAN_OBJ }
            ValueObject::StringValue(_) => { STRING_OBJ }
            ValueObject::ReturnValue(_) => { RETURN_VALUE_OBJ }
            ValueObject::Error(_) => { ERROR_OBJ }
            ValueObject::Function(_) => { FUNCTION_OBJ }
            ValueObject::Builtin(_) => { BUILTIN_OBJ }
            ValueObject::Array(_) => { ARRAY_OBJ }
            ValueObject::Hash(_) => { HASH_OBJ }
            ValueObject::NULL => { NULL_OBJ }
            ValueObject::None => { NONE_OBJ }
        };
    }

    pub fn is_hash(&self) -> bool {
        return match self {
            ValueObject::Integer(_) => { true }
            ValueObject::Boolean(_) => { true }
            ValueObject::StringValue(_) => { true }
            _ => { false }
        };
    }

    pub fn hash_key(&self) -> HashKey {
        return match self {
            ValueObject::Integer(v) => {
                HashKey {
                    object_type: "Integer".to_string(),
                    value: format!("{}", v),
                }
            }
            ValueObject::Boolean(v) => {
                HashKey {
                    object_type: "Boolean".to_string(),
                    value: format!("{}", v),
                }
            }
            ValueObject::StringValue(v) => {
                HashKey {
                    object_type: "String".to_string(),
                    value: format!("{}", v),
                }
            }
            _ => { HashKey { object_type: "".to_string(), value: "".to_string() } }
        };
    }
}

#[derive(Clone)]
pub struct ReturnValue {
    pub value: Box<ValueObject>,
}

impl ReturnValue {
    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

#[derive(Clone)]
pub struct Function {
    pub parameters: Vec<ASTNode>,
    pub body: Box<ASTNode>,
    pub env: Box<Environment>,
}

impl Function {
    fn inspect(&self) -> String {
        let mut params: Vec<String> = Vec::new();
        for v in &self.parameters {
            params.push(v.string());
        }
        format!("fn({}) {}{}{}", params.join(","), "{\n", self.body.string(), "\n}")
    }
}

#[derive(Clone)]
pub struct Builtin {
    pub func: Rc<BuiltinFunction>,
}

impl Builtin {
    fn inspect(&self) -> String {
        "builtin function".to_string()
    }
}

#[derive(Clone)]
pub struct Array {
    pub elements: Vec<ValueObject>,
}

impl Array {
    fn inspect(&self) -> String {
        let mut elements: Vec<String> = Vec::new();
        for v in &self.elements {
            elements.push(v.inspect());
        }

        format!("[{}]", elements.join(","))
    }
}

#[derive(Clone)]
pub struct HashPair {
    pub key: Box<ValueObject>,
    pub value: Box<ValueObject>,
}

#[derive(Clone)]
pub struct Hash {
    pub pairs: HashMap<HashKey, HashPair>,
}

impl Hash {
    fn inspect(&self) -> String {
        let mut pairs: Vec<String> = Vec::new();
        for (_, value) in self.pairs.iter() {
            pairs.push(value.key.inspect() + ":" + value.value.inspect().as_str());
        }

        format!("{{{}}}", pairs.join(","))
    }
}