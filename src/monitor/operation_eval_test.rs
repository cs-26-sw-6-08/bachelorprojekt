use crate::{
    monitor::{operation_eval::eval_operations, streams::IoTStream}, monitor_setup::operation_types::{
        DerivedStream, MIITLType::{self, Eventually},
    }, program::{
        member_types::MemberType, operations::{
            BinaryOperators::{self, Divide}, UnaryOperators,
        },
    }, utils::test_helper_func::{mock_default_device_stream, mock_specific_device_amount_stream},
};

#[test]
fn test_constants() {
    let mut operations = [DerivedStream::Number(4_000), DerivedStream::SpawnTime];
    let (spawn_t, cur_t) = (0, 1);
    let devices = mock_default_device_stream(1).into();
    assert_eq!(
        Some(4_000),
        eval_operations(&mut operations[0..1], &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        Some(0),
        eval_operations(&mut operations[1..2], &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        Some(1_000),
        eval_operations(&mut operations[1..2], &devices, &1, &cur_t).unwrap()
    )
}

#[test]
fn aggregate_functions() {
    let mut sum = [
        DerivedStream::Sum { idx: 1 },
        DerivedStream::Member(MemberType::Power),
    ];
    let mut avg = [
        DerivedStream::Binary {
            bin_op: Divide,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Member(MemberType::Power),
        DerivedStream::Size,
    ];
    let mut foreach = [
        DerivedStream::Foreach { idx: 1 },
        DerivedStream::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 2,
            idx_rhs: 3,
        },
        DerivedStream::Member(MemberType::Power),
        DerivedStream::Number(10_000),
    ];
    let (spawn_t, cur_t) = (0, 1);
    let devices: IoTStream = mock_default_device_stream(3);
    let devices_power_all_10: IoTStream = mock_default_device_stream(3)
        .get_devices_own(1)
        .into_iter()
        .map(|mut device| {
            device.power = 10_000;
            device
        })
        .collect::<Vec<_>>()
        .into();

    assert_eq!(
        eval_operations(&mut sum, &devices_power_all_10, &spawn_t, &cur_t).unwrap(),
        Some(30_000)
    );
    assert_eq!(
        Some(10_000),
        eval_operations(
            &mut sum,
            &mock_specific_device_amount_stream(2, 3),
            &spawn_t,
            &cur_t
        )
        .unwrap()
    );
    assert_eq!(
        Some(20_000),
        eval_operations(
            &mut sum,
            &mock_specific_device_amount_stream(4, 3),
            &spawn_t,
            &cur_t
        )
        .unwrap()
    );
    // (5 + 1 +  3) / 3 = 3
    assert_eq!(
        Some(3_000),
        eval_operations(&mut avg, &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        Some(0),
        eval_operations(&mut foreach, &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        Some(1_000),
        eval_operations(&mut foreach, &devices_power_all_10, &spawn_t, &cur_t).unwrap()
    );
}

#[test]
fn ltl_expressions_bounded() {
    //1,2,3,4
    let ops = [
        DerivedStream::Binary {
            bin_op: BinaryOperators::NotEqual,
            idx_lhs: 2,
            idx_rhs: 3,
        },
        DerivedStream::SpawnTime,
        DerivedStream::Number(2000),
    ];
    let mut always = [DerivedStream::Miitl {
        bound: (1, 4),
        idx: 1,
        miitl_type: MIITLType::Always,
    }]
    .into_iter()
    .chain(ops.clone())
    .collect::<Vec<_>>();
    // [][1,4] t=2
    let mut eventually = [DerivedStream::Miitl {
        bound: (1, 4),
        idx: 1,
        miitl_type: MIITLType::Eventually,
    }]
    .into_iter()
    .chain(ops.clone())
    .collect::<Vec<_>>();
    let devices: IoTStream = mock_default_device_stream(3);

    assert_eq!(
        None,
        eval_operations(&mut always, &devices, &0, &1).unwrap()
    );
    assert_eq!(
        None,
        eval_operations(&mut always, &devices, &2, &2).unwrap()
    );
    assert_eq!(
        None,
        eval_operations(&mut always, &devices, &2, &3).unwrap()
    );
    //todo: This line fails -> Is the logic correct? note: Maybe off by-one ? 
    // 8 < 3 + 4 = 7
    assert_eq!(
        Some(1_000),
        eval_operations(&mut always, &devices, &3, &7).unwrap()
    );
    assert_eq!(
        None,
        eval_operations(&mut eventually, &devices, &2, &2).unwrap()
    );
    //Within bound -> Should be undecided
    //5 < 1 + 4 -> Decideable
    assert_eq!(
        Some(1_000),
        eval_operations(&mut eventually, &devices, &1, &5).unwrap()
    );
    //Outside bound --> Should be decided
    assert_eq!(
        Some(1_000),
        eval_operations(&mut eventually, &devices, &1, &6).unwrap()
    );
    assert_eq!(
        Some(1_000),
        eval_operations(&mut eventually, &devices, &1, &7).unwrap()
    );
}

#[test]
fn time_functions_unbounded() {
    let devices = mock_default_device_stream(5);
    let mut sumtime_unbounded = [
        DerivedStream::Sumtime {
            interval_len: 100,
            idx: 1,
        },
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Number(1_000),
    ];

    assert_eq!(
        // StreamOutput::from(15_000).to_undecided(),
        None,
        eval_operations(&mut sumtime_unbounded, &devices, &0, &2).unwrap()
    );
    (3..100).for_each(|val| {
        assert_eq!(
            // StreamOutput::from(val * 5000 + 5000).to_undecided(),
            None,
            eval_operations(&mut sumtime_unbounded, &devices, &0, &val).unwrap()
        )
    });
    assert_eq!(
        // StreamOutput::from(5_000).to_undecided(),
        None,
        eval_operations(&mut sumtime_unbounded, &devices, &4, &4).unwrap()
    );

    let mut avg_time = [
        DerivedStream::Binary { bin_op: Divide, idx_lhs: 1, idx_rhs: 4 },
        DerivedStream::Sumtime {
            interval_len: 100,
            idx: 2,
        },
        DerivedStream::Sum { idx: 3 },
        DerivedStream::Number(1_000),
        DerivedStream::Number(101_000)
    ];
    assert_eq!(
        Some(3_000),
        eval_operations(&mut avg_time, &devices, &0, &100).unwrap()
    );

    let mut avg_time = [
        DerivedStream::Sumtime {
            interval_len: 100,
            idx: 1,
        },
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Number(1_000),
    ];
    (0..100).for_each(|val| {
        assert_eq!(
            // Some((val * 5000 + 5000) / (100 + 1)).to_undecided(),
            None,
            eval_operations(&mut avg_time, &devices, &0, &val).unwrap()
        )
    });
}

#[test]
fn time_functions_bounded() {
    let devices = mock_default_device_stream(5);
    let mut sumtime_bounded = [
        DerivedStream::Sumtime {
            interval_len: 5,
            idx: 1,
        },
        DerivedStream::Sum { idx: 2 },
        DerivedStream::Number(1_000),
    ];
    //check whether value become decided when out of bounds
    //todo: Because of new algorithm, i would argue this should be none 
    let eval_res = eval_operations(&mut sumtime_bounded, &devices, &0, &4);
    assert_eq!(None, eval_res.unwrap());

    let eval_res = eval_operations(&mut sumtime_bounded, &devices, &0, &5);
    assert_eq!(Some(18_000), eval_res.unwrap());

}

/// This testcase is expected to return undecided because the eventually element returns false and is therefore undecided
#[test]
fn check_undecided_operations() {
    let devices = mock_default_device_stream(3);
    let bin_ops = {
        use BinaryOperators::*;
        [
            Equal,
            Less,
            Greater,
            LessEqual,
            GreaterEqual,
            NotEqual,
            Plus,
            Minus,
            Times,
            Mod,
            Or,
        ]
    };
    let expected_results = [
        None, None, None, None, None, None, None, None, None, None, Some(1_000),
    ];
    for (op, expected_val) in bin_ops.into_iter().zip(expected_results) {
        let mut operations = [
            DerivedStream::Binary {
                bin_op: op.clone(),
                idx_lhs: 1,
                idx_rhs: 2,
            },
            DerivedStream::Number(10_000),
            DerivedStream::Miitl {
                miitl_type: Eventually,
                bound: (0, 1),
                idx: 3,
            },
            DerivedStream::Number(0),
        ];
        assert_eq!(
            expected_val,
            eval_operations(&mut operations, &devices, &0, &0).unwrap()
        );
    }

    let mut negate_ops = [
        DerivedStream::Unary {
            un_op: UnaryOperators::Negative,
            idx: 1,
        },
        DerivedStream::Miitl {
            miitl_type: Eventually,
            bound: (0, 1),
            idx: 2,
        },
        DerivedStream::Number(0),
    ];
    let mut not_ops = [
        DerivedStream::Unary {
            un_op: UnaryOperators::Not,
            idx: 1,
        },
        DerivedStream::Miitl {
            miitl_type: Eventually,
            bound: (0, 1000),
            idx: 2,
        },
        DerivedStream::Number(0),
    ];
    assert_eq!(
        None,
        eval_operations(&mut negate_ops, &devices, &0, &0).unwrap()
    );
    assert_eq!(
        None,
        eval_operations(&mut not_ops, &devices, &0, &0).unwrap()
    );
}

#[test]
fn test_edge_case_modulo() {
    let devices = mock_default_device_stream(1).into();
    let mut modulo = [
        DerivedStream::Binary {
            bin_op: BinaryOperators::Mod,
            idx_lhs: 1,
            idx_rhs: 2,
        },
        DerivedStream::Number(10_000),
        DerivedStream::Number(6_000),
    ];
    assert_eq!(
        Some(4_000),
        eval_operations(&mut modulo, &devices, &0, &0).unwrap()
    );
    //change the order of  10 and 6;
    modulo[1] = DerivedStream::Number(6_000);
    modulo[2] = DerivedStream::Number(10_000);

    assert_eq!(
        Some(6_000),
        eval_operations(&mut modulo, &devices, &0, &0).unwrap()
    );
}

#[test]
fn binary_operations() {
    let devices = mock_default_device_stream(3).into();
    let bin_ops = {
        use BinaryOperators::*;
        [
            Equal,
            Less,
            Greater,
            LessEqual,
            GreaterEqual,
            NotEqual,
            Plus,
            Minus,
            Times,
            Mod,
            Or,
        ]
    };
    let expected_results = [
        Some(0),
        Some(0),
        Some(1_000),
        Some(0),
        Some(1_000),
        Some(1_000),
        Some(12_000),
        Some(8_000),
        Some(20_000),
        Some(0),
        Some(1_000),
    ];
    for (op, expected_val) in bin_ops.into_iter().zip(expected_results) {
        let mut operations = [
            DerivedStream::Binary {
                bin_op: op.clone(),
                idx_lhs: 1,
                idx_rhs: 2,
            },
            DerivedStream::Number(10_000),
            DerivedStream::Number(2_000),
        ];
        println!("{op:#?}: exp: {expected_val:#?}, {:#?}", eval_operations(&mut operations, &devices, &0, &0).unwrap());
        assert_eq!(
            expected_val,
            eval_operations(&mut operations, &devices, &0, &0).unwrap()
        );
    }
}

#[test]
fn unary_operations_test() {
    let devices = mock_default_device_stream(3).into();

    let mut negate_ops = [
        DerivedStream::Unary {
            un_op: UnaryOperators::Negative,
            idx: 1,
        },
        DerivedStream::Number(10_000),
    ];
    let mut not_ops = [
        DerivedStream::Unary {
            un_op: UnaryOperators::Not,
            idx: 1,
        },
        DerivedStream::Number(1_000),
    ];
    assert_eq!(
        Some(-10_000),
        eval_operations(&mut negate_ops, &devices, &0, &0).unwrap()
    );
    assert_eq!(
        Some(0),
        eval_operations(&mut not_ops, &devices, &0, &0).unwrap()
    );
}
