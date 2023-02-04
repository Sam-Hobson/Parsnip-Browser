use std::collections::HashMap;

use crate::style::css::{MatchedRule, PropertyMap, Rule, Selector, SimpleSelector, Stylesheet};
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

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|r| match_rule(elem, r))
        .collect()
}

fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|a, b| a.specificity.cmp(&b.specificity));

    for matched_rule in rules {
        for declaration in &matched_rule.rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    values
}
