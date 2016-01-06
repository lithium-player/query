mod parser;
mod context;

pub use context::Context;
pub use context::Queryable;

#[derive(Debug)]
pub struct Query {
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    /// Plain text with resolved escapes
    Text(String),
    /// A named variable with: name
    Variable(String),
    /// A named function with: Name, Expressions
    Function(String, Vec<Token>),
    None,
}
