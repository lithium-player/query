mod parser;
mod context;

pub use parser::{ParseError, ParseResult};
pub use context::{Context, Queryable};
pub use context::QueryReturn;
pub use context::{EvalResult, EvalError, EvalFunc};


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
}
