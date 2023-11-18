use std::rc::Rc;
use crate::object;

pub fn is_error(obj: &Rc<dyn object::Object>) -> bool {
    return if obj.as_any().is::<object::Error>() {
        true
    } else {
        false
    };
}

pub fn is_truthy(obj: &Rc<dyn object::Object>) -> bool {
    let xx = obj.as_any();
    return if xx.is::<object::NULL>() {
        false
    } else if xx.is::<object::Boolean>() {
        xx.downcast_ref::<object::Boolean>().unwrap().value
    } else { false };
}

pub fn new_error(message: String) -> Option<Rc<dyn object::Object>> {
    Some(Rc::new(object::Error {
        message,
    }))
}
