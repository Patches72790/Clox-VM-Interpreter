use std::cell::RefCell;
use std::rc::Rc;

pub type RcMut<T> = Rc<RefCell<T>>;
