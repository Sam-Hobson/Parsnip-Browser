mod dom;
mod parsing;
mod css;
mod style_tree;

use crate::parsing::html_parser::HtmlParser;

fn main() {
    let p = HtmlParser::parse(String::from("Hello world"));
    println!("{:?}", p);
}
