mod parser;
mod context;

pub use parser::{ParseError, ParseResult};
pub use context::{Context, Queryable};
pub use context::QueryReturn;
pub use context::{EvalResult, EvalError, EvalFunc};


/// A parsed query ready for use with a `Context` and `Queryable`
#[derive(Debug)]
pub struct Query {

    // root token will be a scope
    tokens: Token,
}

/// AST tokens for the query string
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    /// Plain text with resolved escapes
    Text(String),

    /// A named variable
    ///
    /// Parameters: Name
    Variable(String),

    /// A named function that has arguments 
    ///
    /// Parameters: Name, Arguments
    Function(String, Vec<Token>),

    /// A level of scope
    Scope(Vec<Token>),
}
