use std::fmt;
use crate::dom::{Node, NodeType, ElementData};

/// Formats a [Node] as a html tree of elements.
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn self_to_str(n: &Node, indent: usize) -> String {
            let mut res = "  ".repeat(indent);
            res.push_str(n.node_type.to_string().as_str());
            res.push('\n');

            for c in &n.children {
                res.push_str(self_to_str(c, indent + 1).as_str());
            }

            match &n.node_type {
                NodeType::Text(_) => res,
                NodeType::Element(e) => {
                    res.push_str("  ".repeat(indent).as_str());
                    res.push_str("</");
                    res.push_str(e.tag_name.as_str());
                    res.push_str(">\n");
                    res
                }
            }
        }

        writeln!(f, "{}", self_to_str(self, 0))
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(a) => write!(f, "{a}"),
            Self::Element(e) => write!(f, "{e}")
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

