use super::dom;
use std::collections::HashMap;

struct Parser {
    pos: usize,
    input: String,
}

pub struct HtmlParser {
    p: Parser,
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
}

impl HtmlParser {
    fn parse_tag_name(&mut self) -> String {
        let _ranges = [('a', 'z'), ('A', 'Z'), ('0', '0')];

        self.p.consume_while(|x| {
            _ranges
                .iter()
                .fold(false, |acc, (lo, hi)| acc || ((&x >= lo) && (&x <= hi)))
        })
    }

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

        return (key, val);
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
        let mut nodes: Vec<dom::Node> = HtmlParser {
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
