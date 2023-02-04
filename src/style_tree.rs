use crate::css::{Rule, Selector, SimpleSelector, MatchedRule};
use crate::dom::ElementData;

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
