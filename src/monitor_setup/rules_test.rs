use crate::program::operations::UnaryOperators;
use crate::{
    monitor_setup::operation_types::{DerivedStream, MIITLType},
    program::{
        function_types::FunctionType,
        member_types::MemberType,
        operations::BinaryOperators::{self, Divide},
    },
    utils::test_helper_func::*,
};

#[test]
fn constant_rules() {
    let expr = custom_number_expr(10_000);
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(yes_expr.unwrap(), vec![DerivedStream::Number(10_000)]);

    let expr = string_expr();
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(
        yes_expr.unwrap(),
        vec![DerivedStream::String(String::from("christian"))]
    );

    let expr = current_time();
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(yes_expr.unwrap(), vec![DerivedStream::SpawnTime]);

    let expr = member_expr(MemberType::Name);
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(
        yes_expr.unwrap(),
        vec![DerivedStream::Member(MemberType::Name)]
    );

    let expr = member_expr(MemberType::Power);
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(
        yes_expr.unwrap(),
        vec![DerivedStream::Member(MemberType::Power)]
    );
}

#[test]
fn ltl_rules() {
    //Always Unbounded NOT POSSIBLE ANYMORE
    let num_expr = number_expr();
    // let compiled_expr = always_expr(num_expr.clone()).compile_expression();
    // assert!(compiled_expr.is_ok());
    // assert_eq!(
    //     compiled_expr.unwrap().as_slice(),
    //     [
    //         DerivedStream::LTLAlwaysUnbounded { idx: 1 },
    //         DerivedStream::Number(5000)
    //     ]
    // );

    //Always bounded
    let interval = interval_expr(custom_number_expr(10000), custom_number_expr(20000));
    let compiled_expr = always_interval_expr(interval, num_expr.clone()).compile_expression();
    assert!(compiled_expr.is_ok());
    assert_eq!(
        compiled_expr.unwrap().as_slice(),
        [
            DerivedStream::Miitl {
                bound: (10, 20),
                idx: 1,
                miitl_type: MIITLType::Always
            },
            DerivedStream::Number(5000)
        ]
    );

    //Eventually bounded
    let interval = interval_expr(custom_number_expr(10000), custom_number_expr(20000));
    let compiled_expr = eventually_interval_expr(interval, num_expr.clone()).compile_expression();
    assert!(compiled_expr.is_ok());
    assert_eq!(
        compiled_expr.unwrap().as_slice(),
        [
            DerivedStream::Miitl {
                bound: (10, 20),
                idx: 1,
                miitl_type: MIITLType::Eventually
            },
            DerivedStream::Number(5000)
        ]
    );

    //Illegals
    let illegals = [
        eventually_expr(num_expr.clone()),
        always_negated_expr(num_expr.clone()),
    ];
    assert!(illegals.iter().all(|ill| ill.compile_expression().is_err()));
}

#[test]
fn binary_rules() {
    let all = [
        BinaryOperators::Equal,
        BinaryOperators::Less,
        BinaryOperators::LessEqual,
        BinaryOperators::GreaterEqual,
        BinaryOperators::NotEqual,
        BinaryOperators::Plus,
        BinaryOperators::Minus,
        BinaryOperators::Times,
        BinaryOperators::Divide,
        BinaryOperators::Mod,
        BinaryOperators::Or,
        BinaryOperators::Greater,
    ];
    for cur_type in all {
        let expr = binary_expr(
            custom_number_expr(10_000),
            custom_number_expr(10_000),
            cur_type.clone(),
        );
        let yes_expr = expr.compile_expression();
        assert!(yes_expr.is_ok());
        assert_eq!(
            yes_expr.unwrap(),
            vec![
                DerivedStream::Binary {
                    bin_op: cur_type,
                    idx_lhs: 1,
                    idx_rhs: 2
                },
                DerivedStream::Number(10_000),
                DerivedStream::Number(10_000)
            ]
        );
    }
}

#[test]
fn unary_rules() {
    let all = [UnaryOperators::Not, UnaryOperators::Negative];
    for cur_type in all {
        let expr = unary_expr(custom_number_expr(10_000), cur_type.clone());
        let yes_expr = expr.compile_expression();
        assert!(yes_expr.is_ok());
        assert_eq!(
            yes_expr.unwrap(),
            vec![
                DerivedStream::Unary {
                    un_op: cur_type,
                    idx: 1
                },
                DerivedStream::Number(10_000)
            ]
        );
    }
}

#[test]
fn function_rules() {
    let all = [
        FunctionType::Sum,
        FunctionType::Avg,
        FunctionType::Foreach,
        FunctionType::Sumtime,
        FunctionType::Avgtime,
    ];
    let bounds = [
        None,
        None,
        None,
        Some(custom_number_expr(100_000)),
        Some(custom_number_expr(100_000)),
    ];

    for (cur_type, bound) in all.iter().zip(bounds) {
        let expr = function_expr(cur_type.clone(), custom_number_expr(10_000), bound);
        let yes_expr = expr.compile_expression();
        assert!(yes_expr.is_ok());
        match cur_type.clone() {
            FunctionType::Sum => assert_eq!(
                yes_expr.unwrap(),
                vec![DerivedStream::Sum { idx: 1 }, DerivedStream::Number(10_000)]
            ),
            FunctionType::Avg => assert_eq!(
                yes_expr.unwrap(),
                vec![
                    DerivedStream::Binary {
                        bin_op: Divide,
                        idx_lhs: 1,
                        idx_rhs: 3
                    },
                    DerivedStream::Sum { idx: 2 },
                    DerivedStream::Number(10_000),
                    DerivedStream::Size
                ]
            ),
            FunctionType::Foreach => assert_eq!(
                yes_expr.unwrap(),
                vec![
                    DerivedStream::Foreach { idx: 1 },
                    DerivedStream::Number(10_000)
                ]
            ),
            FunctionType::Sumtime => assert_eq!(
                yes_expr.unwrap(),
                vec![
                    DerivedStream::Sumtime {
                        interval_len: 100,
                        idx: 1
                    },
                    DerivedStream::Sum { idx: 2 },
                    DerivedStream::Number(10_000)
                ]
            ),
            FunctionType::Avgtime => assert_eq!(
                yes_expr.unwrap(),
                vec![
                    DerivedStream::Binary {
                        bin_op: Divide,
                        idx_lhs: 1,
                        idx_rhs: 4
                    },
                    DerivedStream::Sumtime {
                        interval_len: 100,
                        idx: 2
                    },
                    DerivedStream::Sum { idx: 3 },
                    DerivedStream::Number(10_000),
                    DerivedStream::Number(100 + 1)
                ]
            ),
            _ => unreachable!(),
        }
    }
}

#[test]
fn function_rules_not_allowed() {
    let all = [FunctionType::Count, FunctionType::Counttime];
    let bounds = [None, Some(custom_number_expr(10_000))];

    for (cur_type, bound) in all.iter().zip(bounds) {
        let expr = function_expr(cur_type.clone(), custom_number_expr(10_000), bound);
        let yes_expr = expr.compile_expression();
        assert!(yes_expr.is_err());
    }
}

#[test]
fn medium_expr() {
    let mem_name = member_expr(MemberType::Name);
    let str = string_expr();
    let bin_op_eq = binary_expr(mem_name, str, BinaryOperators::Equal);
    let mem_pow = member_expr(MemberType::Power);
    let bin_op = binary_expr(mem_pow, bin_op_eq, BinaryOperators::Times);
    let sumtime = function_expr(
        FunctionType::Sumtime,
        bin_op,
        Some(custom_number_expr(100_000)),
    );
    let num = number_expr();
    let large_expr = binary_expr(sumtime, num, BinaryOperators::Less);

    assert_eq!(
        large_expr.compile_expression().unwrap(),
        [
            DerivedStream::Binary {
                bin_op: BinaryOperators::Less,
                idx_lhs: 1,
                idx_rhs: 8
            },
            DerivedStream::Sumtime {
                idx: 2,
                interval_len: 100
            },
            DerivedStream::Sum { idx: 3 },
            DerivedStream::Binary {
                bin_op: BinaryOperators::Times,
                idx_lhs: 4,
                idx_rhs: 5
            },
            DerivedStream::Member(MemberType::Power),
            DerivedStream::Binary {
                bin_op: BinaryOperators::Equal,
                idx_lhs: 6,
                idx_rhs: 7
            },
            DerivedStream::Member(MemberType::Name),
            DerivedStream::String("christian".to_owned()),
            DerivedStream::Number(5_000)
        ]
    )
}
