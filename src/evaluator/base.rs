use crate::ast::ASTNode;
use crate::object::{Environment, ValueObject};

pub fn eval_array_index_expression(left: &ValueObject, index: &ValueObject) -> ValueObject {
    let ary_obj = match left {
        ValueObject::Array(v) => { v }
        _ => { return ValueObject::NULL; }
    };

    let index = match index {
        ValueObject::Integer(v) => { *v as usize }
        _ => { return ValueObject::NULL; }
    };

    let max = ary_obj.elements.len() - 1;
    if index > max {
        return ValueObject::NULL;
    }
    return ary_obj.elements[index].clone();
}

pub fn eval_hash_index_expression(left: &ValueObject, index: &ValueObject) -> ValueObject {
    let hash_obj = match left {
        ValueObject::Hash(v) => { v }
        _ => { return ValueObject::NULL; }
    };

    if !index.is_hash() {
        return new_error(format!("unusable as hash key: {}", index.object_type()));
    }

    let hash_key = index.hash_key();
    return match hash_obj.pairs.get(&hash_key) {
        None => { ValueObject::NULL }
        Some(v) => { *v.value.clone() }
    };
}

pub fn extend_function_env(func: &ValueObject, args: &Vec<ValueObject>) -> Environment {
    let func = match func {
        ValueObject::Function(v) => { v }
        _ => { return Environment::new(); }
    };
    let mut env = Environment::new_enclosed_environment(&*func.env);

    let mut idx: usize = 0;
    for v in &func.parameters {
        let xx = match v {
            ASTNode::Identifier(v) => { v.value.clone() }
            _ => { return Environment::new(); }
        };
        env.set(xx.as_str(), Box::new(args[idx].clone()));
        idx = idx + 1;
    }
    env
}

pub fn eval_bool_infix_expression(operator: &str, left: &ValueObject, right: &ValueObject) -> ValueObject {
    let left_value = match *left {
        ValueObject::Boolean(v) => { v }
        _ => { false }
    };

    let right_value = match *right {
        ValueObject::Boolean(v) => { v }
        _ => { false }
    };

    return match operator {
        "==" => { ValueObject::Boolean(left_value == right_value) }
        "!=" => { ValueObject::Boolean(left_value != right_value) }
        _ => { new_error(format!("unknown operator: {} {} {}", left.object_type(), operator, right.object_type())) }
    };
}

pub
fn eval_integer_infix_expression(operator: &str, left: &ValueObject, right: &ValueObject) -> ValueObject {
    let left_value = match *left {
        ValueObject::Integer(v) => { v }
        _ => { 0 }
    };

    let right_value = match *right {
        ValueObject::Integer(v) => { v }
        _ => { 0 }
    };
    return match operator {
        "+" => { ValueObject::Integer(left_value + right_value) }
        "-" => { ValueObject::Integer(left_value - right_value) }
        "*" => { ValueObject::Integer(left_value * right_value) }
        "/" => { ValueObject::Integer(left_value / right_value) }
        "<" => { ValueObject::Boolean(left_value < right_value) }
        ">" => { ValueObject::Boolean(left_value > right_value) }
        "==" => { ValueObject::Boolean(left_value == right_value) }
        "!=" => { ValueObject::Boolean(left_value != right_value) }
        _ => { new_error(format!("unknown operator: {} {} {}", operator, left.object_type(), right.object_type())) }
    };
}

pub fn eval_string_infix_expression(operator: &str, left: &ValueObject, right: &ValueObject) -> ValueObject {
    if operator != "+" {
        return new_error(format!("unknown operator: {} {} {}",
                                 left.object_type(), operator, right.object_type()));
    }

    let left_value = match left {
        ValueObject::StringValue(v) => { v.clone() }
        _ => { "".to_string() }
    };

    let right_value = match right {
        ValueObject::StringValue(v) => { v.clone() }
        _ => { "".to_string() }
    };

    return ValueObject::StringValue(left_value + right_value.as_str());
}

pub fn eval_bang_operator_expression(right: &ValueObject) -> ValueObject {
    return match right {
        ValueObject::Boolean(v) => { ValueObject::Boolean(!*v) }
        _ => { ValueObject::Boolean(false) }
    };
}

pub fn eval_minus_prefix_operator_expression(right: &ValueObject) -> ValueObject {
    return match right {
        ValueObject::Integer(v) => { ValueObject::Integer(*v * (-1)) }
        _ => {
            new_error(format!("unknown operator: -{}", right.object_type()))
        }
    };
}

pub fn is_error(obj: &ValueObject) -> bool {
    return match obj {
        ValueObject::None => { true }
        ValueObject::Error(_) => { true }
        _ => { false }
    };
}

pub fn is_truthy(obj: &ValueObject) -> bool {
    return match obj {
        ValueObject::None => { false }
        ValueObject::NULL => { false }
        ValueObject::Boolean(v) => { *v }
        _ => { true }
    };
}

pub fn new_error(msg: String) -> ValueObject {
    ValueObject::Error(msg)
}