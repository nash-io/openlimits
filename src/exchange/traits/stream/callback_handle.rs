use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CallbackHandle {
    pub rx: Box<dyn Any + Send>,
}