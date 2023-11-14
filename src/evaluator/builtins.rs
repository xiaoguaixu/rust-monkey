use std::rc::Rc;

use crate::evaluator::base::new_error;
use crate::object;
use crate::object::{BuiltinFunction, ValueObject};

fn len(args: &Vec<ValueObject>) -> ValueObject {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    return match &args[0] {
        ValueObject::Array(v) => {
            ValueObject::Integer(v.elements.len() as i64)
        }
        ValueObject::StringValue(v) => {
            ValueObject::Integer(v.len() as i64)
        }
        _ => {
            new_error(format!("argument to `len` not supported, got {}",
                              args[0].object_type()))
        }
    };
}

fn puts(args: &Vec<ValueObject>) -> ValueObject {
    for v in args {
        println!("{}", v.inspect());
    }
    ValueObject::NULL
}

fn first(args: &Vec<ValueObject>) -> ValueObject {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    return match &args[0] {
        ValueObject::Array(v) => {
            if v.elements.len() > 0 {
                v.elements[0].clone()
            } else {
                ValueObject::NULL
            }
        }
        _ => {
            new_error(format!("argument to `first` must be ARRAY, got {}",
                              args[0].object_type()))
        }
    };
}

fn last(args: &Vec<ValueObject>) -> ValueObject {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    return match &args[0] {
        ValueObject::Array(v) => {
            if v.elements.len() > 1 {
                v.elements[v.elements.len() - 1].clone()
            } else {
                ValueObject::NULL
            }
        }
        _ => {
            new_error(format!("argument to `last` must be ARRAY, got {}",
                              args[0].object_type()))
        }
    };
}

fn rest(args: &Vec<ValueObject>) -> ValueObject {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    return match &args[0] {
        ValueObject::Array(v) => {
            let length = v.elements.len();
            if length > 0 {
                let mut ary: Vec<ValueObject> = vec![];
                for i in 1..length {
                    ary.push(v.elements[i].clone());
                }
                ValueObject::Array(object::Array {
                    elements: ary,
                })
            } else {
                ValueObject::NULL
            }
        }
        _ => {
            new_error(format!("argument to `rest` must be ARRAY, got {}",
                              args[0].object_type()))
        }
    };
}

fn push(args: &Vec<ValueObject>) -> ValueObject {
    if args.len() != 2 {
        return new_error(format!("wrong number of arguments. got={}, want=2", args.len()));
    }

    return match &args[0] {
        ValueObject::Array(v) => {
            let mut ary: Vec<ValueObject> = vec![];
            for item in &v.elements {
                ary.push(item.clone());
            }
            ary.push(args[1].clone());
            ValueObject::Array(object::Array {
                elements: ary,
            })
        }
        _ => {
            new_error(format!("argument to `push` must be ARRAY, got {}",
                              args[0].object_type()))
        }
    };
}

pub fn get_builtin_fn(key: &str) -> Option<Rc<BuiltinFunction>> {
    return match key {
        "len" => { Some(Rc::new(len)) }
        "puts" => { Some(Rc::new(puts)) }
        "first" => { Some(Rc::new(first)) }
        "last" => { Some(Rc::new(last)) }
        "rest" => { Some(Rc::new(rest)) }
        "push" => { Some(Rc::new(push)) }
        _ => { None }
    };
}