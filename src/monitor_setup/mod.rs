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
                    Expr::Always { interval, expr, not: false } | 
                    Expr::Eventually { interval, expr, not: false } => 
                        Ok((
                            match ltl_expr {
                                Expr::Always { .. } => PropLTL::Always,
                                _ =>                   PropLTL::Eventually(false),
                            },
                            expr.compile_expression()?,
                            interval.as_deref().map(Expr::get_bound).transpose()?.map(|(a,b)| (a/1000, b/1000))
                        ).into()),
                    _ => Err(errors::Error::InvalidCompileExpr.into()) 
                }
            )
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?
        );
        Ok(())
    }
}






