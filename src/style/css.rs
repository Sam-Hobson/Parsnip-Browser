use crate::box_model::Display;
use crate::dom::Node;
use std::collections::HashMap;

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

/// A rule stored with it s assosiated specificity.
pub struct MatchedRule<'a> {
    pub specificity: Specificity,
    pub rule: &'a Rule,
}

impl<'a> MatchedRule<'a> {
    pub fn new(specificity: Specificity, rule: &'a Rule) -> Self {
        Self { specificity, rule }
    }
}

/// Different types of selectors for a css rule. TODO: More detail
pub enum Selector {
    Simple(SimpleSelector),
}

/// A simple selector for a rule.
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

/// A key-value pair of a css attribute. Eg: display: none;
/// TODO: Link
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

// Map CSS properties to values.
pub type PropertyMap = HashMap<String, Value>;

/// What type of value is accepted for a declaration?
#[derive(Debug, Clone)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColourValue(Colour),
}

impl Value {
    /// Return the size of a length in px, or zero for non-lengths.
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Unit {
    Px,
}

/// Colour in rgba
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
impl<'a> StyledNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    /// Returns the "display" attribute of the [StyledNode].
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(v)) => match v.as_str() {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }

    pub fn lookup(&self, key: &str, key_2: &str, default_val: &Value) -> Value {
        self.value(key)
            .unwrap_or_else(|| self.value(key_2).unwrap_or_else(|| default_val.clone()))
    }
}

/// Labels the specifier for a node. Stores id count, class count, and then tag count.
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
