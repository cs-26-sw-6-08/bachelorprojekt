
use crate::{program::{function_types::FunctionType, operations::{BinaryOperators, UnaryOperators}}, utils::test_helper_func::{always_expr, always_interval_expr, binary_expr, eventually_expr, eventually_interval_expr, function_expr, interval_expr, number_expr, unary_expr}};


#[test]
fn should_pass(){
    // Always = success
    assert!(always_expr(number_expr()).monitorability_check().is_ok());
    // Always Interval = success
    assert!(always_interval_expr(interval_expr(number_expr(), number_expr()), number_expr()).monitorability_check().is_ok());
    // Eventually interval = success
    assert!(eventually_interval_expr(interval_expr(number_expr(), number_expr()),number_expr()).monitorability_check().is_ok());
    // Always eventually interval = success
    assert!(always_expr(eventually_interval_expr(interval_expr(number_expr(), number_expr()), number_expr())).monitorability_check().is_ok());
    // Always interval eventually interval = success
    assert!(always_interval_expr(interval_expr(number_expr(), number_expr()), eventually_interval_expr(interval_expr(number_expr(), number_expr()),number_expr())).monitorability_check().is_ok());
    // Eventually interval always = success
    assert!(eventually_interval_expr(interval_expr(number_expr(), number_expr()), always_expr(number_expr())).monitorability_check().is_ok());
    // Eventually interval eventually interval = success
    assert!(eventually_interval_expr(interval_expr(number_expr(), number_expr()), eventually_interval_expr(interval_expr(number_expr(), number_expr()), number_expr())).monitorability_check().is_ok());
    // Always bool and always bool = success
    assert!(always_expr(binary_expr(number_expr(), always_expr(number_expr()), BinaryOperators::And)).monitorability_check().is_ok());
    // Always bool or always bool = success
    assert!(always_expr(binary_expr(number_expr(), always_expr(number_expr()), BinaryOperators::Or)).monitorability_check().is_ok());
    // Always sum number = success
    assert!(always_expr(function_expr(FunctionType::Sum, number_expr(), None)).monitorability_check().is_ok());
    // Always foreach foreach foreach number = success
    assert!(always_expr(function_expr(FunctionType::Foreach, function_expr(FunctionType::Foreach, function_expr(FunctionType::Foreach, number_expr(), None), None), None)).monitorability_check().is_ok());
    // Always foreach sumtime number = success
    // assert!(always_expr(function_expr(FunctionType::Foreach, function_expr(FunctionType::Sumtime, number_expr(), Some(number_expr())), None)).monitorability_check().is_ok());
    // Always counttime foreach number = success
    assert!(always_expr(function_expr(FunctionType::Sumtime, function_expr(FunctionType::Foreach, number_expr(), None), Some(number_expr()))).monitorability_check().is_ok());
    // Always unary count foreach number = success
    assert!(always_expr(unary_expr(function_expr(FunctionType::Count, function_expr(FunctionType::Foreach, number_expr(), None), None), UnaryOperators::Not)).monitorability_check().is_ok());
    // Always unary count foreach sum number = success
    assert!(always_expr(unary_expr(function_expr(FunctionType::Count, function_expr(FunctionType::Foreach, function_expr(FunctionType::Sum, number_expr(), None), None), None), UnaryOperators::Not)).monitorability_check().is_ok());
    // Always sum avg count sum number = success
    assert!(always_expr(function_expr(FunctionType::Sum, function_expr(FunctionType::Avg, function_expr(FunctionType::Count, function_expr(FunctionType::Sum, number_expr(), None), None), None), None)).monitorability_check().is_ok());
    // Always sumtime sum count avg number = success
    assert!(always_expr(function_expr(FunctionType::Sumtime, function_expr(FunctionType::Sum, function_expr(FunctionType::Count, function_expr(FunctionType::Avg, number_expr(), None), None), None), Some(number_expr()))).monitorability_check().is_ok());
    // Always binary(always or (avgtime count)) = success
    assert!(always_expr(binary_expr(always_expr(number_expr()), function_expr(FunctionType::Avgtime, function_expr(FunctionType::Count, number_expr(), None), Some(number_expr())), BinaryOperators::Or)).monitorability_check().is_ok());

}

#[test]
fn should_fail() {
    // Eventually = fail
    assert!(eventually_expr(number_expr()).monitorability_check().is_err());
    // Always eventually = fail
    assert!(always_expr(eventually_expr(number_expr())).monitorability_check().is_err());
    // Eventually always = fail
    assert!(eventually_expr(always_expr(number_expr())).monitorability_check().is_err());
    // Eventually eventually = fail
    assert!(eventually_expr(eventually_expr(number_expr())).monitorability_check().is_err());
    // Eventually always interval = fail
    assert!(eventually_expr(always_interval_expr(interval_expr(number_expr(), number_expr()),number_expr())).monitorability_check().is_err());
    // Eventually eventually interval = fail
    assert!(eventually_expr(eventually_interval_expr(interval_expr(number_expr(), number_expr()),number_expr())).monitorability_check().is_err());
    // Always bool and eventually bool = fail
    assert!(always_expr(binary_expr(number_expr(), eventually_expr(number_expr()), BinaryOperators::And)).monitorability_check().is_err());
    // Always bool or eventually bool = fail
    assert!(always_expr(binary_expr(number_expr(), eventually_expr(number_expr()), BinaryOperators::Or)).monitorability_check().is_err());
    // Always sumtime foreach counttime number = fail
    assert!(always_expr(function_expr(FunctionType::Sumtime, function_expr(FunctionType::Foreach, function_expr(FunctionType::Counttime, number_expr(), Some(number_expr())), None), Some(number_expr()))).monitorability_check().is_err());
    // Always foreach sumtime avgtime number = fail
    assert!(always_expr(function_expr(FunctionType::Foreach, function_expr(FunctionType::Sumtime, function_expr(FunctionType::Avgtime, number_expr(), Some(number_expr())), Some(number_expr())), None)).monitorability_check().is_err());
    // Always sum sumtime number = fail
    assert!(always_expr(function_expr(FunctionType::Sum, function_expr(FunctionType::Sumtime, number_expr(), Some(number_expr())), None)).monitorability_check().is_err());
    // Always avg always number = fail
    assert!(always_expr(function_expr(FunctionType::Avg, always_expr(number_expr()), None)).monitorability_check().is_err());
    // Always unary count foreach always number = fail
    assert!(always_expr(unary_expr(function_expr(FunctionType::Count, function_expr(FunctionType::Foreach, always_expr(number_expr()), None), None), UnaryOperators::Not)).monitorability_check().is_err());
    // Always binary((sum sumtime) or (avgtime count)) = fail
    assert!(always_expr(binary_expr(function_expr(FunctionType::Sum, function_expr(FunctionType::Sumtime, number_expr(), Some(number_expr())), None), function_expr(FunctionType::Avgtime, function_expr(FunctionType::Count, number_expr(), None), Some(number_expr())), BinaryOperators::Or)).monitorability_check().is_err());
}


