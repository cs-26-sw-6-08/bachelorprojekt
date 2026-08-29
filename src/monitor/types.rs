use std::{
    error::Error,
    ops::{Add, Sub},
};

use crate::{
    errors,
    monitor::{
        streams::IoTDevice,
        types::StackElement::{Element, LayerShift},
    },
    program::operations::{BinaryOperators, UnaryOperators},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Verdict {
    True,
    False,
    Undecided,
}

// impl Verdict {
//     pub fn to_bool(&self) -> bool {
//         match self {
//             Verdict::True => true,
//             Verdict::False | Verdict::Undecided => false,
//         }
//     }

//     pub fn and(self, rhs: Self) -> Self {
//         match (self, rhs) {
//             (Verdict::True, Verdict::True) => Verdict::True,
//             (_, Verdict::False) => Verdict::False,
//             (Verdict::False, _) => Verdict::False,
//             (Verdict::Undecided, _) => Verdict::Undecided,
//             (_, Verdict::Undecided) => Verdict::Undecided,
//         }
//     }

//     pub fn or(self, rhs: Self) -> Self {
//         match (self, rhs) {
//             (Verdict::False, Verdict::False) => Verdict::False,
//             (_, Verdict::True) => Verdict::True,
//             (Verdict::True, _) => Verdict::True,
//             (Verdict::Undecided, _) => Verdict::Undecided,
//             (_, Verdict::Undecided) => Verdict::Undecided,
//         }
//     }
// }

// impl Not for Verdict {
//     type Output = Verdict;

//     fn not(self) -> Self::Output {
//         use Verdict::*;
//         match self {
//             True => False,
//             False => True,
//             Undecided => Undecided,
//         }
//     }
// }

// impl From<bool> for Verdict {
//     fn from(value: bool) -> Self {
//         match value {
//             true => Verdict::True,
//             false => Verdict::False,
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub struct Device {
    name: String,
    power: i128,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StreamValue<'a> {
    Number(Option<i128>),
    String(&'a str),
}

impl<'a> From<i128> for StreamValue<'a> {
    fn from(value: i128) -> Self {
        StreamValue::Number(Some(value))
    }
}

impl<'a> From<&'a String> for StreamValue<'a> {
    fn from(value: &'a String) -> Self {
        StreamValue::String(value.as_str())
    }
}

impl StreamValue<'_> {
    pub fn get_verdict(&self) -> Result<Option<bool>, Box<dyn Error>> {
        match self {
            StreamValue::Number(v) => {
                if let Some(v) = v {
                    Ok(Some(*v != 0))
                } else {
                    Ok(None)
                }
            }
            _ => Err(errors::Error::ValueStackVal.into()),
        }
    }

    pub fn get_num(&self) -> Result<Option<i128>, Box<dyn Error>> {
        match self {
            StreamValue::Number(v) => Ok(*v),
            _ => Err(errors::Error::ValueStackVal.into()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum StepType {
    Deepen,
    Reduce,
    ReducePartial,
}

#[derive(Debug)]
pub enum StackElement<T> {
    Element(T),
    LayerShift,
}

impl<'a> From<&'a IoTDevice> for StackElement<&'a IoTDevice> {
    fn from(value: &'a IoTDevice) -> Self {
        StackElement::Element(value)
    }
}

impl<T> StackElement<T> {
    pub fn unpack_element(&self) -> Result<&T, Box<dyn Error>> {
        match self {
            Element(v) => Ok(v),
            LayerShift => Err(errors::Error::WrongEnumType.into()),
        }
    }
}

impl<'a> StreamValue<'a> {
    pub fn bin_op(self, rhs: Self, bin_op: &BinaryOperators) -> Self {
        match bin_op {
            BinaryOperators::Equal => self.equals(rhs),
            BinaryOperators::NotEqual => self.not_equals(rhs),
            BinaryOperators::Less => self.less_than(rhs),
            BinaryOperators::Greater => rhs.less_than(self),
            BinaryOperators::LessEqual => self.less_equal(rhs),
            BinaryOperators::GreaterEqual => rhs.less_equal(self),
            BinaryOperators::Plus => self + rhs,
            BinaryOperators::Minus => self - rhs,
            BinaryOperators::Times => self.mul(rhs),
            BinaryOperators::Divide => self.div(rhs),
            BinaryOperators::Mod => self.modulo(rhs),
            BinaryOperators::Or => self.or(&rhs),
            BinaryOperators::And => self.and(&rhs),
            _ => unreachable!(),
        }
    }

    pub fn modulo(self, rhs: Self) -> Self {
        match (self, rhs) {
            (StreamValue::Number(Some(val1)), StreamValue::Number(Some(val2))) => {
                // Rust does not have a correct mathematical mod function, therefore this
                // calculation is used instead: https://stackoverflow.com/a/31210691
                StreamValue::Number(Some(
                    ((val1.checked_rem(val2).unwrap_or(0)) + val2)
                        .checked_rem(val2)
                        .unwrap_or(0),
                ))
            }
            (StreamValue::Number(None), StreamValue::Number(_))
            | (StreamValue::Number(_), StreamValue::Number(None)) => StreamValue::Number(None),
            _ => unreachable!(),
        }
    }

    pub fn equals(self, rhs: Self) -> Self {
        match (self, rhs) {
            (StreamValue::Number(Some(val1)), StreamValue::Number(Some(val2))) => {
                StreamValue::Number(Some((val1 == val2) as i128 * 1_000))
            }
            (StreamValue::Number(None), StreamValue::Number(_))
            | (StreamValue::Number(_), StreamValue::Number(None)) => StreamValue::Number(None),
            (StreamValue::String(val1), StreamValue::String(val2)) => {
                StreamValue::Number(Some((val1 == val2) as i128 * 1_000))
            }
            _ => unreachable!(),
        }
    }

    pub fn not_equals(self, rhs: Self) -> Self {
        match (self, rhs) {
            (StreamValue::Number(Some(val1)), StreamValue::Number(Some(val2))) => {
                StreamValue::Number(Some((val1 != val2) as i128 * 1_000))
            }
            (StreamValue::Number(None), StreamValue::Number(_))
            | (StreamValue::Number(_), StreamValue::Number(None)) => StreamValue::Number(None),
            (StreamValue::String(val1), StreamValue::String(val2)) => {
                StreamValue::Number(Some((val1 != val2) as i128 * 1_000))
            }
            _ => unreachable!(),
        }
    }

    //todo: figure out if this should be the logical and designed in the paper
    pub fn and(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (StreamValue::Number(Some(val1)), StreamValue::Number(Some(val2))) => {
                StreamValue::Number(Some(((*val1 != 0) && (*val2 != 0)) as i128 * 1_000))
            }
            (StreamValue::Number(None), StreamValue::Number(Some(v)))
            | (StreamValue::Number(Some(v)), StreamValue::Number(None)) => {
                StreamValue::Number(if *v != 0 {
                    None
                } else {
                    Some(false as i128 * 1_000)
                })
            }
            (StreamValue::Number(None), StreamValue::Number(None)) => StreamValue::Number(None),
            _ => unreachable!(),
        }
    }

    pub fn or(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (StreamValue::Number(Some(val1)), StreamValue::Number(Some(val2))) => {
                StreamValue::Number(Some(((*val1 != 0) || (*val2 != 0)) as i128 * 1_000))
            }
            (StreamValue::Number(None), StreamValue::Number(Some(v)))
            | (StreamValue::Number(Some(v)), StreamValue::Number(None)) => {
                StreamValue::Number(if *v != 0 {
                    Some(true as i128 * 1_000)
                } else {
                    None
                })
            }
            (StreamValue::Number(None), StreamValue::Number(None)) => StreamValue::Number(None),
            _ => unreachable!(),
        }
    }

    pub fn less_than(self, rhs: Self) -> Self {
        match (self, rhs) {
            (StreamValue::Number(Some(val1)), StreamValue::Number(Some(val2))) => {
                StreamValue::Number(Some((val1 < val2) as i128 * 1_000))
            }
            (StreamValue::Number(None), StreamValue::Number(_))
            | (StreamValue::Number(_), StreamValue::Number(None)) => StreamValue::Number(None),
            _ => unreachable!(),
        }
    }

    pub fn less_equal(self, rhs: Self) -> Self {
        match (self, rhs) {
            (StreamValue::Number(Some(val1)), StreamValue::Number(Some(val2))) => {
                StreamValue::Number(Some((val1 <= val2) as i128 * 1_000))
            }
            (StreamValue::Number(None), StreamValue::Number(_))
            | (StreamValue::Number(_), StreamValue::Number(None)) => StreamValue::Number(None),
            _ => unreachable!(),
        }
    }

    pub fn un_op(self, un_op: &UnaryOperators) -> Self {
        match un_op {
            UnaryOperators::Not => match self {
                StreamValue::Number(Some(v)) => StreamValue::Number(Some((v == 0) as i128 * 1_000)),
                StreamValue::Number(None) => StreamValue::Number(None),
                _ => unreachable!(),
            },
            UnaryOperators::Negative => match self {
                StreamValue::Number(Some(v)) => StreamValue::Number(Some(-v)),
                StreamValue::Number(None) => StreamValue::Number(None),
                _ => unreachable!(),
            },
        }
    }

    fn mul(self, rhs: Self) -> Self {
        use StreamValue::*;
        match (self, rhs) {
            (Number(Some(m)), Number(Some(n))) => {
                let m_int = m / 1000;
                let m_frac = m % 1000;

                let int = n * m_int;
                let frac = (n * m_frac) / 1000;

                Number(Some(int + frac))
            }
            (Number(None), Number(Some(_)))
            | (Number(Some(_)), Number(None))
            | (Number(None), Number(None)) => Number(None),
            _ => unreachable!(),
        }
    }

    fn div(self, rhs: Self) -> Self {
        use StreamValue::*;
        match (self, rhs) {
            (Number(Some(val1)), Number(Some(val2))) => {
                let m_int = val2 / 1000;
                let m_frac = val2 % 1000;

                let int = val1.checked_div(m_int).unwrap_or(0);
                let frac = val1.checked_div(m_frac).unwrap_or(0);

                Number(Some(int + frac))
            }
            (Number(None), Number(Some(_)))
            | (Number(Some(_)), Number(None))
            | (Number(None), Number(None)) => Number(None),
            _ => unreachable!(),
        }
    }
}

impl<'a> Add for StreamValue<'a> {
    type Output = StreamValue<'a>;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (StreamValue::Number(Some(val1)), StreamValue::Number(Some(val2))) => {
                StreamValue::Number(Some(val1 + val2))
            }
            (StreamValue::Number(None), StreamValue::Number(_))
            | (StreamValue::Number(_), StreamValue::Number(None)) => StreamValue::Number(None),
            _ => unreachable!(),
        }
    }
}

impl<'a> Sub for StreamValue<'a> {
    type Output = StreamValue<'a>;

    fn sub(self, rhs: Self) -> Self::Output {
        use StreamValue::*;
        match (self, rhs) {
            (Number(Some(val1)), Number(Some(val2))) => Number(Some(val1 - val2)),
            (Number(None), Number(Some(_)))
            | (Number(Some(_)), Number(None))
            | (Number(None), Number(None)) => Number(None),
            _ => unreachable!(),
        }
    }
}

// impl<'a> Div for StreamValue<'a> {
//     type Output = StreamValue<'a>;

//     fn div(mut self, rhs: Self) -> Self::Output {
//         let value = match (self.get_value(), rhs.get_value()) {
//             (StreamValue::Number(val1), StreamValue::Number(val2)) => {
//                 StreamValue::Number(val1 / val2)
//             }
//             _ => unreachable!(),
//         };

//         self.value = value;
//         self.decided = self.decided.greatest_lower_bound(&rhs.decided);
//         self
//     }
// }
