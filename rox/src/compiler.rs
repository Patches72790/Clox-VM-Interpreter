use std::cell::RefCell;

pub struct Compiler {
    func_table: RefCell<[Box<dyn FnMut()>; 1]>,
}

fn some_func() {
    println!("I'm a compiler function!");
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            func_table: RefCell::new([Box::new(some_func)]),
        }
    }

    pub fn compile(&self) {
        for each_fn in self.func_table.borrow_mut().iter_mut() {
            each_fn();
        }
    }
}
