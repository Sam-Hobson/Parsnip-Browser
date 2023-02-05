mod dom;
mod parsing;
mod style;
mod box_model;

use crate::parsing::html_parser::HtmlParser;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./test/test.html").expect("Couldn't read file!");

    let p = HtmlParser::parse(file_contents);

    println!("File content:\n{p}");
}
