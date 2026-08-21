mod rules;
pub mod operation_types;

#[cfg(test)]
mod rules_test;

use std::{error::Error};
use crate::{errors, program::{Program, expressions::Expr}};

impl Program {
    pub fn compile_properties(&mut self) -> Result<(), Box<dyn Error>> {
        self.environment = Some(
            self.expressions
            .iter()
            .map(|span_expr| &span_expr.expr)
            .map(|ltl_expr|
                match ltl_expr {
                    Expr::Always { interval:_, expr, not: false } | 
                    Expr::Eventually { interval:_, expr, not: false } => 
                        Ok((
                            expr.compile_expression()?
                        ).into()),
                    _ => Err(errors::Error::InvalidCompileExpr.into()) 
                }
            )
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?
        );
        Ok(())
    }
}






