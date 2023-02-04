use crate::css::{Rule, Selector, SimpleSelector, Specificity, Value};
use crate::dom::{ElementData, Node};

use std::collections::HashMap;

// Map CSS properties to values.
type PropertyMap = HashMap<String, Value>;

struct MatchedRule<'a> {
    specificity: Specificity,
    rule: &'a Rule,
}

impl<'a> MatchedRule<'a> {
    fn new(specificity: Specificity, rule: &'a Rule) -> Self {
        Self { specificity, rule }
    }
}

// Hold's a nodes style data (and their children).
// # TODO: Merge styles into Node field (possibly).
struct StyleNode<'a> {
    node: &'a Node, // DOM node
    specified_values: PropertyMap,
    children: Vec<StyleNode<'a>>,
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|x| *x != elem.tag_name) {
        return false;
    }

    if selector.id.iter().any(|x| Some(x) != elem.id()) {
        return false;
    }

    if selector
        .class
        .iter()
        .any(|x| !elem.classes().contains(x.as_str()))
    {
        return false;
    }

    true
}

// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|s| matches(elem, s))
        .map(|s| MatchedRule::new(s.specificity(), rule))
}
