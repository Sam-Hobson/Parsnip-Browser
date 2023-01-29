use std::collections::HashMap;
use crate::parsing::parser::Parser;
use crate::dom;

#[derive(Debug)]
pub struct HtmlParser {
    p: Parser,
}

impl HtmlParser {
    /// Parses a tag name, which can contain 'a'-'z', 'A'-'Z', '0'-'9'.
    fn parse_tag_name(&mut self) -> String {
        let ranges = [('a', 'z'), ('A', 'Z'), ('0', '9')];

        self.p.consume_while(|x| {
            ranges
                .iter()
                .fold(false, |acc, (lo, hi)| acc || ((&x >= lo) && (&x <= hi)))
        })
    }

    /// Parses a node.
    fn parse_node(&mut self) -> dom::Node {
        match self.p.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.p.consume_while(|x| x != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // Parse the opening tag:
        assert!(self.p.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        assert!(self.p.consume_char() == '>');

        // Parse all the children:
        let children = self.parse_nodes();

        // Parse closing tag:
        assert!(self.p.consume_char() == '<');
        assert!(self.p.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.p.consume_char() == '>');

        dom::elem(tag_name, attributes, children)
    }

    fn parse_attribute(&mut self) -> (String, String) {
        let key = self.parse_tag_name();
        assert!(self.p.consume_char() == '=');
        let val = self.parse_attribute_value();

        (key, val)
    }

    fn parse_attribute_value(&mut self) -> String {
        let open_quote = self.p.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let val = self.p.consume_while(|x| x != open_quote);
        assert!(self.p.consume_char() == open_quote);

        val
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();

        loop {
            self.p.consume_whitespace();
            if self.p.next_char() == '>' {
                break;
            }
            let (key, val) = self.parse_attribute();
            attributes.insert(key, val);
        }

        attributes
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();

        loop {
            self.p.consume_whitespace();
            if self.p.eof() || self.p.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node())
        }

        nodes
    }

    pub fn parse(s: String) -> dom::Node {
        let mut nodes = HtmlParser {
            p: Parser { pos: 0, input: s },
        }
        .parse_nodes();

        // If the document contains a root element, return it, else create one.
        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            dom::elem(String::from("html"), HashMap::new(), nodes)
        }
    }
}
