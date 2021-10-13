pub mod date;
pub mod model;
pub mod parser;

pub fn get_parser() -> impl parser::Parser {
    parser::quick::Client {}
}
