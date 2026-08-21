use rv_iot::{
    monitor::streams::OutputStream,
    monitor_setup::operation_types::{AggregateType, DerivedStream, PropLTL},
    program::{
        Program,
        expressions::SpannedExpr,
        function_types::FunctionType,
        member_types::MemberType,
        operations::{BinaryOperators, UnaryOperators},
    },
    utils::test_helper_func::{
        always_expr, always_interval_expr, binary_expr, current_time, custom_number_expr, eventually_interval_expr, function_expr, interval_expr, member_expr, number_expr, string_expr, unary_expr
    },
};

#[test]
fn test1() {
    let mut program =
        Program::new("always (t % 24h = 0s) -> always[0h,24h] sumtime[5s](power) < 10 kWh;")
            .unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.soundness_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(binary_expr(
                unary_expr(
                    binary_expr(
                        binary_expr(
                            current_time(),
                            custom_number_expr(86_400_000),
                            BinaryOperators::Mod,
                        ),
                        custom_number_expr(0),
                        BinaryOperators::Equal,
                    ),
                    UnaryOperators::Not,
                ),
                always_interval_expr(
                    interval_expr(custom_number_expr(0), custom_number_expr(86_400_000)),
                    binary_expr(
                        function_expr(
                            FunctionType::Sumtime,
                            member_expr(MemberType::Power), 
                            Some(custom_number_expr(5_000))
                        ),
                        custom_number_expr(36_000_000_000),
                        BinaryOperators::Less,
                    ),
                ),
                BinaryOperators::Or,
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test2() {
    let mut program = Program::new("! eventually count(power) > 5;").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.soundness_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(unary_expr(
                binary_expr(
                    function_expr(
                        FunctionType::Sum,
                        binary_expr(
                            member_expr(MemberType::Power),
                            custom_number_expr(0),
                            BinaryOperators::NotEqual,
                        ),None
                    ),
                    number_expr(),
                    BinaryOperators::Greater,
                ),
                UnaryOperators::Not,
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test3() {
    let mut program = Program::new("always foreach(power/1w -> eventually[0h,6h] !(power/1w));").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.soundness_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(function_expr(
                FunctionType::Foreach,
                binary_expr(
                    unary_expr(binary_expr(member_expr(MemberType::Power),custom_number_expr(1_000),BinaryOperators::Divide), UnaryOperators::Not),
                    eventually_interval_expr(
                        interval_expr(custom_number_expr(0), custom_number_expr(21_600_000)),
                        unary_expr(binary_expr(member_expr(MemberType::Power),custom_number_expr(1_000),BinaryOperators::Divide), UnaryOperators::Not),
                    ),
                    BinaryOperators::Or,
                ),None
            )),
        }],
        environment: None,
    };


    assert_eq!(program, expected_program);
}

#[test]
fn test4() {
    let mut program = Program::new("always count(name=christian & power);").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.soundness_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(function_expr(
                FunctionType::Sum,
                binary_expr(
                    unary_expr(
                        binary_expr(
                            unary_expr(
                                binary_expr(
                                    member_expr(MemberType::Name),
                                    string_expr(),
                                    BinaryOperators::Equal,
                                ),
                                UnaryOperators::Not,
                            ),
                            unary_expr(member_expr(MemberType::Power), UnaryOperators::Not),
                            BinaryOperators::Or,
                        ),
                        UnaryOperators::Not,
                    ),
                    custom_number_expr(0),
                    BinaryOperators::NotEqual,
                ),None
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test5() {
    let mut program =
        Program::new("always count(power) >= 5 -> eventually[0h,6h] count(power) < 5;").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.soundness_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(binary_expr(
                unary_expr(
                    binary_expr(
                        function_expr(
                            FunctionType::Sum,
                            binary_expr(
                                member_expr(MemberType::Power),
                                custom_number_expr(0),
                                BinaryOperators::NotEqual,
                            ),
                            None
                        ),
                        number_expr(),
                        BinaryOperators::GreaterEqual,
                    ),
                    UnaryOperators::Not,
                ),
                eventually_interval_expr(
                    interval_expr(custom_number_expr(0), custom_number_expr(21_600_000)),
                    binary_expr(
                        function_expr(
                            FunctionType::Sum,
                            binary_expr(
                                member_expr(MemberType::Power),
                                custom_number_expr(0),
                                BinaryOperators::NotEqual,
                            ),
                            None
                        ),
                        number_expr(),
                        BinaryOperators::Less,
                    ),
                ),
                BinaryOperators::Or,
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test6() {
    let mut program = Program::new("always sum(1 * power) <= 100 W;").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.soundness_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(binary_expr(
                function_expr(
                    FunctionType::Sum,
                    binary_expr(
                        custom_number_expr(1_000),
                        member_expr(MemberType::Power),
                        BinaryOperators::Times,
                    ),
                    None
                ),
                custom_number_expr(100_000),
                BinaryOperators::LessEqual,
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test7() {
    let mut program = Program::new(
        "always 1; 
    ! eventually 4;",
    )
    .unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.soundness_check().is_ok());

    let expected_program = Program {
        expressions: vec![
            SpannedExpr {
                line: 1,
                expr: always_expr(custom_number_expr(1_000)),
            },
            SpannedExpr {
                line: 2,
                expr: always_expr(custom_number_expr(0)),
            },
        ],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test8() {
    let mut program = Program::new(
        "!eventually foreach( 1 -> power > 5 W);
        always sumtime[5s + 5s + 5s * 1](power * (name = Roomba)) < 200Ws;",
    )
    .unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.soundness_check().is_ok());
    assert!(program.compile_properties().is_ok());

    let expected_env = [
        OutputStream::from((
            PropLTL::Always,
            vec![
                Operation::Unary { un_op: UnaryOperators::Not, idx: 1 },
                Operation::Foreach { idx: 2 },
                Operation::Binary {
                    bin_op: BinaryOperators::Or,
                    idx_lhs: 3,
                    idx_rhs: 4,
                },
                Operation::Number(0),
                Operation::Binary {
                    bin_op: BinaryOperators::Greater,
                    idx_lhs: 5,
                    idx_rhs: 6,
                },
                Operation::Member(MemberType::Power),
                Operation::Number(5_000),
            ],
            None,
        )),
        OutputStream::from((
            PropLTL::Always,
            vec![
                Operation::Binary {
                    bin_op: BinaryOperators::Less,
                    idx_lhs: 1,
                    idx_rhs: 8,
                },
                Operation::TimeFunction {
                    idx: 2,
                    function_type: AggregateType::Sum,
                    history: Vec::new(),
                    bound: 15,
                },
                Operation::AggregateFunction {
                    idx: 3,
                    function_type: AggregateType::Sum,
                },
                Operation::Binary {
                    bin_op: BinaryOperators::Times,
                    idx_lhs: 4,
                    idx_rhs: 5,
                },
                Operation::Member(MemberType::Power),
                Operation::Binary {
                    bin_op: BinaryOperators::Equal,
                    idx_lhs: 6,
                    idx_rhs: 7,
                },
                Operation::Member(MemberType::Name),
                Operation::String("roomba".to_owned()),
                Operation::Number(200_000),
            ],
            None,
        )),
    ];
    assert_eq!(program.environment.unwrap().as_slice(), expected_env);
}

/*
    Because of the functionality of running the monitor, 
    this is tested within streams_test
*/
