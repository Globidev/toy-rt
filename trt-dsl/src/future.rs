
use std::{pin::Pin};
use futures::{prelude::*, future::Shared};
use std::rc::Rc;

pub struct PyFuture<T>(Shared<Pin<Box<dyn Future<Output = Rc<T>>>>>);

impl<T: 'static> PyFuture<T> {
    pub fn ready(x: T) -> Self {
        PyFuture(future::ready(Rc::new(x)).boxed_local().shared())
    }

    pub fn shared(&self) -> impl Future<Output = Rc<T>> {
        self.0.clone()
    }
}
