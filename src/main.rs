mod dom;
mod dom_fmt;
mod parsing;
mod style;

use crate::parsing::html_parser::HtmlParser;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./test.html").expect("Couldn't read file!");

    let p = HtmlParser::parse(file_contents);

    println!("File content:\n{p}");
}
