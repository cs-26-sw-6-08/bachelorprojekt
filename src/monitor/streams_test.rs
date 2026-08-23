
use std::{collections::HashSet, vec};

use crate::{
    monitor_setup::operation_types::DerivedStream, program::{member_types::MemberType, operations::{BinaryOperators, UnaryOperators}}, utils::test_helper_func::*,
    
};


#[test]
/// Prop: [] 0;
/// Vaiolation: each time
fn always_false() {
    let repeats = 100i128; 

    let operations: Vec<DerivedStream> = vec![DerivedStream::Number(0)];
    let program = always_prop_helper(operations);
    let device_stream = mock_default_device_stream(repeats as usize);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, repeats);
    
    let errors = create_error_set((0..repeats as usize).into_iter(), 1);
    validate_run(result, errors );
}

#[test]
/// Prop: [] 1;
/// Vaiolation: never
fn always_true() {
    let repeats = 100i128; 

    let operations: Vec<DerivedStream> = vec![DerivedStream::Number(1)];
    let program = always_prop_helper(operations);
    let device_stream = mock_default_device_stream(repeats as usize);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, repeats);
    
    validate_run(result, HashSet::new());
}


#[test]
/// Prop: [] t%2;
/// Vaiolation: ever other
fn always_t_mod_switch() {
    let repeats = 100i128; 

    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Mod,
            idx_lhs: 1,
            idx_rhs: 2,
        },
        DerivedStream::SpawnTime,
        DerivedStream::Number(2000),
    ];
    let program = always_prop_helper(operations);
    let device_stream = mock_default_device_stream(repeats as usize);
    let streams = &mut program.environment.unwrap();

    let result = run_monitor_x_steps(streams, &device_stream, repeats);
   
    let errors = create_error_set((0..(repeats as usize)).skip(2), 1);
    validate_run(result, errors);
}

#[test]
/// Prop: [] sum(1) = 10; 
fn always_simple_count_true() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        DerivedStream::Sum {
            idx: 2
        },
        DerivedStream::Number(1000),
        DerivedStream::Number(10000),
    ];
    let program = always_prop_helper(operations);
    let device_stream = mock_specific_device_amount_stream(10, 100);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, 100);
    validate_run(result, HashSet::new());
}

#[test]
/// Prop: [] sum(1) == 1;
/// Vaiolation: each time
fn always_simple_count_false() {
    let repeats = 100i128;

    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        DerivedStream::Sum {
            idx: 2,
        },
        DerivedStream::Number(1000),
        DerivedStream::Number(1000),
    ];
    let program = always_prop_helper(operations);
    let device_stream = mock_specific_device_amount_stream(10, repeats as usize);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, repeats);

    let errors = create_error_set((0..repeats as usize).into_iter(), 1);
    validate_run(result, errors);
}

#[test]
/// Prop: [] sum(power) == 5;
/// Vaiolation: never
fn always_simple_sum_member_true() {
    let repeats = 10i128;

    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        DerivedStream::Sum {
            idx: 2,
        },
        DerivedStream::Member(MemberType::Power),
        DerivedStream::Number(5000),
    ];
    let program = always_prop_helper(operations);
    let device_stream = mock_specific_device_amount_stream(1, repeats as usize);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, repeats);

    validate_run(result, HashSet::new());
}
//
// #[test]
// /// Prop: [] Sum(power) == 55
// /// Violation: Never
// fn always_simple_sum_member_true2() {
//     let operations: Vec<DerivedStream> = vec![
//         Operation::Binary {
//             bin_op: BinaryOperators::Equal,
//             idx_lhs: 1,
//             idx_rhs: 3,
//         },
//         Operation::AggregateFunction {
//             idx: 2,
//             function_type: AggregateType::Sum,
//         },
//         Operation::Member(MemberType::Power),
//         Operation::Number(55_000),
//     ];
//     let mut program = always_prop_helper(operations, None);
//     let device_stream = ten_device_stream();
//     let Some(streams) = &mut program.environment else {
//         panic!()
//     };
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
//
//     for (_, value) in result {
//         assert!(value.is_empty());
//     }
// }
//
// #[test]
// /// Sum(Power) == 10
// /// Violation: Always 
// fn always_simple_sum_member_false() {
//     let operations: Vec<DerivedStream> = vec![
//         Operation::Binary {
//             bin_op: BinaryOperators::Equal,
//             idx_lhs: 1,
//             idx_rhs: 3,
//         },
//         Operation::AggregateFunction {
//             idx: 2,
//             function_type: AggregateType::Sum,
//         },
//         Operation::Member(MemberType::Power),
//         // Operation::Number(1_000),
//         Operation::Number(10_000),
//     ];
//     let mut program = always_prop_helper(operations, None);
//     let device_stream = ten_device_stream();
//     let Some(streams) = &mut program.environment else {
//         panic!()
//     };
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 1);
//
//     for (_, value) in result {
//         assert!(value[0].1);
//     }
// }
//
//
// #[test]
// ///Remove
// fn always_simple_avg_member_true() {
//     let operations: Vec<DerivedStream> = vec![
//         Operation::Binary {
//             bin_op: BinaryOperators::Equal,
//             idx_lhs: 1,
//             idx_rhs: 3,
//         },
//         Operation::AggregateFunction {
//             idx: 2,
//             function_type: AggregateType::Avg,
//         },
//         Operation::Member(MemberType::Power),
//         Operation::Number(5500),
//     ];
//     let mut program = always_prop_helper(operations, None);
//     let device_stream = ten_device_stream();
//     let Some(streams) = &mut program.environment else {
//         panic!()
//     };
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
//
//     for (_, value) in result {
//         assert!(value.is_empty());
//     }
// }
//
// #[test]
// /// Prop: [] t * 1 = t;
// /// Violation: Never
// fn always_mul_check() {
//     let operations: Vec<DerivedStream> = vec![
//         Operation::Binary {
//             bin_op: BinaryOperators::Equal,
//             idx_lhs: 1,
//             idx_rhs: 4,
//         },
//         Operation::Binary {
//             bin_op: BinaryOperators::Times,
//             idx_lhs: 2,
//             idx_rhs: 3,
//         },
//         Operation::SpawnTime,
//         Operation::Number(1000),
//         Operation::SpawnTime
//     ];
//     let mut program = always_prop_helper(operations, None);
//     let device_stream = ten_device_stream();
//     let Some(streams) = &mut program.environment else {
//         panic!()
//     };
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 5);
//
//     for (_, value) in result {
//         assert!(value.is_empty());
//     }
// }
//
// #[test]
// /// Sum(power) / 2 == 2.5
// /// Violates: Never
// fn always_div_check() {
//     let operations: Vec<DerivedStream> = vec![
//         Operation::Binary {
//             bin_op: BinaryOperators::Equal,
//             idx_lhs: 1,
//             idx_rhs: 5,
//         },
//         Operation::Binary {
//             bin_op: BinaryOperators::Divide,
//             idx_lhs: 2,
//             idx_rhs: 4,
//         },
//         Operation::AggregateFunction { idx: 3, function_type: AggregateType::Sum },
//         Operation::Member(MemberType::Power),
//         Operation::Number(2_000),
//         Operation::Number(2_500)
//     ];
//     let mut program = always_prop_helper(operations, None);
//     let device_stream = single_device_stream();
//     let Some(streams) = &mut program.environment else {
//         panic!()
//     };
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 5);
//
//     for (_, value) in result {
//         assert!(value.is_empty());
//     }
// }
//
// #[test]
// /// [] 2 - 1 == 1
// /// violates: Never
// fn always_minus_check() {
//     let operations: Vec<DerivedStream> = vec![
//         Operation::Binary {
//             bin_op: BinaryOperators::Equal,
//             idx_lhs: 1,
//             idx_rhs: 4,
//         },
//         Operation::Binary {
//             bin_op: BinaryOperators::Minus,
//             idx_lhs: 2,
//             idx_rhs: 3,
//         },
//         Operation::Number(2_000),
//         Operation::Number(1_000),
//         Operation::Number(1_000)
//     ];
//     let mut program = always_prop_helper(operations, None);
//     let device_stream = single_device_stream();
//     let Some(streams) = &mut program.environment else {
//         panic!()
//     };
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 5);
//
//     for (_, value) in result {
//         assert!(value.is_empty());
//     }
// }
//
//
// #[test]
// /// [] Sum(Foreach(1) != 0) == 10
// /// Violates: Never
// fn always_nested_device_stack() {
//     let operations: Vec<DerivedStream> = vec![
//         Operation::Binary {
//             bin_op: BinaryOperators::Equal,
//             idx_lhs: 1,
//             idx_rhs: 6,
//         },
//         Operation::AggregateFunction {
//             idx: 2,
//             function_type: AggregateType::Sum,
//         },
//         Operation::Binary {
//             bin_op: BinaryOperators::NotEqual,
//             idx_lhs: 3,
//             idx_rhs: 5,
//         },
//         Operation::Foreach { idx: 4 },
//         Operation::Number(1_000),
//         Operation::Number(0),
//         Operation::Number(10_000),
//     ];
//     let mut program = always_prop_helper(operations, None);
//     let device_stream = ten_device_stream();
//     let Some(streams) = &mut program.environment else {
//         panic!()
//     };
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
//
//     for (_, value) in result {
//         assert!(value.is_empty());
//     }
// }
//
//
// #[test]
// /// Sum(foreach(1) != 1) == 1
// /// Violates: Always
// fn always_nested_device_stack_false() {
//     let operations: Vec<DerivedStream> = vec![
//         Operation::Binary {
//             bin_op: BinaryOperators::Equal,
//             idx_lhs: 1,
//             idx_rhs: 6,
//         },
//         Operation::AggregateFunction {
//             idx: 2,
//             function_type: AggregateType::Sum,
//         },
//         Operation::Binary {
//             bin_op: BinaryOperators::NotEqual,
//             idx_lhs: 3,
//             idx_rhs: 5,
//         },
//         Operation::Foreach { idx: 4 },
//         Operation::Number(1_000),
//         Operation::Number(0),
//         Operation::Number(1_000),
//     ];
//     let mut program = always_prop_helper(operations, None);
//     let device_stream = ten_device_stream();
//     let Some(streams) = &mut program.environment else {
//         panic!()
//     };
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
//
//     for (_, value) in result {
//         assert!(value[0].1);
//     }
// }
//
// #[test]
// /// Prop: [] t%24s = 0S -> always[0,24] sumtime[24](1) < 24s;
// /// Violation: Look at the test data
// fn time_behaviour_test() {
//     let operations = { 
//         use DerivedStream::*;
//         vec![
//             Binary { bin_op: BinaryOperators::Or, idx_lhs: 1, idx_rhs: 7, },
//             Unary { un_op: UnaryOperators::Not, idx: 2, },
//             Binary { bin_op: BinaryOperators::Equal, idx_lhs: 3, idx_rhs: 6, },
//             Binary { bin_op: BinaryOperators::Mod, idx_lhs: 4, idx_rhs: 5, },
//             SpawnTime,
//             Number( 24000, ),
//             Number( 0, ),
//             LTLBounded { bound: ( 0, 24, ), idx: 8, not: false, ltl_type: ExprLTL::Always, },
//             Binary { bin_op: BinaryOperators::LessEqual, idx_lhs: 9, idx_rhs: 12, },
//             TimeFunction {
//                 idx: 10,
//                 function_type: AggregateType::Sum,
//                 history: Vec::new(),
//                 bound: 24,
//             },
//             AggregateFunction {
//                 idx: 11,
//                 function_type: AggregateType::Sum,
//             },
//             Number(1000,),
//             Number(24000,),
//         ] 
//     };
//     // Always t%24s = 0S -> always[0,24] sumtime(1) < 24s;
//     let program = always_prop_helper(operations, None);
//     let device_stream = single_device_stream();
//     let streams = &mut program.environment.unwrap();
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
//     for (idx, value) in result {
//         if idx == 24 || idx == 48 || idx == 72 || idx == 96{
//             assert!(value[0].1);
//         } else {
//             assert!(value.is_empty());
//         }
//     }
//     // Test for violation at 23_000
//     //Reset the property and set number as 23
//     streams[0].operations[12] = DerivedStream::Number(23_000);
//     streams[0].operations[9] = DerivedStream::TimeFunction { idx: 10, function_type: AggregateType::Sum, history: Vec::new(), bound: 24,};
//     streams[0].time_verdicts.clear();
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
//     for (idx, value) in result {
//         if idx == 23 || idx == 47 || idx == 71 || idx == 95{
//             assert!(value[0].1);
//         } else {
//             assert!(value.is_empty());
//         }
//     }
//
//     // Test no violation
//     streams[0].operations[12] = DerivedStream::Number(25_000);
//     streams[0].operations[9] = DerivedStream::TimeFunction { idx: 10, function_type: AggregateType::Sum, history: Vec::new(), bound: 24,};
//     streams[0].time_verdicts.clear();
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
//
//     for (_, value) in result { assert!(value.is_empty()) }
//
// }
//
//
// #[test]
// /// [] <>[1,1] 1
// /// Violates: Never
// fn eventually_expr_true() {
//     let operations = vec![
//         Operation::LTLBounded { bound: (1,1), idx: 1, not: false, ltl_type: ExprLTL::Eventually(Vec::new()) },
//         Operation::Number(1_000)
//     ];
//     let program = always_prop_helper(operations, None);
//     let device_stream = single_device_stream();
//     let streams = &mut program.environment.unwrap();
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
//
//     for (_, value) in result {
//         assert!(value.is_empty());
//     }
// }
//
// #[test]
// /// [] <>[1,1] 0
// /// Violates: false once, then true always
// fn eventually_expr_false() {
//     let operations = vec![
//             Operation::LTLBounded { bound: (1,1), idx: 1, not: false, ltl_type: ExprLTL::Eventually(Vec::new()) },
//             Operation::Number(0)
//         ] ;
//     let program = always_prop_helper(operations, None);
//     let device_stream = single_device_stream();
//     let streams = &mut program.environment.unwrap();
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
//
//     for (idx, value) in result {
//         if idx == 0{
//             assert!(value.is_empty());
//         } else {
//             assert!(value[0].1);
//         }
//     }
// }
//
// #[test]
// /// Prop: [] <>[2,5] t = 2;
// /// Violation: Never
// fn eventually_expr_time_true() {
//     let operations = {
//         use DerivedStream::*;
//         vec![
//             LTLBounded { bound: ( 2, 5 ), idx: 1, not: false, ltl_type: ExprLTL::Eventually(Vec::new())},
//             Binary {bin_op: BinaryOperators::NotEqual, idx_lhs: 2,idx_rhs: 3},
//             SpawnTime,
//             Number(2_000),
//         ]};
//
//     let program = always_prop_helper(operations, None);
//     let device_stream = single_device_stream();
//     let streams = &mut program.environment.unwrap();
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
//
//     for idx in (0..100).filter(|&num| !(2..5).contains(&num) ) {
//         let value = result.get(&idx).unwrap();
//         assert!(value.is_empty());
//     }
// }
//
// #[test]
// /// Prop: always (t = 0) -> (always[6,6](always[9,9]0));
// /// Violation: Gives vilolation at 15, and only 15
// fn always_always_always(){
//     let operations = {
//         use DerivedStream::*;
//         vec![
//             Binary {
//                     bin_op: BinaryOperators::Or,
//                     idx_lhs: 1,
//                     idx_rhs: 5,
//                 },
//                 Unary {
//                     un_op: UnaryOperators::Not,
//                     idx: 2,
//                 },
//                 Binary {
//                     bin_op: BinaryOperators::Equal,
//                     idx_lhs: 3,
//                     idx_rhs: 4,
//                 },
//                 SpawnTime,
//                 Number(
//                     0,
//                 ),
//                 LTLBounded {
//                     bound: (
//                         6,
//                         6,
//                     ),
//                     idx: 6,
//                     not: false,
//                     ltl_type: ExprLTL::Always,
//                 },
//                 LTLBounded {
//                     bound: (
//                         9,
//                         9,
//                     ),
//                     idx: 7,
//                     not: false,
//                     ltl_type: ExprLTL::Always,
//                 },
//                 Number(
//                     0,
//                 ),
//         ]};
//
//     let program = always_prop_helper(operations, None);
//     let device_stream = single_device_stream();
//     let streams = &mut program.environment.unwrap();
//     let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
//
//     for (idx, value) in result {
//         if idx != 15{
//             assert!(value.is_empty());
//         } else {
//             assert!(value[0].1);
//         }
//     }
// }
