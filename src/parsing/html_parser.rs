use std::collections::HashMap;
use crate::parsing::parser::Parser;
use crate::dom;

#[derive(Debug)]
pub struct HtmlParser {
    p: Parser,
}

impl HtmlParser {
    /// Parses a node.
    fn parse_node(&mut self) -> dom::Node {
        match self.p.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    /// Parse text until a tag appears.
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.p.consume_while(|x| x != '<'))
    }

    /// Parse an element (tag).
    fn parse_element(&mut self) -> dom::Node {
        // Parse the opening tag:
        assert!(self.p.consume_char() == '<');
        let tag_name = self.p.parse_standard_word();
        let attributes = self.parse_attributes();
        assert!(self.p.consume_char() == '>');

        // Parse all the children:
        let children = self.parse_nodes();

        // Parse closing tag:
        assert!(self.p.consume_char() == '<');
        assert!(self.p.consume_char() == '/');
        assert!(self.p.parse_standard_word() == tag_name);
        assert!(self.p.consume_char() == '>');

        dom::elem(tag_name, attributes, children)
    }

    /// Parses an attribute of an element/tag.
    fn parse_attribute(&mut self) -> (String, String) {
        let key = self.p.parse_standard_word();
        assert!(self.p.consume_char() == '=');
        let val = self.parse_attribute_value();

        (key, val)
    }

    /// Parses an attribute value.
    /// TODO: Replace with parse_between
    fn parse_attribute_value(&mut self) -> String {
        let open_quote = self.p.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let val = self.p.consume_while(|x| x != open_quote);
        assert!(self.p.consume_char() == open_quote);

        val
    }

    /// Parses multiple attributes.
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

    /// Parse all nodes in the DOM.
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

    /// Parse a [String] of html code.
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
