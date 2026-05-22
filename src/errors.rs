use crate::{program::{expressions::Expr, operations::BinaryOperators}, unit_check::types::Type};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("hime parse error")]
    HimeParse,
    #[error("ast node invalid: {0}")]
    ASTNodeValueInvalid(String),
    #[error("program parse error line {1} column {2}: {0}")]
    ProgramParse(String, usize, usize),
    #[error("typechecking failed at expr: {0}, got type: {1}")]
    IncorrectType(Expr, Type),
    #[error("typechecking failed at expr: {0}, got types: {1} and {2}")]
    IncorrectTwoTypes(Expr, Type, Type),
    #[error("binary operation failed: {0}, lhs: {1}, rhs: {2}")]
    BinaryOperationFail(BinaryOperators, i128, i128),
    #[error("interval value below zero start: {0} end: {1}")]
    IntervalBelowZero(i128, i128),
    #[error("interval start value: {0} is greater than end value: {1}")]
    IntervalStartGreaterThanEnd(i128, i128),
    #[error("Property is not monitorable: {0}")]
    Unmonitorable(Expr),
    #[error("Property at line {0} is not monitorable")]
    UnmonitorableLine(usize),
    #[error("Only the foreach with an non-temporal aggregate function or always/eventually is allowed")]
    OnlyForeachTemporalExpressionAllowed,

    #[error("Invalid MIITL Interval Expression")]
    InvalidIntervalExpr,
    #[error("Invalid Expression for compilation")]
    InvalidCompileExpr,
    #[error("Invalid Function Interval Expression")]
    InvalidFunctionIntervalExpr,

    #[error("Value Stack error: Not enough values in stack")]
    ValueStackPop,
    #[error("Invalid Device Pointer Assignment")]
    DevicePointer,
    #[error("Value Stack Error: Not the correct value type")]
    ValueStackVal,
    #[error("Illegal operation during evaluation")]
    IllegalOperation,
    #[error("The environment have not been set for the program")]
    EnvironmentNotPresent
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ASTNodeValueInvalid(l0), Self::ASTNodeValueInvalid(r0)) => l0 == r0,
            (Self::ProgramParse(l0, l1, l2), Self::ProgramParse(r0, r1, r2)) => l0 == r0 && l1 == r1 && l2 == r2,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
