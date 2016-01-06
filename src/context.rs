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
    fn get_func(&self, name: &str) -> Option<&Box<(Fn(&Vec<Token>) -> QueryReturn)>>;
}

impl Query {
    pub fn run(&self, queryable: &Queryable, context: &Context) -> Result<String, ()> {
        let mut output = String::new();
        for token in &self.tokens {
            let result = eval_token(token, queryable, context);
            output.push_str(&result.text);
        }
        Ok(output)
    }
}

fn eval_token(token: &Token, queryable: &Queryable, context: &Context) -> QueryReturn {
    match token {
        &Token::Text(ref t) => QueryReturn{ text: t.to_owned(), condition: None },
        &Token::Variable(ref v) => {
            match queryable.query(v) {
                Some(r) => QueryReturn{ text: r, condition: Some(true) },
                None => QueryReturn{ text: String::new(), condition: Some(false) },
            }
        },
        &Token::Function(ref f, ref arg) => {
            match context.get_func(f) {
                Some(func) => {
                    func(arg)
                },
                None => {
                    panic!();
                }
            }
        }
        _ => panic!(),
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

impl Context for HashMap<String, Box<(Fn(&Vec<Token>) -> QueryReturn)>> {
    fn get_func(&self, name: &str) -> Option<&Box<(Fn(&Vec<Token>) -> QueryReturn)>> {
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
        
        let mut func = HashMap::<String, Box<(Fn(&Vec<Token>) -> QueryReturn)>>::new();

        let result = Query::parse("Hello %name%".to_owned()).unwrap()
                     .run(&map, &func).unwrap();

        assert_eq!("Hello Dave".to_owned(), result);
    }
}
