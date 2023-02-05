use std::collections::{HashMap, HashSet};
use std::fmt;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    attributes: AttrMap,
}

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attributes: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes,
        }),
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classes) => classes.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

/// Formats a [Node] as a html tree of elements.
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn start_tag(n: &Node, indent_width: usize, num_indents: usize) -> String {
            let mut res = " ".repeat(indent_width).repeat(num_indents);
            res.push_str(n.node_type.to_string().as_str());
            res.push('\n');
            res
        }

        fn end_tag(n: &Node, indent_width: usize, num_indents: usize) -> String {
            let mut res = String::new();
            match &n.node_type {
                NodeType::Text(_) => res,
                NodeType::Element(e) => {
                    res.push_str(" ".repeat(indent_width).repeat(num_indents).as_str());
                    res.push_str("</");
                    res.push_str(e.tag_name.as_str());
                    res.push_str(">\n");
                    res
                }
            }
        }

        fn self_to_str(n: &Node, num_indents: usize) -> String {
            let indent_width = 4;

            let mut res = start_tag(n, indent_width, num_indents);

            for c in &n.children {
                res.push_str(self_to_str(c, num_indents + 1).as_str());
            }

            res.push_str(end_tag(n, indent_width, num_indents).as_str());
            res
        }

        writeln!(f, "{}", self_to_str(self, 0))
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(a) => write!(f, "{a}"),
            Self::Element(e) => write!(f, "{e}"),
        }
    }
}

impl fmt::Display for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::from("<");
        res.push_str(self.tag_name.as_str());

        let mut attrs = String::new();
        for i in &self.attributes {
            attrs.push(' ');
            attrs.push_str(i.0.as_str());
            attrs.push_str("=\"");
            attrs.push_str(i.1.as_str());
            attrs.push('\"');
        }

        res.push_str(attrs.as_str());
        res.push('>');

        write!(f, "{res}")
    }
}
