extern crate hime_redist;

use crate::grammar::{cfg, const_properties};
use hime_redist::ast::AstNode;

fn tree_to_str<'a>(node: AstNode<'_, '_, 'a>, crossings: Vec<bool>) -> String {
    let mut temp: String = String::new();
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            temp.push_str(if crossings[i] { "|   " } else { "    " });
            i += 1;
        }
        temp.push_str("+-> ");
    }
    let newlinenode: String = node.to_string() + &"\n".to_owned();
    temp.push_str(newlinenode.as_str());
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        temp.push_str(tree_to_str(children.at(i), child_crossings).as_str());
        i += 1;
    }
    temp
}

#[test]
fn property1() {
    let actual: String = tree_to_str(
        cfg::parse_str("always (t % 24h = 0) -> always[0h,24h] sumtime(active * power) <10 kWh;")
            .get_ast()
            .get_root(),
        Vec::<bool>::new(),
    );
    assert_eq!(actual, const_properties::PROPERTY1.to_string());
}

#[test]
fn property2() {
    let actual: String = tree_to_str(
        cfg::parse_str("not eventually count(active) > 5;")
            .get_ast()
            .get_root(),
        Vec::<bool>::new(),
    );
    assert_eq!(actual, const_properties::PROPERTY2.to_string());
}

#[test]
fn property3() {
    let actual: String = tree_to_str(
        cfg::parse_str("always foreach(active -> eventually[0h,6h] !active);")
            .get_ast()
            .get_root(),
        Vec::<bool>::new(),
    );
    assert_eq!(actual, const_properties::PROPERTY3.to_string());
}

#[test]
fn property4() {
    let actual: String = tree_to_str(
        cfg::parse_str("always count(name=fridge & active);")
            .get_ast()
            .get_root(),
        Vec::<bool>::new(),
    );
    assert_eq!(actual, const_properties::PROPERTY4.to_string());
}

#[test]
fn property5() {
    let actual: String = tree_to_str(
        cfg::parse_str("always count(active) >= 5 -> eventually[0h,6h] count(active) < 5;")
            .get_ast()
            .get_root(),
        Vec::<bool>::new(),
    );
    assert_eq!(actual, const_properties::PROPERTY5.to_string());
}

#[test]
fn property6() {
    let actual: String = tree_to_str(
        cfg::parse_str("always sum(active * power) <= 100 w;")
            .get_ast()
            .get_root(),
        Vec::<bool>::new(),
    );
    assert_eq!(actual, const_properties::PROPERTY6.to_string());
}

#[test]
fn property7() {
    let actual: String = tree_to_str(
        cfg::parse_str("always 7; eventually 7;")
            .get_ast()
            .get_root(),
        Vec::<bool>::new(),
    );
    assert_eq!(actual, const_properties::PROPERTY7.to_string());
}

#[test]
fn property8() {
    let actual: String = tree_to_str(
        cfg::parse_str("not until(active,10);").get_ast().get_root(),
        Vec::<bool>::new(),
    );
    assert_eq!(actual, const_properties::PROPERTY8.to_string());
}
