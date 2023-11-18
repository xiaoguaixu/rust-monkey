mod evaluator_test;
mod base;
mod builtins;

use std::collections::HashMap;
use std::rc::Rc;

use crate::{ast, object};
use crate::evaluator::builtins::get_builtin_fn;
use self::base::*;

#[allow(dead_code)]
pub const NULL: object::NULL = object::NULL {};
#[allow(dead_code)]
pub const TRUE: &object::Boolean = &object::Boolean { value: true };
#[allow(dead_code)]
pub const FALSE: &object::Boolean = &object::Boolean { value: false };


#[allow(unused_variables)]
#[allow(dead_code)]
pub fn eval(node: &dyn ast::Node, env: &mut object::Environment) -> Option<Rc<dyn object::Object>> {
    let nd = node.as_any();
    if nd.is::<ast::Program>() {
        if let Some(program) = nd.downcast_ref::<ast::Program>() {
            return eval_program(program, env);
        }
    } else if nd.is::<ast::BlockStatement>() {
        if let Some(block) = nd.downcast_ref::<ast::BlockStatement>() {
            return eval_block_statement(block, env);
        }
    } else if nd.is::<ast::ExpressionStatement>() {
        if let Some(expression_statement) = nd.downcast_ref::<ast::ExpressionStatement>() {
            return eval(expression_statement.expression.upcast(), env);
        }
    } else if nd.is::<ast::ReturnStatement>() {
        if let Some(return_statement) = nd.downcast_ref::<ast::ReturnStatement>() {
            if return_statement.return_value.is_some() {
                if let Some(value) = eval(return_statement.return_value.as_ref().unwrap().upcast(), env) {
                    if is_error(&value) {
                        return Some(value);
                    }

                    return Some(Rc::new(object::ReturnValue { value }));
                }
            }
        }
    } else if nd.is::<ast::LetStatement>() {
        if let Some(statement) = nd.downcast_ref::<ast::LetStatement>() {
            if statement.value.is_some() {
                if let Some(value) = eval(statement.value.as_ref().unwrap().upcast(), env) {
                    if is_error(&value) {
                        return Some(value);
                    }

                    env.set(statement.name.value.as_str(), value);
                }
            }
        }
    } else if nd.is::<ast::IntegerLiteral>() {
        if let Some(expression) = nd.downcast_ref::<ast::IntegerLiteral>() {
            return Some(Rc::new(object::Integer {
                value: expression.value,
            }));
        }
    } else if nd.is::<ast::StringLiteral>() {
        if let Some(expression) = nd.downcast_ref::<ast::StringLiteral>() {
            return Some(Rc::new(object::StringValue {
                value: expression.value.clone(),
            }));
        }
    } else if nd.is::<ast::Boolean>() {
        if let Some(expression) = nd.downcast_ref::<ast::Boolean>() {
            return Some(Rc::new(object::Boolean {
                value: expression.value,
            }));
        }
    } else if nd.is::<ast::PrefixExpression>() {
        if let Some(expression) = nd.downcast_ref::<ast::PrefixExpression>() {
            if let Some(value) = eval(expression.right.upcast(), env) {
                if is_error(&value) {
                    return Some(value);
                }
                return eval_prefix_expression(expression.operator.as_str(), &value);
            }
        }
    } else if nd.is::<ast::InfixExpression>() {
        if let Some(expression) = nd.downcast_ref::<ast::InfixExpression>() {
            let left = match eval(expression.left.upcast(), env) {
                None => { return None; }
                Some(v) => { v }
            };

            if is_error(&left) {
                return Some(left);
            }

            let right = match eval(expression.right.upcast(), env) {
                None => { return None; }
                Some(v) => { v }
            };

            if is_error(&right) {
                return Some(right);
            }
            return eval_infix_expression(expression.operator.as_str(), &left, &right);
        }
    } else if nd.is::<ast::IfExpression>() {
        if let Some(expression) = nd.downcast_ref::<ast::IfExpression>() {
            return eval_if_expression(expression, env);
        }
    } else if nd.is::<ast::Identifier>() {
        if let Some(expression) = nd.downcast_ref::<ast::Identifier>() {
            return eval_identifier(expression, env);
        }
    } else if nd.is::<ast::FunctionLiteral>() {
        if let Some(expression) = nd.downcast_ref::<ast::FunctionLiteral>() {
            let parameters = expression.parameters.clone();
            let body = expression.body.clone();
            return Some(Rc::new(object::Function {
                parameters,
                body,
                env: Rc::new(env.clone()),
            }));
        }
    } else if nd.is::<ast::CallExpression>() {
        if let Some(expression) = nd.downcast_ref::<ast::CallExpression>() {
            let function = match eval(expression.function.upcast(), env) {
                None => { return None; }
                Some(v) => { v }
            };

            if is_error(&function) {
                return Some(function);
            }

            let args = eval_expressions(&expression.arguments, env);

            if args.len() == 1 && is_error(&args[0]) {
                return Some(args[0].clone());
            }
            return apply_function(&function, &args);
        }
    } else if nd.is::<ast::ArrayLiteral>() {
        if let Some(expression) = nd.downcast_ref::<ast::ArrayLiteral>() {
            let elements = eval_expressions(&expression.elements, env);
            if elements.len() == 1 && is_error(&elements[0]) {
                return Some(elements[0].clone());
            }

            return Some(Rc::new(object::Array {
                elements,
            }));
        }
    } else if nd.is::<ast::IndexExpression>() {
        if let Some(expression) = nd.downcast_ref::<ast::IndexExpression>() {
            let left = match eval(expression.left.upcast(), env) {
                None => { return None; }
                Some(v) => { v }
            };

            if is_error(&left) {
                return Some(left);
            }

            let index = match eval(expression.index.upcast(), env) {
                None => { return None; }
                Some(v) => { v }
            };

            if is_error(&index) {
                return Some(index);
            }

            return eval_index_expression(&left, &index);
        }
    } else if nd.is::<ast::HashLiteral>() {
        if let Some(expression) = nd.downcast_ref::<ast::HashLiteral>() {
            return eval_hash_literal(expression, env);
        }
    }

    None
}

pub fn eval_program(program: &ast::Program, env: &mut object::Environment) -> Option<Rc<dyn object::Object>> {
    let mut rlt = None;
    for statement in &program.statements {
        rlt = eval(statement.upcast(), env);
        if let Some(v) = rlt.as_ref() {
            if v.as_any().is::<object::ReturnValue>() {
                return Some(
                    v.as_any().downcast_ref::<object::ReturnValue>().unwrap().value.clone()
                );
            } else if v.as_any().is::<object::Error>() {
                return rlt;
            }
        }
    }
    rlt
}

pub fn eval_block_statement(block_statement: &ast::BlockStatement, env: &mut object::Environment) -> Option<Rc<dyn object::Object>> {
    let mut rlt = None;
    for statement in &block_statement.statements {
        rlt = eval(statement.upcast(), env);
        if let Some(v) = rlt.as_ref() {
            if v.as_any().is::<object::ReturnValue>() {
                return rlt;
            } else if v.as_any().is::<object::Error>() {
                return rlt;
            }
        }
    }
    rlt
}

fn eval_prefix_expression(operator: &str, right: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
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

fn eval_infix_expression(operator: &str, left: &Rc<dyn object::Object>, right: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
    return if left.object_type() == object::INTEGER_OBJ && right.object_type() == object::INTEGER_OBJ {
        eval_integer_infix_expression(operator, left, right)
    } else if left.object_type() == object::STRING_OBJ && right.object_type() == object::STRING_OBJ {
        eval_string_infix_expression(operator, left, right)
    } else if left.object_type() == object::BOOLEAN_OBJ && right.object_type() == object::BOOLEAN_OBJ {
        eval_bool_infix_expression(operator, left, right)
    } else if left.object_type() != right.object_type() {
        new_error(format!("type mismatch: {} {} {}", left.object_type(), operator, right.object_type()))
    } else {
        new_error(format!("unknown operator: {} {} {}", left.object_type(), operator, right.object_type()))
    };
}

fn eval_if_expression(expression: &ast::IfExpression, env: &mut object::Environment) -> Option<Rc<dyn object::Object>> {
    let condition = match eval(expression.condition.upcast(), env) {
        None => { return None; }
        Some(v) => { v }
    };

    if is_error(&condition) {
        return Some(condition);
    }

    return if is_truthy(&condition) {
        eval(expression.consequence.upcast(), env)
    } else if !expression.alternative.is_none() {
        eval(expression.alternative.as_ref().unwrap().upcast(), env)
    } else {
        Some(Rc::new(NULL))
    };
}

fn eval_identifier(expression: &ast::Identifier, env: &mut object::Environment) -> Option<Rc<dyn object::Object>> {
    return match env.get(expression.value.as_str()) {
        None => {
            match get_builtin_fn(expression.value.as_str()) {
                None => {
                    new_error(format!("identifier not found: {}", expression.value))
                }
                Some(v) => {
                    Some(Rc::new(object::Builtin{
                        func: v,
                    }))
                }
            }

        }
        Some(v) => {
            Some(v.clone())
        }
    };
}

fn eval_expressions(nodes: &Vec<Rc<dyn ast::Expression>>, env: &mut object::Environment) -> Vec<Rc<dyn object::Object>> {
    let mut rlt: Vec<Rc<dyn object::Object>> = vec![];

    for v in nodes {
        match eval(v.upcast(), env) {
            None => { return rlt; }
            Some(v) => {
                if is_error(&v) {
                    return rlt;
                }
                rlt.push(v);
            }
        }
    }

    rlt
}

fn eval_hash_literal(expression: &ast::HashLiteral, env: &mut object::Environment) -> Option<Rc<dyn object::Object>> {
    let mut pairs: HashMap<object::HashKey, object::HashPair> = HashMap::new();
    for (key_node, value_node) in &expression.pairs {
        let key = eval(key_node.upcast(), env);
        if key.is_none() || is_error(key.as_ref().unwrap()) {
            return key;
        }

        let key = key.unwrap();

        if !key.is_hash() {
            return new_error(format!("unusable as hash key: {}", key.object_type()));
        }

        let value = eval(value_node.upcast(), env);
        if value.is_none() || is_error(value.as_ref().unwrap()) {
            return value;
        }

        let value = value.unwrap();

        pairs.insert(key.hash_key().unwrap(), object::HashPair {
            key: key.clone(),
            value: value.clone(),
        });
    }

    Some(Rc::new(object::Hash {
        pairs,
    }))
}

fn eval_index_expression(left: &Rc<dyn object::Object>, index: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
    return if left.object_type() == object::ARRAY_OBJ && index.object_type() == object::INTEGER_OBJ {
        eval_array_index_expression(left, index)
    } else if left.object_type() == object::HASH_OBJ {
        eval_hash_index_expression(left, index)
    } else {
        new_error(format!("index operator not supported: {}", left.object_type()))
    };
}

fn eval_array_index_expression(left: &Rc<dyn object::Object>, index: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
    let ary_obj = left.as_any().downcast_ref::<object::Array>().unwrap();
    let index = index.as_any().downcast_ref::<object::Integer>().unwrap().value as usize;

    let max = ary_obj.elements.len() - 1;
    if index > max {
        return Some(Rc::new(object::NULL {}));
    }
    return Some(ary_obj.elements[index].clone());
}

fn eval_hash_index_expression(left: &Rc<dyn object::Object>, index: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
    let hash_obj = left.as_any().downcast_ref::<object::Hash>().unwrap();

    if !index.is_hash() {
        return new_error(format!("unusable as hash key: {}", index.object_type()));
    }

    return match index.hash_key() {
        None => { new_error(format!("unusable as hash key: {}", index.object_type())) }
        Some(v) => {
            match hash_obj.pairs.get(&v) {
                None => { None }
                Some(v) => { Some(v.value.clone()) }
            }
        }
    };
}

fn eval_integer_infix_expression(operator: &str, left: &Rc<dyn object::Object>, right: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
    let left_value = left.as_any().downcast_ref::<object::Integer>().unwrap().value;
    let right_value = right.as_any().downcast_ref::<object::Integer>().unwrap().value;

    return match operator {
        "+" => { Some(Rc::new(object::Integer { value: left_value + right_value })) }
        "-" => { Some(Rc::new(object::Integer { value: left_value - right_value })) }
        "*" => { Some(Rc::new(object::Integer { value: left_value * right_value })) }
        "/" => { Some(Rc::new(object::Integer { value: left_value / right_value })) }
        "<" => { Some(Rc::new(object::Boolean { value: left_value < right_value })) }
        ">" => { Some(Rc::new(object::Boolean { value: left_value > right_value })) }
        "==" => { Some(Rc::new(object::Boolean { value: left_value == right_value })) }
        "!=" => { Some(Rc::new(object::Boolean { value: left_value != right_value })) }
        _ => { new_error(format!("unknown operator: {} {} {}", operator, left.object_type(), right.object_type())) }
    };
}

fn eval_string_infix_expression(operator: &str, left: &Rc<dyn object::Object>, right: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
    if operator != "+" {
        return new_error(format!("unknown operator: {} {} {}",
                                 left.object_type(), operator, right.object_type()));
    }

    let left_value = left.as_any().downcast_ref::<object::StringValue>().unwrap();
    let right_value = right.as_any().downcast_ref::<object::StringValue>().unwrap();

    Some(Rc::new(object::StringValue { value: left_value.value.clone() + right_value.value.as_str() }))
}

fn eval_bool_infix_expression(operator: &str, left: &Rc<dyn object::Object>, right: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
    let left_value = left.as_any().downcast_ref::<object::Boolean>().unwrap().value;
    let right_value = right.as_any().downcast_ref::<object::Boolean>().unwrap().value;
    return match operator {
        "==" => { Some(Rc::new(object::Boolean { value: left_value == right_value })) }
        "!=" => { Some(Rc::new(object::Boolean { value: left_value != right_value })) }
        _ => { new_error(format!("unknown operator: {} {} {}", left.object_type(), operator, right.object_type())) }
    };
}

pub fn eval_bang_operator_expression(right: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
    let right = right.as_any();
    return if right.is::<object::Boolean>() {
        let xx = right.downcast_ref::<object::Boolean>().unwrap();
        Some(Rc::new(object::Boolean {
            value: !xx.value,
        }))
    } else {
        Some(Rc::new(object::Boolean {
            value: false,
        }))
    }
}

pub fn eval_minus_prefix_operator_expression(right: &Rc<dyn object::Object>) -> Option<Rc<dyn object::Object>> {
    return if right.as_any().is::<object::Integer>() {
        let xx = right.as_any().downcast_ref::<object::Integer>().unwrap();
        Some(Rc::new(object::Integer {
            value: xx.value * (-1),
        }))
    } else {
        new_error(format!("unknown operator: -{}", right.object_type()))
    }
}

fn apply_function(func: &Rc<dyn object::Object>, nodes: &Vec<Rc<dyn object::Object>>) -> Option<Rc<dyn object::Object>> {
    let any = func.as_any();
    if any.is::<object::Function>() {
        let function = any.downcast_ref::<object::Function>().unwrap();
        let mut extend_env = extend_function_env(function, nodes);
        let value = eval(function.body.upcast(), &mut extend_env);
        if value.is_some() {
            return value;
        }

        let value = value.unwrap();
        return if value.as_any().is::<object::ReturnValue>() {
            Some(value.as_any().downcast_ref::<object::ReturnValue>().unwrap().value.clone())
        } else {
            Some(value)
        };
    } else if any.is::<object::Builtin>() {
        let v = any.downcast_ref::<object::Builtin>().unwrap();
        return (v.func)(nodes);
    }
    None
}

fn extend_function_env(func: &object::Function, args: &Vec<Rc<dyn object::Object>>) -> object::Environment {
    let mut env = object::Environment::new_enclosed_environment(&*func.env);

    let mut idx: usize = 0;
    for v in &func.parameters {
        env.set(v.value.as_str(), args[idx].clone());
        idx = idx + 1;
    }
    env
}

