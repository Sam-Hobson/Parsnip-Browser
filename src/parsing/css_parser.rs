use crate::css::{Colour, Declaration, Rule, Selector, SimpleSelector, Unit, Value};
use crate::parsing::parser::{valid_standard_char, Parser};

pub struct CssParser {
    p: Parser,
}

impl CssParser {
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        // TODO: Make this not error prone.
        while !self.p.eof() {
            match self.p.next_char() {
                '#' => {
                    self.p.consume_char();
                    selector.id = Some(self.p.parse_standard_word());
                }
                '.' => {
                    self.p.consume_char();
                    selector.class.push(self.p.parse_standard_word());
                }
                '*' => {
                    self.p.consume_char();
                }
                c if valid_standard_char(c) => {
                    selector.tag_name = Some(self.p.parse_standard_word());
                }
                _ => break,
            }
        }

        selector
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.p.consume_whitespace();

            match self.p.next_char() {
                ',' => {
                    self.p.consume_char();
                    self.p.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }

        selectors.sort_by(|x, y| y.specificity().cmp(&x.specificity()));
        return selectors;
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.p.consume_char(), '{');
        let mut declarations = Vec::new();
        loop {
            self.p.consume_whitespace();
            if self.p.next_char() == '}' {
                self.p.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    fn parse_declaration(&mut self) -> Declaration {
        let key = self.p.parse_standard_word();
        self.p.consume_whitespace();
        assert_eq!(self.p.consume_char(), ':');
        self.p.consume_whitespace();
        let val = self.parse_value();
        self.p.consume_whitespace();
        assert_eq!(self.p.consume_char(), ';');

        Declaration {
            name: key,
            value: val,
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.p.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_colour(),
            _ => Value::Keyword(self.p.parse_standard_word()),
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        let s = self.p.consume_while(|c| matches!(c, '0'..='9' | '.'));
        s.parse().unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
        match &*self.p.parse_standard_word().to_lowercase() {
            "px" => Unit::Px,
            _ => panic!("Unrecognised unit!"),
        }
    }

    fn parse_colour(&mut self) -> Value {
        assert_eq!(self.p.consume_char(), '#');
        Value::ColourValue(Colour {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255,
        })
    }

    /// Parse two hexadecimal digits.
    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.p.input[self.p.pos..self.p.pos + 2];
        self.p.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }
}
