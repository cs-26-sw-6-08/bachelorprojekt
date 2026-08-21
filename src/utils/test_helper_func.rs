use crate::{
    monitor::streams::IoTDevice,
    monitor_setup::operation_types::DerivedStream,
    program::{
        expressions::Expr,
        function_types::FunctionType,
        member_types::MemberType,
        operations::{BinaryOperators, UnaryOperators},
        units::Unit,
    },
    utils::vec_helper_funcs::ExtVec,
};

use std::{collections::HashSet, ops::Range};

use crate::{
    monitor::streams::{IoTStream, OutputStream},
    program::Program,
};

type Violations = Vec<Vec<bool>>;

pub fn mock_default_device_stream(repeats: usize) -> IoTStream {
    mock_device_stream(
        vec![
            ("Roomba".into(), 5_000).into(),
            ("Light".into(), 1_000).into(),
            ("Fridge".into(), 3_000).into(),
        ],
        repeats,
    )
}

pub fn mock_specific_device_amount_stream(amt: usize, repeats: usize) -> IoTStream {
    mock_device_stream(
        vec![("Roomba".into(), 5_000).into()]
            .into_iter()
            .cycle()
            .take(amt)
            .collect(),
        repeats,
    )
}

pub fn mock_device_stream(devices: Vec<IoTDevice>, repeats: usize) -> IoTStream {
    vec![devices]
        .into_iter()
        .cycle()
        .take(repeats)
        .collect::<Vec<_>>()
        .into()
}

pub fn combine_device_streams(stream1: IoTStream, stream2: IoTStream, repeats: usize) -> IoTStream {
    let d_stream = stream1
        .get_all_own()
        .into_iter()
        .chain(stream2.get_all_own().into_iter())
        .cycle()
        .take(repeats)
        .collect::<Vec<_>>();

    d_stream.into()
}

/**
@return Timepoint < Prop Num < Violations (true if violation occurred, false if not) > >
*/
pub fn run_monitor_x_steps<'a>(
    env: &'a mut [OutputStream],
    device_stream: &'a IoTStream,
    step_amount: i128,
) -> Violations {
    (0..step_amount).into_iter().fold(Vec::new(), |acc, val| {
        let cur_result = Program::monitor_logic(env, &val, device_stream)
            .map(|res| res.unwrap().1) //Returns the is_violated variable
            .collect::<Vec<_>>();
        acc.with(cur_result)
    })
}

pub fn create_error_set(
    steps: impl Iterator<Item = usize>,
    props_amt: usize,
) -> HashSet<(usize, usize)> {
    steps
        .map(|i| (0..props_amt).map(move |j| (i, j)))
        .flatten()
        .collect::<HashSet<_>>()
}

pub fn validate_run(to_be_validated: Violations, violation_timepoints: HashSet<(usize, usize)>) {
    for (i, prop_num) in to_be_validated.iter().enumerate() {
        for (j, &device_violation) in prop_num.iter().enumerate() {
            //if device_violation true then violation occurred and violation_timepoints should contain the indices
            assert_eq!(device_violation, violation_timepoints.contains(&(i, j)))
        }
    }
}

pub fn print_run(to_be_printed: Violations) {
    todo!()
}

pub fn binary_expr(lhs: Expr, rhs: Expr, operator: BinaryOperators) -> Expr {
    Expr::BinaryOperations {
        lhs: lhs.into(),
        rhs: rhs.into(),
        operator,
    }
}

pub fn unary_expr(operand: Expr, operator: UnaryOperators) -> Expr {
    Expr::UnaryOperations {
        operand: operand.into(),
        operator,
    }
}

pub fn number_expr() -> Expr {
    Expr::Number(5000)
}

pub fn custom_number_expr(n: i128) -> Expr {
    Expr::Number(n)
}

pub fn string_expr() -> Expr {
    Expr::String("christian".into())
}

pub fn current_time() -> Expr {
    Expr::CurrentTime
}

pub fn unit_expr(unit: Unit) -> Expr {
    Expr::Unit {
        number: number_expr().into(),
        unit,
    }
}

pub fn custom_unit_expr(number: i128, unit: Unit) -> Expr {
    Expr::Unit {
        number: custom_number_expr(number).into(),
        unit,
    }
}

pub fn member_expr(access_type: MemberType) -> Expr {
    Expr::Member { access_type }
}

pub fn function_expr(aggregate_type: FunctionType, expr: Expr, bound: Option<Expr>) -> Expr {
    Expr::Function {
        aggregate_type,
        expr: expr.into(),
        bound: bound.map(|v| v.into()),
    }
}

pub fn interval_expr(unit1: Expr, unit2: Expr) -> Expr {
    Expr::Interval {
        start: unit1.into(),
        end: unit2.into(),
    }
}

pub fn always_expr(expr: Expr) -> Expr {
    Expr::Always {
        interval: None,
        not: false,
        expr: expr.into(),
    }
}

pub fn always_negated_expr(expr: Expr) -> Expr {
    Expr::Always {
        interval: None,
        not: true,
        expr: expr.into(),
    }
}

pub fn always_interval_expr(interval: Expr, expr: Expr) -> Expr {
    Expr::Always {
        interval: Some(interval.into()),
        not: false,
        expr: expr.into(),
    }
}

pub fn eventually_expr(expr: Expr) -> Expr {
    Expr::Eventually {
        interval: None,
        not: false,
        expr: expr.into(),
    }
}

pub fn eventually_negated_expr(expr: Expr) -> Expr {
    Expr::Eventually {
        interval: None,
        not: true,
        expr: expr.into(),
    }
}

pub fn eventually_interval_expr(interval: Expr, expr: Expr) -> Expr {
    Expr::Eventually {
        interval: Some(interval.into()),
        not: false,
        expr: expr.into(),
    }
}

//always[25s, 40s] sumtime(power) < always[500, 1000] sumtime (1)
// pub fn operations_vec_with_sumtime() -> Vec<DerivedStream> {
//     [
//         DerivedStream::Binary {
//             bin_op: BinaryOperators::Less,
//             idx_lhs: 1,
//             idx_rhs: 5,
//         },
//         DerivedStream::LTLBounded {
//             bound: (25, 40),
//             idx: 2,
//             not: false,
//             ltl_type: ExprLTL::Always,
//         },
//         DerivedStream::TimeFunction {
//             idx: 3,
//             function_type: AggregateType::Sum,
//             history: Vec::new(),
//             bound: 1000,
//         },
//         DerivedStream::AggregateFunction {
//             idx: 4,
//             function_type: AggregateType::Sum,
//         },
//         DerivedStream::Member(MemberType::Power),
//         DerivedStream::LTLBounded {
//             bound: (500, 1000),
//             idx: 6,
//             not: false,
//             ltl_type: ExprLTL::Always,
//         },
//         DerivedStream::TimeFunction {
//             idx: 7,
//             function_type: AggregateType::Sum,
//             history: Vec::new(),
//             bound: 1000,
//         },
//         DerivedStream::AggregateFunction {
//             idx: 8,
//             function_type: AggregateType::Sum,
//         },
//         DerivedStream::Number(1),
//     ]
//     .into()
// }

pub fn always_prop_helper(derived_streams: Vec<DerivedStream>) -> Program {
    Program {
        expressions: vec![],
        environment: Some(vec![OutputStream {
            unresolved_timepoints: vec![],
            derived_streams,
        }]),
    }
}
