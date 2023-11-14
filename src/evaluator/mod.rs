use std::collections::HashMap;

use crate::ast::ASTNode;
use crate::evaluator::base::*;
use crate::evaluator::builtins::get_builtin_fn;
use crate::object;
use crate::object::{Builtin, Environment, HashKey, HashPair, ValueObject};

mod evaluator_test;
mod base;
mod builtins;

pub fn eval(node: &ASTNode, env: &mut object::Environment) -> ValueObject {
    match node {
        ASTNode::Program(_) => {
            return eval_program(node, env);
        }
        ASTNode::BlockStatement(_) => {
            return eval_block_statement(node, env);
        }
        ASTNode::ExpressionStatement(v) => {
            return eval(&*v.expression, env);
        }
        ASTNode::ReturnStatement(v) => {
            let value = eval(&*v.return_value, env);
            if is_error(&value) {
                return value;
            }

            return ValueObject::ReturnValue(object::ReturnValue {
                value: Box::new(value),
            });
        }
        ASTNode::LetStatement(v) => {
            let value = eval(&*v.value, env);
            if is_error(&value) {
                return value;
            }

            let name = match &*v.name {
                ASTNode::Identifier(v) => { v }
                _ => { return value; }
            };
            env.set(name.value.as_str(), Box::new(value));
        }
        ASTNode::IntegerLiteral(v) => {
            return ValueObject::Integer(v.value);
        }
        ASTNode::StringLiteral(v) => {
            return ValueObject::StringValue(v.value.clone());
        }
        ASTNode::Boolean(v) => {
            return ValueObject::Boolean(v.value);
        }

        ASTNode::PrefixExpression(v) => {
            let right = eval(&*v.right, env);
            if is_error(&right) {
                return right;
            }
            return eval_prefix_expression(v.operator.as_str(), &right);
        }
        ASTNode::InfixExpression(v) => {
            let left = eval(&*v.left, env);
            if is_error(&left) {
                return left;
            }

            let right = eval(&*v.right, env);
            if is_error(&right) {
                return right;
            }

            return eval_infix_expression(v.operator.as_str(), &left, &right);
        }
        ASTNode::IfExpression(_) => {
            return eval_if_expression(node, env);
        }

        ASTNode::Identifier(_) => {
            return eval_identifier(node, env);
        }

        ASTNode::FunctionLiteral(v) => {
            let parameters = v.parameters.clone();
            let body = v.body.clone();
            return ValueObject::Function(object::Function {
                parameters,
                body,
                env: Box::new(env.clone()),
            });
        }

        ASTNode::CallExpression(v) => {
            let function = eval(&*v.function, env);
            if is_error(&function) {
                return function;
            }

            let args = eval_expressions(&v.arguments, env);
            if args.len() == 1 && is_error(&args[0]) {
                return args[0].clone();
            }

            return apply_function(&function, &args);
        }

        ASTNode::ArrayLiteral(v) => {
            let elements = eval_expressions(&v.elements, env);
            if elements.len() == 1 && is_error(&elements[0]) {
                return elements[0].clone();
            }

            return ValueObject::Array(object::Array {
                elements,
            });
        }

        ASTNode::IndexExpression(v) => {
            let left = eval(&*v.left, env);
            if is_error(&left) {
                return left;
            }
            let index = eval(&*v.index, env);
            if is_error(&index) {
                return index;
            }

            return eval_index_expression(&left, &index);
        }
        ASTNode::HashLiteral(_) => {
            return eval_hash_literal(node, env);
        }
        ASTNode::None => {}
    }

    ValueObject::None
}

fn eval_program(program: &ASTNode, env: &mut object::Environment) -> ValueObject {
    let mut rlt = ValueObject::None;

    let program = match program {
        ASTNode::Program(v) => { v }
        _ => { return rlt; }
    };

    for statement in &program.statements {
        rlt = eval(statement, env);
        match rlt {
            ValueObject::ReturnValue(v) => {
                return *v.value;
            }
            ValueObject::Error(v) => {
                return ValueObject::Error(v);
            }
            _ => {}
        }
    }
    rlt
}

fn eval_block_statement(statement: &ASTNode, env: &mut object::Environment) -> ValueObject {
    let mut rlt = ValueObject::None;

    let statement = match statement {
        ASTNode::BlockStatement(v) => { v }
        _ => { return rlt; }
    };

    for statement in &statement.statements {
        rlt = eval(statement, env);
        match rlt {
            ValueObject::ReturnValue(_) => { return rlt; }
            ValueObject::Error(_) => { return rlt; }
            _ => {}
        }
    }
    rlt
}

fn eval_prefix_expression(operator: &str, right: &ValueObject) -> ValueObject {
    return match operator {
        "!" => {
            eval_bang_operator_expression(right)
        }
        "-" => {
            eval_minus_prefix_operator_expression(right)
        }
        _ => {
            new_error(format!("unknown operator: {}{}", operator, right.object_type()))
        }
    };
}

fn eval_infix_expression(operator: &str, left: &ValueObject, right: &ValueObject) -> ValueObject {
    if left.object_type() == object::INTEGER_OBJ && right.object_type() == object::INTEGER_OBJ {
        return eval_integer_infix_expression(operator, left, right);
    } else if left.object_type() == object::STRING_OBJ && right.object_type() == object::STRING_OBJ {
        return eval_string_infix_expression(operator, left, right);
    } else if left.object_type() == object::BOOLEAN_OBJ && right.object_type() == object::BOOLEAN_OBJ {
        return eval_bool_infix_expression(operator, left, right);
    } else if left.object_type() != right.object_type() {
        return new_error(format!("type mismatch: {} {} {}", left.object_type(), operator, right.object_type()));
    } else {
        return new_error(format!("unknown operator: {} {} {}", left.object_type(), operator, right.object_type()));
    }
}

fn eval_if_expression(expression: &ASTNode, env: &mut object::Environment) -> ValueObject {
    let ie = match expression {
        ASTNode::IfExpression(v) => { v }
        _ => { return ValueObject::None; }
    };

    let condition = eval(&*ie.condition, env);
    if is_error(&condition) {
        return condition;
    }

    return if is_truthy(&condition) {
        eval(&*ie.consequence, env)
    } else if !ie.alternative.is_none() {
        eval(&*ie.alternative, env)
    } else {
        ValueObject::NULL
    };
}

fn eval_identifier(expression: &ASTNode, env: &mut object::Environment) -> ValueObject {
    let identifier = match expression {
        ASTNode::Identifier(v) => { v }
        _ => { return ValueObject::None; }
    };

    return match env.get(identifier.value.as_str()) {
        None => {
            match get_builtin_fn(identifier.value.as_str()) {
                None => { new_error(format!("identifier not found: {}", identifier.value)) }
                Some(v) => {
                    ValueObject::Builtin(Builtin {
                        func: v.clone(),
                    })
                }
            }
        }
        Some(v) => {
            *v.clone()
        }
    };
}

fn eval_index_expression(left: &ValueObject, index: &ValueObject) -> ValueObject {
    return if left.object_type() == object::ARRAY_OBJ && index.object_type() == object::INTEGER_OBJ {
        eval_array_index_expression(left, index)
    } else if left.object_type() == object::HASH_OBJ {
        eval_hash_index_expression(left, index)
    } else {
        new_error(format!("index operator not supported: {}", left.object_type()))
    };
}

fn eval_expressions(nodes: &Vec<ASTNode>, env: &mut object::Environment) -> Vec<ValueObject> {
    let mut rlt: Vec<ValueObject> = vec![];

    for v in nodes {
        let value = eval(v, env);
        if is_error(&value) {
            return vec![];
        }

        rlt.push(value);
    }

    rlt
}

fn eval_hash_literal(node: &ASTNode, env: &mut Environment) -> ValueObject {
    let node = match node {
        ASTNode::HashLiteral(v) => { v }
        _ => { return ValueObject::NULL; }
    };

    let mut pairs: HashMap<HashKey, HashPair> = HashMap::new();

    for (key_node, value_node) in &node.pairs {
        let key = eval(&*key_node, env);
        if is_error(&key) {
            return key;
        }

        if !key.is_hash() {
            return new_error(format!("unusable as hash key: {}", key.object_type()));
        }

        let value = eval(&*value_node, env);
        if is_error(&value) {
            return value;
        }
        pairs.insert(key.hash_key(), object::HashPair {
            key: Box::new(key),
            value: Box::new(value),
        });
    }

    ValueObject::Hash(object::Hash {
        pairs,
    })
}


fn apply_function(func: &ValueObject, nodes: &Vec<ValueObject>) -> ValueObject {
    return match func {
        ValueObject::Function(v) => {
            let mut extend_env = extend_function_env(func, nodes);
            let value = eval(&*v.body, &mut extend_env);
            match value {
                ValueObject::ReturnValue(v) => { *v.value.clone() }
                _ => { value }
            }
        }
        ValueObject::Builtin(v) => {
            (v.func)(nodes)
        }
        _ => {
            new_error(format!("not a function: {}", func.object_type()))
        }
    };
}

