use Query;
use Token;

use std::collections::HashMap;

pub struct QueryReturn {
    text: String,
    condition: Option<bool>,
}

pub trait Queryable {
    fn query(&self, key: &str) -> Option<String>;
}

pub trait Context {
    fn get_func(&self, name: &str) -> Option<&Box<EvalFunc>>;
}

/// Errors found in evaluating query
#[derive(Debug)]
pub enum EvalError {
    /// Function used is not in the current context
    /// String passed is the function name
    FunctionNotFound(String),
}

pub type EvalResult<T> = Result<T, EvalError>;
pub type EvalFunc = (Fn(&Vec<Token>) -> EvalResult<QueryReturn>);

impl Query {
    /// Evaluate the query with a queryable object to be based off and a
    /// context
    pub fn eval(&self, queryable: &Queryable, context: &Context) -> EvalResult<String> {
        let mut output = String::new();
        for token in &self.tokens {
            let result = match eval_token(token, queryable, context) {
                Ok(txt) => txt,
                Err(err) => return Err(err),
            };
            output.push_str(&result.text);
        }
        Ok(output)
    }
}

fn eval_token(token: &Token, queryable: &Queryable, context: &Context) -> EvalResult<QueryReturn> {
    match token {
        &Token::Text(ref t) => {
            Ok(QueryReturn {
                text: t.to_owned(),
                condition: None,
            })
        }
        &Token::Variable(ref v) => {
            match queryable.query(v) {
                Some(r) => {
                    Ok(QueryReturn {
                        text: r,
                        condition: Some(true),
                    })
                }
                None => {
                    Ok(QueryReturn {
                        text: String::new(),
                        condition: Some(false),
                    })
                }
            }
        }
        &Token::Function(ref f, ref arg) => {
            match context.get_func(f) {
                Some(func) => func(arg),
                None => {
                    return Err(EvalError::FunctionNotFound(f.to_owned()));
                }
            }
        }
    }
}

impl Queryable for HashMap<String, String> {
    fn query(&self, query: &str) -> Option<String> {
        match self.get(query) {
            Some(ans) => Some(ans.to_owned()),
            None => None,
        }
    }
}

impl Context for HashMap<String, Box<(Fn(&Vec<Token>) -> EvalResult<QueryReturn>)>> {
    fn get_func(&self, name: &str) -> Option<&Box<(Fn(&Vec<Token>) -> EvalResult<QueryReturn>)>> {
        self.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Query;
    use Token;
    use std::collections::HashMap;

    #[test]
    fn test_run() {

        let mut map = HashMap::new();
        map.insert("name".to_owned(), "Dave".to_owned());

        let mut func = HashMap::<String, Box<(Fn(&Vec<Token>) -> EvalResult<QueryReturn>)>>::new();

        let result = Query::parse("Hello %name%".to_owned())
                         .unwrap()
                         .eval(&map, &func)
                         .unwrap();

        assert_eq!("Hello Dave".to_owned(), result);
    }
}
