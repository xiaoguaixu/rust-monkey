#[macro_export]
macro_rules! const_val_declare {
    ($const_name:ident, $const_type:ty, $const_value:expr) => {
        #[allow(dead_code)]
        pub const $const_name : $const_type = $const_value;
    }
}

#[macro_export]
macro_rules! const_str_val_declare {
    ($const_name:ident, $const_value:literal) => {
        #[allow(dead_code)]
        pub const $const_name : &str = $const_value;
    }
}

pub struct Nil {}

pub const NIL: Nil = Nil {};

pub enum VariantValue {
    ValueInt(i64),
    ValueBool(bool),
    ValueString(String),
    ValueIntArray(Vec<i64>),
    ValueNull(Nil),
}

impl VariantValue {
    pub fn as_int(&self) -> i64 {
        return match self {
            VariantValue::ValueInt(v) => { *v }
            _ => { 0 }
        };
    }

    pub fn as_string(&self) -> String {
        return match self {
            VariantValue::ValueString(v) => { v.clone() }
            _ => { "".to_string() }
        };
    }

    pub fn as_bool(&self) -> bool {
        return match self {
            VariantValue::ValueBool(v) => { *v }
            _ => { false }
        };
    }
}

pub trait Overloaded<T> {
    fn from_value(value: T) -> Self;
}

impl Overloaded<&str> for VariantValue {
    fn from_value(value: &str) -> Self {
        VariantValue::ValueString(value.to_string())
    }
}

impl Overloaded<i64> for VariantValue {
    fn from_value(value: i64) -> Self {
        VariantValue::ValueInt(value)
    }
}

impl Overloaded<Nil> for VariantValue {
    fn from_value(value: Nil) -> Self {
        VariantValue::ValueNull(value)
    }
}

impl Overloaded<Vec<i64>> for VariantValue {
    fn from_value(value: Vec<i64>) -> Self {
        VariantValue::ValueIntArray(value.clone())
    }
}

impl Overloaded<bool> for VariantValue {
    fn from_value(value: bool) -> Self {
        VariantValue::ValueBool(value)
    }
}