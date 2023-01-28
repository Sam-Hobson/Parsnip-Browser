use super::dom;
use std::collections::HashMap;

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    // Returns the next char
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Do the chars match the current position in string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // Is there any input left to process?
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, current) = iter.next().unwrap();
        let (off, _) = iter.next().unwrap_or((1, ' '));
        self.pos += off;

        current
    }

    fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
        let mut res = String::new();

        while !self.eof() && test(self.next_char()) {
            res.push(self.consume_char());
        }

        res
    }

    fn consume_whitespace(&mut self) -> () {
        self.consume_while(char::is_whitespace);
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|x| {
            (('a' <= x) && (x <= 'z')) || (('A' <= x) && (x <= 'Z')) || (('0' <= x) && (x <= '9'))
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|x| x != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // Parse the opening tag:
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Parse all the children:
        let children = self.parse_nodes();

        // Parse closing tag:
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        dom::elem(tag_name, attributes, children)
    }

    fn parse_attribute(&mut self) -> (String, String) {
        let key = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let val = self.parse_attribute_value();

        return (key, val);
    }

    fn parse_attribute_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let val = self.consume_while(|x| x != open_quote);
        assert!(self.consume_char() == open_quote);

        val
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();

        loop {
            self.consume_whitespace();

            if self.next_char() == '>' {
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
            self.consume_whitespace();

            if self.eof() || self.starts_with("</") {
                break;
            }

            nodes.push(self.parse_node())
        }

        nodes
    }
}
