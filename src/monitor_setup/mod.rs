pub mod operation_types;
mod rules;

#[cfg(test)]
mod rules_test;

use crate::{
    errors,
    program::{Program, expressions::Expr},
};
use std::error::Error;

impl Program {
    pub fn compile_properties(&mut self) -> Result<(), Box<dyn Error>> {
        self.environment = Some(
            self.expressions
                .iter()
                .map(|span_expr| &span_expr.expr)
                .map(|ltl_expr| match ltl_expr {
                    Expr::Always {
                        interval: _,
                        expr,
                        not: false,
                    }
                    | Expr::Eventually {
                        interval: _,
                        expr,
                        not: false,
                    } => Ok((expr.compile_expression()?).into()),
                    _ => Err(errors::Error::InvalidCompileExpr.into()),
                })
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?,
        );
        Ok(())
    }

    pub fn compute_iotstream_len(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(streams) = &self.environment {
            self.iotstream_len = streams
                .iter()
                .map(Expr::stream_max_bound)
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?
                .into_iter()
                .max()
                //If the value is 0, then it should be set to 1
                .map(|v| v.max(1));
        }
        Ok(())
    }
}
