use std::collections::HashMap;
use crate::dom::Node;
use crate::box_model::Display;

/// Represents a set of styling rules. (Aka, an entires stylesheet or css file).
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

/// A rule is a set of declarations(characteristics) with a specifier that
/// is targeted. (Eg: div .salad {color: blue;})
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub struct MatchedRule<'a> {
    pub specificity: Specificity,
    pub rule: &'a Rule,
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
pub type PropertyMap = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColourValue(Colour),
}

#[derive(Debug, Clone)]
pub enum Unit {
    Px,
}

#[derive(Debug, Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// Hold's a nodes style data (and their children).
// # TODO: Merge styles into Node field (possibly).
pub struct StyledNode<'a> {
    pub node: &'a Node, // DOM node
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

/// TODO: Fix this garbage
impl StyledNode<'_> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(v)) => {
                match v.as_str() {
                    "block" => Display::Block,
                    "none" => Display::None,
                    _ => Display::Inline
                }
            }
            _ => Display::Inline
        }
    }
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
