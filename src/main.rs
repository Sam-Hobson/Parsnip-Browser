mod dom;
mod parsing;
mod css;

use crate::parsing::html_parser::HtmlParser;

fn main() {
    let p = HtmlParser::parse(String::from("Hello world"));

    println!("{:?}", p);
}
