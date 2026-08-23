use crate::{program::{
    Program,
    expressions::Expr,
    function_types::FunctionType,
    member_types::MemberType,
    operations::{BinaryOperators, UnaryOperators},
    units::Unit,
}, utils::test_helper_func::{ always_expr, binary_expr, custom_number_expr, string_expr, custom_unit_expr, function_expr, member_expr, number_expr, unary_expr, unit_expr}
};

// #[test]
// fn eventually() {
//     let program = Program::new("eventually true;").unwrap();
//     assert_eq!(
//         program.expressions.first().unwrap().expr,
//         eventually_expr(custom_number_expr(1_000))
//     );
// }

#[test]
fn always() {
    let program = Program::new("always false;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(custom_number_expr(0))
    );
}

// #[test]
// fn eventually_interval() {
//     let program = Program::new("eventually[1s,3h] false;").unwrap();
//     assert_eq!(
//         program.expressions.first().unwrap().expr,
//         eventually_interval_expr(interval_expr(custom_unit_expr(1_000, Unit::Seconds), custom_unit_expr(3_000, Unit::Hours)), custom_number_expr(0))
//     );
// }

// #[test]
// fn always_interval() {
//     let program = Program::new("always[1s,3h] false;").unwrap();
//     assert_eq!(
//         program.expressions.first().unwrap().expr,
//         always_interval_expr(interval_expr(custom_unit_expr(1_000, Unit::Seconds), custom_unit_expr(3_000, Unit::Hours)), custom_number_expr(0))
//     );
// }

#[test]
fn current_time() {
    let program = Program::new("always t > 0;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(Expr::CurrentTime, custom_number_expr(0), BinaryOperators::Greater))
    );
}

#[test]
fn power_unit() {
    let program = Program::new("always 5W = 5W;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(unit_expr(Unit::Watt), unit_expr(Unit::Watt), BinaryOperators::Equal))
    );
}

#[test]
fn greater() {
    let program = Program::new("always 1 > 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Greater))
    );
}

#[test]
fn less() {
    let program = Program::new("always 1 < 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Less))
    );
}

#[test]
fn greater_equal() {
    let program = Program::new("always 1 >= 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::GreaterEqual))
    );
}

#[test]
fn less_equal() {
    let program = Program::new("always 1 <= 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::LessEqual))
    );
}

#[test]
fn equal() {
    let program = Program::new("always 1 = 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Equal))
    );
}

#[test]
fn not_equal() {
    let program = Program::new("always 1 != 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::NotEqual))
    );
}

#[test]
fn or() {
    let program = Program::new("always 1 | 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Or))
    );
}

#[test]
fn implies() {
    let program = Program::new("always 1 -> 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Implies))
    );
}

#[test]
fn and() {
    let program = Program::new("always 1 & 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::And))
    );
}

#[test]
fn add() {
    let program = Program::new("always (1 + 5) > 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Plus), number_expr(), BinaryOperators::Greater))
    );
}

#[test]
fn minus() {
    let program = Program::new("always (1 - 5) > 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Minus), number_expr(), BinaryOperators::Greater))
    );
}

#[test]
fn times() {
    let program = Program::new("always (1 * 5) > 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Times), number_expr(), BinaryOperators::Greater))
    );
}

#[test]
fn devide() {
    let program = Program::new("always (1 / 5) > 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Divide), number_expr(), BinaryOperators::Greater))
    );
}

#[test]
fn modulo() {
    let program = Program::new("always (1 % 5) > 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(binary_expr(custom_number_expr(1_000), number_expr(), BinaryOperators::Mod), number_expr(), BinaryOperators::Greater))
    );
}

#[test]
fn not() {
    let program = Program::new("always !true;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(unary_expr(custom_number_expr(1_000), UnaryOperators::Not))
    );
}

#[test]
fn negative() {
    let program = Program::new("always -5 = 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(unary_expr(number_expr(), UnaryOperators::Negative), number_expr(), BinaryOperators::Equal))
    );
}

#[test]
fn sum() {
    let program = Program::new("always sum(5) > 1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(function_expr(FunctionType::Sum, number_expr(), None), custom_number_expr(1_000), BinaryOperators::Greater))
    );
}

#[test]
fn avg() {
    let program = Program::new("always avg(5) > 1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(function_expr(FunctionType::Avg, number_expr(), None), custom_number_expr(1_000), BinaryOperators::Greater))
    );
}

#[test]
fn count() {
    let program = Program::new("always count(5) > 1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(function_expr(FunctionType::Count, number_expr(), None), custom_number_expr(1_000), BinaryOperators::Greater))
    );
}

#[test]
fn sumtime() {
    let program = Program::new("always sumtime[10s](5) > 1;").unwrap();
    let bound = Some(custom_unit_expr(10_000, Unit::Seconds));
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(function_expr(FunctionType::Sumtime, number_expr(), bound), custom_number_expr(1_000), BinaryOperators::Greater))
    );
}

#[test]
fn avgtime() {
    let program = Program::new("always avgtime[10s](5) > 1;").unwrap();
    let bound = Some(custom_unit_expr(10_000, Unit::Seconds));
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(function_expr(FunctionType::Avgtime, number_expr(), bound), custom_number_expr(1_000), BinaryOperators::Greater)));
}

#[test]
fn counttime() {
    let program = Program::new("always counttime[10s](5) > 1;").unwrap();
    let bound = Some(custom_unit_expr(10_000, Unit::Seconds));
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(binary_expr(function_expr(FunctionType::Counttime, number_expr(), bound), custom_number_expr(1_000), BinaryOperators::Greater)));
}

#[test]
fn foreach() {
    let program = Program::new("always foreach(5);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(function_expr(FunctionType::Foreach, number_expr(), None))
    );
}

#[test]
fn bool() {
    let program = Program::new("always true;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        always_expr(custom_number_expr(1_000))
    );
}

#[test]
fn string() {
    let program = Program::new("always count(name=christian);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr, 
        always_expr(function_expr(FunctionType::Count, binary_expr(member_expr(MemberType::Name), string_expr(), BinaryOperators::Equal), None))
    );
}
