use std::{rc::Rc, sync::Mutex};

pub struct SharedPtr<T>(Rc<Mutex<T>>);

impl<T> Clone for SharedPtr<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> SharedPtr<T> {
    pub fn new(v: T) -> Self {
        Self(Rc::new(Mutex::new(v)))
    }

    pub fn lock(&self) -> std::sync::MutexGuard<T> {
        self.0.lock().unwrap()
    }
}
