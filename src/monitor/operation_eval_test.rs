use crate::{monitor::{operation_eval::eval_operations, streams::IoTStream, types::StreamOutput}, monitor_setup::operation_types::{AggregateType, ExprLTL, DerivedStream}, program::{member_types::MemberType, operations::{BinaryOperators, UnaryOperators}}, utils::test_helper_func::mock_devices};


#[test]
fn test_constants() {

    let mut operations = [
        DerivedStream::Number(4000),
        DerivedStream::String("christian".into()),
        DerivedStream::SpawnTime,
    ];
    let (spawn_t, cur_t) = (0, 1);
    let devices = mock_devices(1).into();
    assert_eq!(
        StreamOutput::from(4000),    
        eval_operations(&mut operations[0..1], &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StreamOutput::from(&"christian".into()),    
        eval_operations(&mut operations[1..2], &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StreamOutput::from(0),    
        eval_operations(&mut operations[2..3], &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StreamOutput::from(1000),    
        eval_operations(&mut operations[2..3], &devices, &1, &cur_t).unwrap()
    )
}


#[test]
fn aggregate_functions() {
    let mut sum = [
        DerivedStream::AggregateFunction { idx: 1, function_type: AggregateType::Sum },
        DerivedStream::Member(MemberType::Power)
    ];
    let mut avg = [
        DerivedStream::AggregateFunction { idx: 1, function_type: AggregateType::Avg },
        DerivedStream::Member(MemberType::Power)
    ];
    let mut foreach = [
        DerivedStream::Foreach { idx: 1 },
        DerivedStream::Binary { bin_op: BinaryOperators::Equal, idx_lhs: 2, idx_rhs: 3 },
        DerivedStream::Member(MemberType::Power),
        DerivedStream::Number(10)
    ];
    let (spawn_t, cur_t) = (0, 1);
    let devices: IoTStream = mock_devices(3).into();
    let devices_power_all_10: IoTStream = mock_devices(3).into_iter().map(|mut device| { 
        device.power = 10;
        device
     }).collect::<Vec<_>>().into(); 

    assert_eq!(
        StreamOutput::from(30),    
        eval_operations(&mut sum, &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StreamOutput::from(20),    
        eval_operations(&mut sum, &mock_devices(2).into(), &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StreamOutput::from(10),    
        eval_operations(&mut avg, &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StreamOutput::from(false),    
        eval_operations(&mut foreach, &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StreamOutput::from(true),    
        eval_operations(&mut foreach, &devices_power_all_10, &spawn_t, &cur_t).unwrap()
    );
}

#[test]
fn ltl_expressions_always_unbounded() {
    let mut always_unb = [
        DerivedStream::LTLAlwaysUnbounded { idx: 1 }, 
        DerivedStream::Binary { bin_op: BinaryOperators::NotEqual, idx_lhs: 2, idx_rhs: 3 },
        DerivedStream::SpawnTime, 
        DerivedStream::Number(10000)
    ];
    let devices: IoTStream = mock_devices(3).into();
    //Should be false for all times, when t != 10
    assert!(
        (0..10000).filter(|n| *n != 10).all(|t_spawn| eval_operations(&mut always_unb, &devices, &t_spawn, &t_spawn).unwrap() == StreamOutput::from(true) )
    );
    assert_eq!(
        StreamOutput::from(false),
        eval_operations(&mut always_unb, &devices, &10, &10).unwrap()
    );
}

#[test]
fn ltl_expressions_bounded() {
    //1,2,3,4
    let ops = [
         DerivedStream::Binary { bin_op: BinaryOperators::NotEqual, idx_lhs: 2, idx_rhs: 3 },
        DerivedStream::SpawnTime, 
        DerivedStream::Number(2000)
    ];
    let mut always = [
        DerivedStream::LTLBounded { bound: (1,4), idx: 1, not: false, ltl_type: ExprLTL::Always }, 
    ].into_iter().chain(ops.clone()).collect::<Vec<_>>();
    // [][1,4] t=2
    let mut eventually = [
        DerivedStream::LTLBounded { bound: (1,4), idx: 1, not: false, ltl_type: ExprLTL::Eventually(Vec::new()) }, 
    ].into_iter().chain(ops.clone()).collect::<Vec<_>>();
    let devices: IoTStream = mock_devices(3).into();

    assert_eq!(
        StreamOutput::from(true).to_undecided(),
        eval_operations(&mut always, &devices, &0, &1).unwrap()
    );
    assert_eq!(
        StreamOutput::from(true).to_undecided(),
        eval_operations(&mut always, &devices, &2, &2).unwrap()
    );
    assert_eq!(
        StreamOutput::from(true).to_undecided(),
        eval_operations(&mut always, &devices, &2, &3).unwrap()
    );
    assert_eq!(
        StreamOutput::from(true),
        eval_operations(&mut always, &devices, &3, &8).unwrap()
    );
    assert_eq!(
        StreamOutput::from(true).to_undecided(),
        eval_operations(&mut eventually, &devices, &2, &2).unwrap()
    );
    //Within bound -> Should be undecided
    assert_eq!(
        StreamOutput::from(false),
        eval_operations(&mut eventually, &devices, &1, &5).unwrap()
    );
    //Outside bound --> Should be decided
    assert_eq!(
        StreamOutput::from(false),
        eval_operations(&mut eventually, &devices, &1, &6).unwrap()
    );
    assert_eq!(
        StreamOutput::from(false),
        eval_operations(&mut eventually, &devices, &1, &7).unwrap()
    );
}

#[test] 
fn time_functions_unbounded() {
    let devices = mock_devices(5).into();
    let mut sumtime_unbounded = [
        DerivedStream::TimeFunction { idx: 1, function_type: AggregateType::Sum, history: Vec::new(), bound: 100 },
        DerivedStream::AggregateFunction { idx: 2, function_type: AggregateType::Sum }, 
        DerivedStream::Number(1_000)
    ];
    let eval_res = (0..=2).try_fold(StreamOutput::from(0), |_, t_c| {
         eval_operations(&mut sumtime_unbounded, &devices, &0, &t_c)
    });
    assert_eq!(
        StreamOutput::from(15_000).to_undecided(),
        eval_res.unwrap()
    );
    (3..100).for_each(|val| {
        assert_eq!(
            StreamOutput::from(val*5000 + 5000).to_undecided(),
            eval_operations(&mut sumtime_unbounded, &devices, &0, &val).unwrap()
        )
    });
    //Test length of history array
    assert_eq!(
        1,
        if let Operation::TimeFunction { history, .. } = &sumtime_unbounded[0] { history.len() } else { 0 }
    );
    assert_eq!(
        StreamOutput::from(5_000).to_undecided(),
        eval_operations(&mut sumtime_unbounded, &devices, &4, &4).unwrap()
    );
    //Test length of history array
    assert_eq!(
        5,
        if let Operation::TimeFunction { history, .. } = &sumtime_unbounded[0] { history.len() } else { 0 }
    );
    let mut avg_time = [
        DerivedStream::TimeFunction { idx: 1, function_type: AggregateType::Avg, history: Vec::new(), bound: 100 },
        DerivedStream::AggregateFunction { idx: 2, function_type: AggregateType::Sum }, 
        DerivedStream::Number(1_000)
    ];
    let eval_res = (0..=100).try_fold(StreamOutput::from(0), |_, t_c| {
         eval_operations(&mut avg_time, &devices, &0, &t_c)
    });
    assert_eq!(
        StreamOutput::from(15_000/3).to_undecided(),
        eval_res.unwrap()
    );

    let mut avg_time = [
        DerivedStream::TimeFunction { idx: 1, function_type: AggregateType::Avg, history: Vec::new(), bound: 100 },
        DerivedStream::AggregateFunction { idx: 2, function_type: AggregateType::Sum }, 
        DerivedStream::Number(1_000)
    ];
    (0..100).for_each(|val| {
        assert_eq!(
            StreamOutput::from((val*5000 + 5000)/(100+1)).to_undecided(),
            eval_operations(&mut avg_time, &devices, &0, &val).unwrap()
        )
    });
}


#[test] 
fn time_functions_bounded() {
    let devices = mock_devices(5).into();
    let mut sumtime_bounded = [
        DerivedStream::TimeFunction { idx: 1, function_type: AggregateType::Sum, history: Vec::new(), bound: 5 },
        DerivedStream::AggregateFunction { idx: 2, function_type: AggregateType::Sum }, 
        DerivedStream::Number(1_000)
    ];
    //check whether value become decided when out of bounds
    let eval_res = (0..=6).try_fold(StreamOutput::from(0), |_, t_c| {
         eval_operations(&mut sumtime_bounded, &devices, &0, &t_c)
    });
    assert_eq!( StreamOutput::from(30_000), eval_res.unwrap() );

    //Check whether history array stops growing
    let _ = (0..=100).try_fold(StreamOutput::from(0), |_, t_c|
        eval_operations(&mut sumtime_bounded, &devices, &t_c, &t_c)
    );
    assert_eq!(
        6,
        if let Operation::TimeFunction { history, .. } = &sumtime_bounded[0] { history.len() } else { 0 }
    );
}


/// This testcase is expected to return undecided because the eventually element returns false and is therefore undecided 
#[test]
fn check_undecided_operations() {
    let devices = mock_devices(3).into();
    let bin_ops = { 
        use BinaryOperators::*;
        [ Equal, Less, Greater, LessEqual, GreaterEqual, NotEqual, Plus, Minus, Times, Mod,Or ] 
    };
    let expected_results = [
        StreamOutput::from(false).to_undecided(), // ==
        StreamOutput::from(false).to_undecided(), //<
        StreamOutput::from(true).to_undecided(), //>
        StreamOutput::from(false).to_undecided(), //<=
        StreamOutput::from(true).to_undecided(), //>=
        StreamOutput::from(true).to_undecided(), //_ !=
        StreamOutput::from(10_000).to_undecided(), // +
        StreamOutput::from(10_000).to_undecided(), // - 
        StreamOutput::from(0).to_undecided(), //_ * 
        StreamOutput::from(0).to_undecided(), // %,
        StreamOutput::from(true).to_undecided(), // ||
    ];  
    for (op, expected_val) in bin_ops.into_iter().zip(expected_results) {
        let mut operations =  [ 
            DerivedStream::Binary { bin_op: op.clone(), idx_lhs: 1, idx_rhs: 2 },
            DerivedStream::Number(10_000), 
            DerivedStream::LTLBounded { bound: (0, 1000), idx: 3, not: false, ltl_type: ExprLTL::Eventually(Vec::new()) },
            DerivedStream::Number(0)
        ];
        assert_eq!(
            expected_val,
            eval_operations(&mut operations, &devices, &0, &0).unwrap()
        );
    }

    
    let mut negate_ops =  [ 
        DerivedStream::Unary { un_op: UnaryOperators::Negative, idx: 1 },
        DerivedStream::LTLBounded { bound: (0,1000), idx:2, not:false, ltl_type: ExprLTL::Eventually(Vec::new()) },
        DerivedStream::Number(0) 
    ];
     let mut not_ops =  [ 
        DerivedStream::Unary { un_op: UnaryOperators::Not, idx: 1 },
        DerivedStream::LTLBounded { bound: (0,1000), idx:2, not:false, ltl_type: ExprLTL::Eventually(Vec::new()) },
        DerivedStream::Number(0) 
    ];
    assert_eq!(
        StreamOutput::from(0).to_undecided(),
        eval_operations(&mut negate_ops, &devices, &0, &0).unwrap()
    );
     assert_eq!(
        StreamOutput::from(true).to_undecided(),
        eval_operations(&mut not_ops, &devices, &0, &0).unwrap()
    );


}

#[test]
fn test_edge_case_modulo() {
    let devices = mock_devices(1).into();
    let mut modulo = [
        DerivedStream::Binary { bin_op: BinaryOperators::Mod, idx_lhs: 1, idx_rhs: 2 },
        DerivedStream::Number(10_000),
        DerivedStream::Number(6_000)
    ];
    assert_eq!(
        StreamOutput::from(4_000),
        eval_operations(&mut modulo, &devices, &0, &0).unwrap()
    );
    //change the order of  10 and 6;
    modulo[1] = DerivedStream::Number(6_000);
    modulo[2] = DerivedStream::Number(10_000);

     assert_eq!(
        StreamOutput::from(6_000),
        eval_operations(&mut modulo, &devices, &0, &0).unwrap()
    );
}

#[test]
fn binary_operations() {
    let devices = mock_devices(3).into();
    let bin_ops = { 
        use BinaryOperators::*;
        [ Equal, Less, Greater, LessEqual, GreaterEqual, NotEqual, Plus, Minus, Times, Mod,Or ] 
    };
    let expected_results = [
        StreamOutput::from(false), // ==
        StreamOutput::from(false), //<
        StreamOutput::from(true), //>
        StreamOutput::from(false), //<=
        StreamOutput::from(true), //>=
        StreamOutput::from(true), //_ !=
        StreamOutput::from(12_000), // +
        StreamOutput::from(8_000), // - 
        StreamOutput::from(20_000), //_ * 
        StreamOutput::from(0), // %,
        StreamOutput::from(true), // ||
    ];  
    for (op, expected_val) in bin_ops.into_iter().zip(expected_results) {
        let mut operations =  [ 
            DerivedStream::Binary { bin_op: op.clone(), idx_lhs: 1, idx_rhs: 2 },
            DerivedStream::Number(10_000), 
            DerivedStream::Number(2_000)
        ];
        assert_eq!(
            expected_val,
            eval_operations(&mut operations, &devices, &0, &0).unwrap()
        );
    }
}

#[test]
fn unary_operations_test() {
    let devices = mock_devices(3).into();
    
    let mut negate_ops =  [ 
        DerivedStream::Unary { un_op: UnaryOperators::Negative, idx: 1 },
        DerivedStream::Number(10_000), 
    ];
     let mut not_ops =  [ 
        DerivedStream::Unary { un_op: UnaryOperators::Not, idx: 1 },
        DerivedStream::Number(1_000), 
    ];
    assert_eq!(
        StreamOutput::from(-10_000),
        eval_operations(&mut negate_ops, &devices, &0, &0).unwrap()
    );
     assert_eq!(
        StreamOutput::from(false),
        eval_operations(&mut not_ops, &devices, &0, &0).unwrap()
    );
}

