use std::{collections::HashSet, vec};

use crate::{
    monitor_setup::operation_types::{DerivedStream, MIITLType::{self, Always}}, program::{
        member_types::MemberType,
        operations::{BinaryOperators, UnaryOperators},
    }, utils::test_helper_func::*,
};

#[test]
/// Prop: [] 0;
/// Vaiolation: each time
fn always_false() {
    let repeats = 100i128;

    let operations: Vec<DerivedStream> = vec![DerivedStream::Number(0)];
    let program = program_init(operations);
    let device_stream = mock_default_device_stream(repeats as usize);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, repeats);

    let errors = create_error_set(0..repeats as usize, 1);
    validate_run(result, errors);
}

#[test]
/// Prop: [] 1;
/// Vaiolation: never
fn always_true() {
    let repeats = 100i128;

    let operations: Vec<DerivedStream> = vec![DerivedStream::Number(1)];
    let program = program_init(operations);
    let device_stream = mock_default_device_stream(repeats as usize);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, repeats);

    validate_run(result, HashSet::new());
}

#[test]
/// Prop: [] t%2;
/// Vaiolation: every other
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
    let program = program_init(operations);
    let device_stream = mock_default_device_stream(repeats as usize);
    let streams = &mut program.environment.unwrap();

    let result = run_monitor_x_steps(streams, &device_stream, repeats);
    let errors = create_error_set((0..=(repeats as usize)).step_by(2), 1);
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
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Number(1000),
        DerivedStream::Number(10000),
    ];
    let program = program_init(operations);
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
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Number(1000),
        DerivedStream::Number(1000),
    ];
    let program = program_init(operations);
    let device_stream = mock_specific_device_amount_stream(10, repeats as usize);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, repeats);

    let errors = create_error_set(0..repeats as usize, 1);
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
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Member(MemberType::Power),
        DerivedStream::Number(5000),
    ];
    let program = program_init(operations);
    let device_stream = mock_specific_device_amount_stream(1, repeats as usize);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, repeats);

    validate_run(result, HashSet::new());
}

#[test]
/// Prop: [] Sum(power) == 55
/// Violation: Never
fn always_simple_sum_member_true2() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Member(MemberType::Power),
        DerivedStream::Number(55_000),
    ];
    let program = program_init(operations);
    let device_stream = mock_specific_device_amount_stream(11, 10);
    let streams = &mut program.environment.unwrap();

    let result = run_monitor_x_steps(streams, &device_stream, 10);

    validate_run(result, HashSet::new());
}

#[test]
/// Sum(Power) == 10
/// Violation: Always
fn always_simple_sum_member_false() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Member(MemberType::Power),
        // DerivedStream::Number(1_000),
        DerivedStream::Number(10_000),
    ];
    let mut program = program_init(operations);
    let device_stream = mock_specific_device_amount_stream(10, 1);
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 1);

    let errors = create_error_set(0..1, 1);
    validate_run(result, errors);
}


#[test]
/// Prop: [] t * 1 = t;
/// Violation: Never
fn always_mul_check() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 4,
        },
        DerivedStream::Binary {
            bin_op: BinaryOperators::Times,
            idx_lhs: 2,
            idx_rhs: 3,
        },
        DerivedStream::SpawnTime,
        DerivedStream::Number(1000),
        DerivedStream::SpawnTime,
    ];
    let mut program = program_init(operations);
    let device_stream = mock_specific_device_amount_stream(10, 5);
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 5);

    validate_run(result, HashSet::new());
}

#[test]
/// Sum(power) / 2 == 2.5
/// Violates: Never
fn always_div_check() { // TODO: UPdate the mock data here
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 5,
        },
        DerivedStream::Binary {
            bin_op: BinaryOperators::Divide,
            idx_lhs: 2,
            idx_rhs: 4,
        },
        DerivedStream::Sum { idx: 3 },
        DerivedStream::Member(MemberType::Power),
        DerivedStream::Number(2_000),
        DerivedStream::Number(2_500),
    ];
    let mut program = program_init(operations);
    //power of 1 device = 5
    let device_stream = mock_specific_device_amount_stream(1, 1);
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 5);

    validate_run(result, HashSet::new());
}

#[test]
/// [] 2 - 1 == 1
/// violates: Never
fn always_minus_check() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 4,
        },
        DerivedStream::Binary {
            bin_op: BinaryOperators::Minus,
            idx_lhs: 2,
            idx_rhs: 3,
        },
        DerivedStream::Number(2_000),
        DerivedStream::Number(1_000),
        DerivedStream::Number(1_000),
    ];
    let mut program = program_init(operations);

    let device_stream = mock_default_device_stream(5);

    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 5);

    validate_run(result, HashSet::new());
}

#[test]
/// [] 2 * 4 == 8
/// violates: Never
fn always_times_check() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 4,
        },
        DerivedStream::Binary {
            bin_op: BinaryOperators::Times,
            idx_lhs: 2,
            idx_rhs: 3,
        },
        DerivedStream::Number(2_000),
        DerivedStream::Number(4_000),
        DerivedStream::Number(8_000),
    ];
    let mut program = program_init(operations);

    let device_stream = mock_default_device_stream(5);

    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 5);

    validate_run(result, HashSet::new());
}

#[test]
/// [] 8 / 2 == 4
/// violates: Never
fn always_divsion_check() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 4,
        },
        DerivedStream::Binary {
            bin_op: BinaryOperators::Divide,
            idx_lhs: 2,
            idx_rhs: 3,
        },
        DerivedStream::Number(8_000),
        DerivedStream::Number(2_000),
        DerivedStream::Number(4_000),
    ];
    let mut program = program_init(operations);

    let device_stream = mock_default_device_stream(5);

    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 1);

    validate_run(result, HashSet::new());
}

#[test]
/// [] 2 < 4;
/// violates: Never
fn always_less_check() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Less,
            idx_lhs: 1,
            idx_rhs: 2,
        },
        DerivedStream::Number(2_000),
        DerivedStream::Number(4_000),
    ];
    let mut program = program_init(operations);

    let device_stream = mock_default_device_stream(5);

    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 1);

    validate_run(result, HashSet::new());
}

#[test]
/// [] Sum(Foreach(1) != 0) == 10
/// With 10 devices
/// Violates: Never
fn always_nested_device_stack() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 6,
        },
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Binary {
            bin_op: BinaryOperators::NotEqual,
            idx_lhs: 3,
            idx_rhs: 5,
        },
        DerivedStream::Foreach { idx: 4 },
        DerivedStream::Number(1_000),
        DerivedStream::Number(0),
        DerivedStream::Number(10_000),
    ];
    let mut program = program_init(operations);

    let device_stream = mock_specific_device_amount_stream(10, 1);

    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 5);
    println!("{:#?}", result);

    validate_run(result, HashSet::new());
}

#[test]
/// Sum(foreach(1) != 1) == 1
/// Violates: Always
fn always_nested_device_stack_false() {
    let operations: Vec<DerivedStream> = vec![
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 6,
        },
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Binary {
            bin_op: BinaryOperators::NotEqual,
            idx_lhs: 3,
            idx_rhs: 5,
        },
        DerivedStream::Foreach { idx: 4 },
        DerivedStream::Number(1_000),
        DerivedStream::Number(0),
        DerivedStream::Number(1_000),
    ];
    let mut program = program_init(operations);

    let device_stream = mock_default_device_stream(10);

    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 10);

    let errors = create_error_set(0..10, 1);
    validate_run(result, errors);
}

#[test]
/// Prop: [] t%24s = 0S -> always[0,24] sumtime[24](1) < 24s;
/// Violation: Look at the test data
fn time_behaviour_test_1() {
    let operations = {
        use DerivedStream::*;
        vec![
            Binary {
                bin_op: BinaryOperators::Or,
                idx_lhs: 1,
                idx_rhs: 7,
            },
            Unary {
                un_op: UnaryOperators::Not,
                idx: 2,
            },
            Binary {
                bin_op: BinaryOperators::Equal,
                idx_lhs: 3,
                idx_rhs: 6,
            },
            Binary {
                bin_op: BinaryOperators::Mod,
                idx_lhs: 4,
                idx_rhs: 5,
            },
            SpawnTime,
            Number(24000),
            Number(0),
            Miitl {
                miitl_type: Always,
                bound: (0, 24),
                idx: 8,
            },
            Binary {
                bin_op: BinaryOperators::LessEqual,
                idx_lhs: 9,
                idx_rhs: 12,
            },
            Sumtime {
                interval_len: 24,
                idx: 10,
            },
            Sum { idx: 11 },
            Number(1000),
            Number(24000),
        ]
    };
    let mut program = program_init(operations);
    let device_stream = mock_specific_device_amount_stream(1, 100);
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 100);
    let errors = create_error_set([24, 48, 72, 96].into_iter(), 1);
    validate_run(result, errors);
}

#[test]
/// Prop: [] t%24s = 0S -> always[0,24] sumtime[24](1) < 23s;
/// Violation: Look at the test data
fn time_behaviour_test_2() {
    let operations = {
        use DerivedStream::*;
        vec![
            Binary {
                bin_op: BinaryOperators::Or,
                idx_lhs: 1,
                idx_rhs: 7,
            },
            Unary {
                un_op: UnaryOperators::Not,
                idx: 2,
            },
            Binary {
                bin_op: BinaryOperators::Equal,
                idx_lhs: 3,
                idx_rhs: 6,
            },
            Binary {
                bin_op: BinaryOperators::Mod,
                idx_lhs: 4,
                idx_rhs: 5,
            },
            SpawnTime,
            Number(24000),
            Number(0),
            Miitl {
                miitl_type: Always,
                bound: (0, 24),
                idx: 8,
            },
            Binary {
                bin_op: BinaryOperators::LessEqual,
                idx_lhs: 9,
                idx_rhs: 12,
            },
            Sumtime {
                interval_len: 24,
                idx: 10,
            },
            Sum { idx: 11 },
            Number(1000),
            Number(23000),
        ]
    };

    let mut program = program_init(operations);
    let device_stream = mock_specific_device_amount_stream(1, 100);
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 100);

    println!("{:#?}", result);
    let errors = create_error_set([24, 48, 72, 96].into_iter(), 1);
    validate_run(result, errors);
}

#[test]
/// Prop: [] t%24s = 0S -> always[0,24] sumtime[24](1) < 25s;
/// Violation: Look at the test data
fn time_behaviour_test_3() {
    let operations = {
        use DerivedStream::*;
        vec![
            Binary {
                bin_op: BinaryOperators::Or,
                idx_lhs: 1,
                idx_rhs: 7,
            },
            Unary {
                un_op: UnaryOperators::Not,
                idx: 2,
            },
            Binary {
                bin_op: BinaryOperators::Equal,
                idx_lhs: 3,
                idx_rhs: 6,
            },
            Binary {
                bin_op: BinaryOperators::Mod,
                idx_lhs: 4,
                idx_rhs: 5,
            },
            SpawnTime,
            Number(24000),
            Number(0),
            Miitl {
                miitl_type: Always,
                bound: (0, 24),
                idx: 8,
            },
            Binary {
                bin_op: BinaryOperators::LessEqual,
                idx_lhs: 9,
                idx_rhs: 12,
            },
            Sumtime {
                interval_len: 24,
                idx: 10,
            },
            Sum { idx: 11 },
            Number(1000),
            Number(25000),
        ]
    };

    let mut program = program_init(operations);
    let device_stream = mock_specific_device_amount_stream(1, 100);
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_monitor_x_steps(streams, &device_stream, 100);
    let errors = create_error_set([].into_iter(), 1);
    validate_run(result, errors);
}

#[test]
/// [] <>[1,1] 1
/// Violates: Never
fn eventually_expr_true() {
    let operations = vec![
        DerivedStream::Miitl {
            bound: (1, 1),
            idx: 1,
            miitl_type: MIITLType::Eventually
        },
        DerivedStream::Number(1_000),
    ];
    let program = program_init(operations);
    let device_stream = mock_default_device_stream(100);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, 100);

    validate_run(result, HashSet::new());
}

#[test]
/// [] <>[1,1] 0
/// Violates: false once, then true always
fn eventually_expr_false() {
    let operations = vec![
            DerivedStream::Miitl { bound: (1,1), idx: 1, miitl_type: MIITLType::Eventually },
            DerivedStream::Number(0)
        ] ;
    let program = program_init(operations);
    let device_stream = mock_default_device_stream(100);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, 100);

    let errors = create_error_set(1..100, 1);
    validate_run(result, errors);
}

#[test]
/// Prop: [] <>[2,5] t = 2;
/// Violation: Never
fn eventually_expr_time_true() {
    let operations = {
        use DerivedStream::*;
        vec![
            Miitl { bound: ( 2, 5 ), idx: 1, miitl_type: MIITLType::Eventually },
            Binary {bin_op: BinaryOperators::NotEqual, idx_lhs: 2,idx_rhs: 3},
            SpawnTime,
            Number(2_000),
        ]};

    let program = program_init(operations);
    let device_stream = mock_default_device_stream(100);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, 100);



    let errors = create_error_set(2..5, 1);
    validate_run(result, errors);

}

#[test]
/// Prop: always (t = 0) -> (always[6,6](always[9,9]0));
/// Violation: Gives vilolation at 15, and only 15
fn always_always_always(){
    let operations = {
        use DerivedStream::*;
        vec![
            Binary {
                    bin_op: BinaryOperators::Or,
                    idx_lhs: 1,
                    idx_rhs: 5,
                },
                Unary {
                    un_op: UnaryOperators::Not,
                    idx: 2,
                },
                Binary {
                    bin_op: BinaryOperators::Equal,
                    idx_lhs: 3,
                    idx_rhs: 4,
                },
                SpawnTime,
                Number(
                    0,
                ),
                Miitl { 
                    bound: (
                        6,
                        6,
                    ),
                    idx: 6,
                    miitl_type: MIITLType::Always,
                },
                Miitl {
                    bound: (
                        9,
                        9,
                    ),
                    idx: 7,
                    miitl_type: MIITLType::Always,
                },
                Number(
                    0,
                ),
        ]
    };

    let program = program_init(operations);
    let device_stream = mock_default_device_stream(100);
    let streams = &mut program.environment.unwrap();
    let result = run_monitor_x_steps(streams, &device_stream, 100);
 
    let errors = create_error_set(15..=15, 1);
    validate_run(result, errors);
}
