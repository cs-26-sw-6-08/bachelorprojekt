use std::error::Error;

use crate::errors;



pub trait ExtVec<T> {
    fn with(self, item: T) -> Self;
    fn chain<I: IntoIterator<Item = T>>(self, iter: I) -> Self; 
    fn pop_or_err(&mut self) -> Result<T, Box<dyn Error>>;
}

impl<T> ExtVec<T> for Vec<T> {
    #[inline]
    /// Consumes the vec, pushes an item, and returns the vec.
    fn with(mut self, item: T) -> Self {
        self.push(item);
        self 
    }

    #[inline]
    /// Consumes the vec, extends it with an iterator, and returns the vec.
    fn chain<I: IntoIterator<Item = T>>(mut self, iter: I) -> Self {
        self.extend(iter);
        self
    }

    #[inline]
    fn pop_or_err(&mut self) -> Result<T, Box<dyn Error>> {
        self.pop().ok_or(errors::Error::ValueStackPop.into())
    }
}
