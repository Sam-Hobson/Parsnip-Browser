#[derive(Debug)]
pub struct Parser {
    pub pos: usize,
    pub input: String,
}

impl Parser {
    /// Returns the next char
    pub fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Do the chars match the current position in string?
    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    pub fn string(&mut self, s: &str) -> bool {
        if self.starts_with(s) {
            self.pos += s.len();
            return true
        }
        false
    }

    /// Is there any input left to process?
    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Consume a single character from the input.
    /// Returns: The next character.
    pub fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, current) = iter.next().unwrap();
        let (off, _) = iter.next().unwrap_or((1, ' '));
        self.pos += off;

        current
    }

    /// Consumes characters while a given condition is met.
    pub fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
        let mut res = String::new();

        while !self.eof() && test(self.next_char()) {
            res.push(self.consume_char());
        }

        res
    }

    /// Consumes all whitespace from the start of the input.
    pub fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Parses a tag name, which can contain the characters allowed in [valid_standard_char].
    pub fn parse_standard_word(&mut self) -> String {
        self.consume_while(standard_char)
    }

    /// Parse a string that is between two characters
    pub fn parse_between(&mut self, c: char) -> String {
        assert!(self.next_char() == c);
        self.consume_char();
        let res = self.consume_while(|x| x != c);
        assert!(self.next_char() == c);
        self.consume_char();

        res
    }
}

/// Check if a character is in the range 'a'-'z', 'A'-'Z', or '0'-'9'.
pub fn standard_char(c: char) -> bool {
    let ranges = [('a', 'z'), ('A', 'Z'), ('0', '9')];
    ranges
        .iter()
        .fold(false, |acc, (lo, hi)| acc || ((&c >= lo) && (&c <= hi)))
}
