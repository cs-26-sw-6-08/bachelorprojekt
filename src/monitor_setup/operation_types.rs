
use crate::{program::{member_types::MemberType, operations::{BinaryOperators, UnaryOperators}}};


#[derive(Debug, PartialEq, Clone)]
pub enum DerivedStream {
    Number(i128),
    String(String),
    Member(MemberType),

    SpawnTime,
    Size,

    Sum { idx: usize }, 
    Sumtime { interval_len: i128, idx: usize },
    Foreach { idx: usize },

    Binary { bin_op: BinaryOperators, idx_lhs: usize, idx_rhs: usize },
    Unary { un_op: UnaryOperators, idx: usize },

    Miitl { miitl_type: MIITLType, bound: (i128, i128), idx: usize }
}


#[derive(Debug, PartialEq, Clone)]
pub enum MIITLType { Always, Eventually }