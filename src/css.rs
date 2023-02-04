use std::collections::HashMap;
use crate::dom::Node;

pub struct Stylesheet {
    rules: Vec<Rule>,
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub struct MatchedRule<'a> {
    specificity: Specificity,
    rule: &'a Rule,
}

impl<'a> MatchedRule<'a> {
    pub fn new(specificity: Specificity, rule: &'a Rule) -> Self {
        Self { specificity, rule }
    }
}

pub enum Selector {
    Simple(SimpleSelector),
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

pub struct Declaration {
    pub name: String,
    pub value: Value,
}

// Map CSS properties to values.
type PropertyMap = HashMap<String, Value>;

// Hold's a nodes style data (and their children).
// # TODO: Merge styles into Node field (possibly).
struct StyleNode<'a> {
    node: &'a Node, // DOM node
    specified_values: PropertyMap,
    children: Vec<StyleNode<'a>>,
}

pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColourValue(Colour),
}

pub enum Unit {
    Px,
}

pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}
