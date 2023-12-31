use std::any::Any;
use std::rc::Rc;

use crate::{const_str_val_declare, object};

pub type ObjectType = str;

const_str_val_declare!(NULL_OBJ, "NULL");
const_str_val_declare!(ERROR_OBJ, "ERROR");

const_str_val_declare!(INTEGER_OBJ, "INTEGER");
const_str_val_declare!(BOOLEAN_OBJ, "BOOLEAN");
const_str_val_declare!(STRING_OBJ, "STRING");

const_str_val_declare!(RETURN_VALUE_OBJ, "RETURN_VALUE");

const_str_val_declare!(FUNCTION_OBJ, "FUNCTION");
const_str_val_declare!(BUILTIN_OBJ, "BUILTIN");

const_str_val_declare!(ARRAY_OBJ, "ARRAY");
const_str_val_declare!(HASH_OBJ, "HASH");


#[derive(Default, Eq, PartialEq, Hash)]
pub struct HashKey {
    pub object_type: String,
    pub value: String,
}

pub trait Downcast {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Object: Downcast {
    fn object_type(&self) -> &'static ObjectType;
    fn inspect(&self) -> String;

    fn is_hash(&self) -> bool { false }

    fn hash_key(&self) -> Option<HashKey> {
        None
    }
}

pub type BuiltinFunction = dyn Fn(&Vec<Rc<dyn object::Object>>) -> Option<Rc<dyn object::Object>>;

