use crate::errors;
use crate::monitor::streams::OutputStream;
use crate::monitor::types::StepType;
use crate::utils::vec_helper_funcs::ExtVec;
use crate::{
    monitor_setup::operation_types::{DerivedStream, MIITLType},
    program::{expressions::Expr, function_types::FunctionType, operations::BinaryOperators::*},
};
use std::error::Error;

impl Expr {
    pub fn compile_expression(&self) -> Result<Vec<DerivedStream>, Box<dyn Error>> {
        self.compile_expression_helper(Vec::new(), 0)
            .map(|res| res.0)
    }

    fn compile_expression_helper(
        &self,
        streams: Vec<DerivedStream>,
        key: usize,
    ) -> Result<(Vec<DerivedStream>, usize), Box<dyn Error>> {
        Ok(match self {
            Expr::Number(c) => (streams.with(DerivedStream::Number(*c)), key + 1),
            Expr::String(str) => (streams.with(DerivedStream::String(str.to_owned())), key + 1),
            Expr::CurrentTime => (streams.with(DerivedStream::SpawnTime), key + 1),
            Expr::Member { access_type } => (
                streams.with(DerivedStream::Member(access_type.clone())),
                key + 1,
            ),
            Expr::Always {
                interval: Some(val),
                not: _,
                expr,
            }
            | Expr::Eventually {
                interval: Some(val),
                not: _,
                expr,
            } => {
                let (new_streams, new_key) = expr.compile_expression_helper(Vec::new(), key + 1)?;
                (
                    streams
                        .with(DerivedStream::Miitl {
                            bound: val.get_bound().map(|(a, b)| (a / 1000, b / 1000))?,
                            idx: key + 1,
                            miitl_type: match self {
                                Expr::Always { .. } => MIITLType::Always,
                                Expr::Eventually { .. } => MIITLType::Eventually,
                                _ => unreachable!()
                            },
                        })
                        .chain(new_streams),
                    new_key,
                )
            }
            Expr::BinaryOperations { lhs, rhs, operator } => {
                let (new_1_streams, new_1_key) =
                    lhs.compile_expression_helper(Vec::new(), key + 1)?;
                let (new_2_streams, new_2_key) =
                    rhs.compile_expression_helper(Vec::new(), new_1_key)?;

                (
                    streams
                        .with(DerivedStream::Binary {
                            bin_op: match operator {
                                And | Implies => Err(errors::Error::InvalidCompileExpr),
                                val => Ok(val.clone()),
                            }?,
                            idx_lhs: key + 1,
                            idx_rhs: new_1_key,
                        })
                        .chain(new_1_streams)
                        .chain(new_2_streams),
                    new_2_key,
                )
            }
            Expr::UnaryOperations { operand, operator } => {
                let (new_streams, new_key) =
                    operand.compile_expression_helper(Vec::new(), key + 1)?;
                (
                    streams
                        .with(DerivedStream::Unary {
                            un_op: operator.clone(),
                            idx: key + 1,
                        })
                        .chain(new_streams),
                    new_key,
                )
            }
            Expr::Function {
                aggregate_type,
                expr,
                bound,
            } => match aggregate_type {
                FunctionType::Foreach => {
                    let (new_streams, new_key) =
                        expr.compile_expression_helper(Vec::new(), key + 1)?;
                    (
                        streams
                            .with(DerivedStream::Foreach { idx: key + 1 })
                            .chain(new_streams),
                        new_key,
                    )
                }
                //todo: Check the logic here!!!! 
                // Avg should be sum(e) / size
                FunctionType::Avg => {
                    let (new_streams, new_key) =
                        expr.compile_expression_helper(Vec::new(), key + 2)?;
                    (
                        streams
                            .with(DerivedStream::Binary {
                                bin_op: Divide,
                                idx_lhs: key + 1,
                                idx_rhs: new_key,
                            })
                            .with(DerivedStream::Sum { idx: key + 2 })
                            .chain(new_streams)
                            .with(DerivedStream::Size),
                        new_key + 1,
                    )
                }
                FunctionType::Sum => {
                    let (new_streams, new_key) =
                        expr.compile_expression_helper(Vec::new(), key + 1)?;
                    (
                        streams
                            .with(DerivedStream::Sum { idx: key + 1 })
                            .chain(new_streams),
                        new_key,
                    )
                }
                FunctionType::Sumtime => {
                    let wrap_function = Expr::Function {
                        aggregate_type: FunctionType::Sum,
                        expr: expr.clone(),
                        bound: None,
                    };
                    let (new_streams, new_key) =
                        wrap_function.compile_expression_helper(Vec::new(), key + 1)?;
                    let Some(bound) = bound else {
                        return Err(errors::Error::InvalidFunctionIntervalExpr.into());
                    };
                    (
                        streams
                            .with(DerivedStream::Sumtime {
                                idx: key + 1,
                                interval_len: bound.get_bound_time_function().map(|b| b / 1000)?,
                            })
                            .chain(new_streams),
                        new_key,
                    )
                }
                FunctionType::Avgtime => {
                    let wrap_function = Expr::Function {
                        aggregate_type: FunctionType::Sum,
                        expr: expr.clone(),
                        bound: None,
                    };
                    let (new_streams, new_key) =
                        wrap_function.compile_expression_helper(Vec::new(), key + 2)?;
                    let Some(bound) = bound else {
                        return Err(errors::Error::InvalidFunctionIntervalExpr.into());
                    };
                    let bound = bound.get_bound_time_function().map(|b| b / 1000)?;
                    (
                        streams
                            .with(DerivedStream::Binary { bin_op: Divide, idx_lhs: key + 1, idx_rhs: new_key })
                            .with(DerivedStream::Sumtime {
                                idx: key + 2,
                                interval_len: bound,
                            })
                            .chain(new_streams)
                            .with(DerivedStream::Number(bound + 1)),
                        new_key + 1,
                    )
                }
                //todo: check if the frontend removes avgtime --> Else this needs to be changed
                _ => Err(errors::Error::InvalidCompileExpr)?,
            },
            Expr::Interval { .. }
            | Expr::Unit { .. }
            | Expr::Always { interval: None, .. }
            | Expr::Eventually { interval: None, .. } => Err(errors::Error::InvalidCompileExpr)?,
        })
    }

    pub fn stream_max_bound(stream: &OutputStream) -> Result<usize, Box<dyn Error>> {
        use StepType::*;

        let mut idx_stack = vec![(0usize, Deepen)];
        let mut size_stack: Vec<usize> = vec![0usize];

        while let Some((cur_idx, step)) = idx_stack.pop() {
            let cur_stream = &stream.derived_streams[cur_idx];
            match (cur_stream, step) {
                (DerivedStream::Number(_), Deepen) |
                (DerivedStream::String(_), Deepen) |
                (DerivedStream::Member(_), Deepen) |
                (DerivedStream::SpawnTime, Deepen) |
                (DerivedStream::Size, Deepen) => (),

                (DerivedStream::Sum { idx }, Deepen)
                | (DerivedStream::Foreach { idx }, Deepen)
                | (DerivedStream::Unary { idx, .. }, Deepen)  => idx_stack.push((*idx, Deepen)),

                (DerivedStream::Sumtime { idx , .. }, Deepen) => {
                    idx_stack.push((cur_idx, Reduce));
                    idx_stack.push((*idx, Deepen));
                },
                (DerivedStream::Sumtime { interval_len, .. }, Reduce) => {
                    let size = size_stack.pop_or_err()?;
                    size_stack.push(size + (*interval_len as usize));
                },
                (DerivedStream::Binary { idx_lhs, idx_rhs , .. }, Deepen) => {
                    let size = size_stack.pop_or_err()?;
                    size_stack.push(size);
                    size_stack.push(size);

                    idx_stack.push((cur_idx, Reduce));
                    idx_stack.push((*idx_rhs, Deepen));
                    idx_stack.push((*idx_lhs, Deepen));
                },
                (DerivedStream::Binary { .. }, Reduce) => {
                    let rhs = size_stack.pop_or_err()?;
                    let lhs = size_stack.pop_or_err()?;
                    size_stack.push(rhs.max(lhs));
                },
                (DerivedStream::Miitl { idx , .. }, Deepen) => {
                    idx_stack.push((cur_idx, Reduce));
                    idx_stack.push((*idx, Deepen));
                },
                (DerivedStream::Miitl { bound,  .. }, Reduce) => {
                    let (_, b) = bound;
                    let size = size_stack.pop_or_err()?;
                    size_stack.push(size + (*b as usize));
                },
                _ => unreachable!()
            }
        }
        size_stack.pop_or_err()
    }

}
