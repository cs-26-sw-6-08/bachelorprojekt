use std::error::Error;

use crate::{errors, program::Program};
pub mod expressions;
#[cfg(test)]
mod expressions_test;

impl Program {
    pub fn soundness_check(&self) -> Result<(), Box<dyn Error>> {
        for spannedexpr in self.expressions.iter() {
            match spannedexpr.expr.soundness_check() {
                Ok(_) => {},
                Err(_) => return Err(errors::Error::UnmonitorableLine(spannedexpr.line).into()),
            }
        }
            
        Ok(())
    }
}