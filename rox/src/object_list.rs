use crate::RoxObject;


#[derive(Debug)]
pub struct ObjectList(Option<*mut RoxObject>);

impl ObjectList {
    pub fn new() -> ObjectList {
        ObjectList(None)
    }

    ///
    /// This adds an allocated object to the implicit linked list
    /// of objects tracked by this VM.
    ///
    pub fn add_object(&mut self, new_object: &mut RoxObject) {
        new_object.next_object = self.0;
        self.0 = Some(new_object);
    }

    pub fn reset(&mut self) {
        self.0 = None;
    }

    pub fn print_objects(&self) {
        unsafe {
            let mut current = self.0;
            while let Some(obj) = current {
                let obj_type = &(*obj).object_type;
                println!("{}", obj_type);
                current = (*obj).next_object;
            }
        }
    }
}
