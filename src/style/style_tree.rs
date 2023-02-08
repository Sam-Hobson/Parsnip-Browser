use crate::dom::{ElementData, Node, NodeType};
use crate::style::css::{
    MatchedRule, PropertyMap, Rule, Selector, SimpleSelector, StyledNode, Stylesheet,
};

/// Returns whether a [Selector] matches a given element.
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

/// Returns whether a [SimpleSelector] matches a given element.
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

/// Returns the [MatchedRule]s that match a given element within a given [Stylesheet].
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|r| match_rule(elem, r))
        .collect()
}

/// Returns a map of properties for a given element, sorted in order of [specificity].
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = PropertyMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Most strongly specified rules go first.
    rules.sort_by(|a, b| a.specificity.cmp(&b.specificity));

    // Copy declarations into output.
    for matched_rule in rules {
        for declaration in &matched_rule.rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    values
}

// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => PropertyMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}
