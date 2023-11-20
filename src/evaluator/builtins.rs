use std::rc::Rc;
use std::collections::HashMap;

use crate::evaluator::base::new_error;
use crate::object;
use crate::object::{BuiltinFunction};

fn len(args: &Vec<Rc<dyn object::Object>>) -> Option<Rc<dyn object::Object>> {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    let any = args[0].as_any();

    return if any.is::<object::Array>() {
        let value = any.downcast_ref::<object::Array>().unwrap();
        Some(Rc::new(object::Integer {
            value: value.elements.len() as i64,
        }))
    } else if any.is::<object::StringValue>() {
        let value = any.downcast_ref::<object::StringValue>().unwrap();
        Some(Rc::new(object::Integer {
            value: value.value.len() as i64,
        }))
    } else {
        new_error(format!("argument to `len` not supported, got {}",
                          args[0].object_type()))
    }
}

fn puts(args: &Vec<Rc<dyn object::Object>>) -> Option<Rc<dyn object::Object>> {
    for v in args {
        println!("{}", v.inspect());
    }
    None
}

fn first(args: &Vec<Rc<dyn object::Object>>) -> Option<Rc<dyn object::Object>>{
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    let any = args[0].as_any();

    return if any.is::<object::Array>() {
        let v = any.downcast_ref::<object::Array>().unwrap();
        if v.elements.len() > 0 {
            Some(v.elements[0].clone())
        } else {
            None
        }
    } else {
        new_error(format!("argument to `first` must be ARRAY, got {}",
                          args[0].object_type()))
    };
}

fn last(args: &Vec<Rc<dyn object::Object>>) -> Option<Rc<dyn object::Object>> {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    let any = args[0].as_any();

    return if any.is::<object::Array>() {
        let v = any.downcast_ref::<object::Array>().unwrap();
        if v.elements.len() > 1 {
            Some(v.elements[v.elements.len() - 1].clone())
        } else {
            None
        }
    } else {
        new_error(format!("argument to `last` must be ARRAY, got {}",
                          args[0].object_type()))
    };
}

fn rest(args: &Vec<Rc<dyn object::Object>>) -> Option<Rc<dyn object::Object>> {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    let any = args[0].as_any();

    return if any.is::<object::Array>() {
        let v = any.downcast_ref::<object::Array>().unwrap();
        let length = v.elements.len();
        if length > 0 {
            let mut ary: Vec<Rc<dyn object::Object>> = vec![];
            for i in 1..length {
                ary.push(v.elements[i].clone());
            }
            Some(Rc::new(object::Array {
                elements: ary,
            }))
        } else {
            None
        }
    } else {
        new_error(format!("argument to `first` must be ARRAY, got {}",
                          args[0].object_type()))
    };
}

fn push(args: &Vec<Rc<dyn object::Object>>) -> Option<Rc<dyn object::Object>> {
    if args.len() != 2 {
        return new_error(format!("wrong number of arguments. got={}, want=2", args.len()));
    }

    let any = args[0].as_any();

    return if any.is::<object::Array>() {
        let v = any.downcast_ref::<object::Array>().unwrap();
        let mut ary: Vec<Rc<dyn object::Object>> = vec![];
        for item in &v.elements {
            ary.push(item.clone());
        }
        ary.push(args[1].clone());
        Some(Rc::new(object::Array {
            elements: ary,
        }))
    } else {
        new_error(format!("argument to `push` must be ARRAY, got {}",
                          args[0].object_type()))
    };
}



thread_local! {
    pub static BUILTIN_FN: HashMap<&'static str, Rc<BuiltinFunction> > = init_builtin_fun_map();
}

pub fn get_builtin_fn(key: &str) -> Option<Rc<BuiltinFunction>> {
    // return match key {
    //     "len" => { Some(Rc::new(len)) }
    //     "puts" => { Some(Rc::new(puts)) }
    //     "first" => { Some(Rc::new(first)) }
    //     "last" => { Some(Rc::new(last)) }
    //     "rest" => { Some(Rc::new(rest)) }
    //     "push" => { Some(Rc::new(push)) }
    //     _ => { None }
    // };

    BUILTIN_FN.with(|val|{
        if let Some(v) = val.get(key) {
            return Some(v.clone());
        }
        return None;
    })
}

pub  fn init_builtin_fun_map() -> HashMap<&'static str, Rc<BuiltinFunction> > {
    let mut rlt: HashMap<&'static str, Rc<BuiltinFunction> > = HashMap::new();
    rlt.insert("len", Rc::new(len));
    rlt.insert("puts", Rc::new(puts));
    rlt.insert("first",Rc::new(first));
    rlt.insert("last",Rc::new(last));
    rlt.insert("rest", Rc::new(rest));
    rlt.insert("push", Rc::new(push));

    return rlt;
}