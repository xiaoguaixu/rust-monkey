use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;

#[derive(Clone)]
pub struct Environment {
    pub store: HashMap<String, Rc<dyn Object>>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Self {
            store: Default::default(),
            outer: None,
        }
    }

    pub fn new_enclosed_environment(outer: &Environment) -> Environment {
        Self {
            store: Default::default(),
            outer: Some(Box::new(outer.clone())),
        }
    }
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&Rc<dyn Object>> {
        return match self.store.get(name) {
            None => {
                match &self.outer {
                    None => { None }
                    Some(v) => {
                        v.get(name)
                    }
                }
            }
            Some(v) => { Some(v) }
        };
    }

    #[allow(dead_code)]
    pub fn set(&mut self, name: &str, value: Rc<dyn Object>) {
        self.store.insert(name.to_string(), value);
    }
}