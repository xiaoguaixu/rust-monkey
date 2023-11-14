use crate::const_str_val_declare;

const_str_val_declare!(NONE_OBJ, "NONE");
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
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HashKey {
    pub object_type: String,
    pub value: String,
}



