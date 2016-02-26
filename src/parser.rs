use Query;
use Token;
use std::str::Chars;

// consts for character representing tokens
const TOKEN_VAR_START: char = '%';
const TOKEN_VAR_END: char = '%';
const TOKEN_FUNC_NAME_START: char = '$';
const TOKEN_FUNC_PARAM_START: char = '(';
const TOKEN_FUNC_PARAM_END: char = ')';
const TOKEN_FUNC_PARAM_SEP: char = ',';
const TOKEN_ESCAPE: char = '\\';

// TODO: Proper error type instead of String
pub type ParseResult<T> = Result<T, ParseError>;

/// Errors found in parsing query
#[derive(Debug)]
pub enum ParseError {
    /// Unknown escape sequence.
    /// String contains the sequece
    UnknownEscape(String),

    /// Escape squence at end of query
    EscapeAtEndOfQuery,

    /// Variable is missing closing character 
    VariableMissingClosing,

    /// Function is missing start of parameters
    FuncMissingParameter,

    /// Function parameters is missing closing character
    FuncParameterNotClosed,

    /// End of Query
    EndOfQuery,
}

impl Query {
    pub fn parse(src: String) -> ParseResult<Query> {
        let mut iter = src.chars();
        match parse_scope(&mut iter) {
            Ok(t) => Ok(Query { tokens: t }),
            Err(e) => return Err(e),
        }
    }
}

fn parse_scope(iter: &mut Chars) -> ParseResult<Token> {
    let mut tokens = Vec::new();
    loop {
        let token = parse_expr(iter);
        tokens.push(match token { 
            Ok(t) => t,
            Err(e) => {
                match e {
                    ParseError::EndOfQuery => break,
                    _ => return Err(e),
                }
            }
        });
    }
    Ok(Token::Scope(tokens))
}

fn parse_expr(iter: &mut Chars) -> ParseResult<Token> {
    match iter.clone().peekable().peek() {
        Some(c) => {
            return match *c {
                TOKEN_VAR_START => return parse_variable(iter),
                TOKEN_FUNC_NAME_START => return parse_function(iter),
                _ => return parse_text(iter),
            }
        }
        None => return Err(ParseError::EndOfQuery),
    }
}

fn parse_text(iter: &mut Chars) -> ParseResult<Token> {
    let mut text = String::new();

    loop {
        // Peek to check for next expression
        if let Some(peek) = iter.clone().peekable().peek() {
            match *peek {
                TOKEN_VAR_START => break,
                TOKEN_FUNC_PARAM_SEP => break,
                TOKEN_FUNC_NAME_START => break, 
                TOKEN_FUNC_PARAM_START => break, 
                TOKEN_FUNC_PARAM_END => break,
                _ => (),
            }
        }

        if let Some(next) = iter.next() {
            match next {
                TOKEN_ESCAPE => {
                    text.push(match parse_escape(iter) {
                        Ok(esc) => esc,
                        Err(err) => return Err(err),
                    })
                }
                _ => text.push(next),
            }
        } else {
            // Made it to the end of the string
            break;
        }
    }
    Ok(Token::Text(text))
}

fn parse_escape(iter: &mut Chars) -> ParseResult<char> {
    if let Some(next) = iter.next() {
        Ok(match next {
            'n' => '\n',
            't' => '\t',
            TOKEN_ESCAPE => TOKEN_ESCAPE,
            TOKEN_VAR_START => TOKEN_VAR_START,
            TOKEN_FUNC_NAME_START => TOKEN_FUNC_NAME_START,
            TOKEN_FUNC_PARAM_SEP => TOKEN_FUNC_PARAM_SEP,
            _ => return Err(ParseError::UnknownEscape(format!("\\{}", next))),
        })
    } else {
        Err(ParseError::EscapeAtEndOfQuery)
    }
}

fn parse_variable(iter: &mut Chars) -> ParseResult<Token> {
    let _ = iter.next(); // ignore TOKEN_VAR_START

    let mut name = String::new();
    while let Some(c) = iter.next() {
        match c {
            TOKEN_VAR_END => return Ok(Token::Variable(name)),
            _ => name.push(c),
        };
    }
    Err(ParseError::VariableMissingClosing)
}

fn parse_function(iter: &mut Chars) -> ParseResult<Token> {
    let _ = iter.next(); // ignore TOKEN_FUNC_NAME_START
    let mut name = String::new();
    let mut args = Vec::<Token>::new();

    // get the name of the function
    loop {
        if let Some(c) = iter.next() {
            match c { // TODO: enforce name limits here
                TOKEN_FUNC_PARAM_START => break,
                _ => name.push(c),
            };
        } else {
            return Err(ParseError::FuncMissingParameter);
        }
    }

    // get the parameters of the function
    while let Some(c) = iter.clone().peekable().peek() {
        match *c { // TODO: parse tokens seperated by commas
            TOKEN_FUNC_PARAM_END => {
                let _ = iter.next();
                return Ok(Token::Function(name, args));
            }
            TOKEN_FUNC_PARAM_SEP => {
                let _ = iter.next();
                continue;
            }
            _ => {
                args.push(match parse_expr(iter) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                })
            }
        }
    }

    Err(ParseError::FuncParameterNotClosed)
}

#[cfg(test)]
mod tests {
    use ::Token::*;
    use ParseError;
    use Query;

    macro_rules! parse_test {
        ($name: ident, $input: expr, $out: expr) => {
            #[test]
            fn $name() {
                let out = Query::parse($input.to_owned()).unwrap();
                assert_eq!(out.tokens, Scope(vec![$out]));
            }
        };
        ($name: ident, $input: expr, $out: expr, $($more: expr),*) => {
            #[test]
            fn $name() {
                let out = Query::parse($input.to_owned()).unwrap();
                assert_eq!(out.tokens, Scope(vec![$out $(, $more)*]));
            }
        };
    }

    macro_rules! parse_fail_test {
        ($name: ident, $input: expr, $expt: pat) => {
            #[test]
            fn $name() {
                let out = Query::parse($input.to_owned());
                assert!(out.is_err());
                match out {
                    Ok(_) => unreachable!(),
                    Err(e) => match e {
                        $expt => (),
                        _ => unreachable!(),
                    }
                }
            }
        };
    }

    #[test]
    fn parse_empty() {
        let out = Query::parse("".to_owned()).unwrap();
        assert_eq!(out.tokens, Scope(vec![]));
    }

    // Single tokens tests
    parse_test!(parse_variable, "%hello%", Variable("hello".to_owned()));
    parse_test!(parse_text, "hello", Text("hello".to_owned()));
    parse_test!(parse_escape, "\\t", Text("\t".to_owned()));
    parse_test!(parse_func, "$func()", Function("func".to_owned(), vec![]));
    parse_test!(parse_func_param,
                "$func(expr, expr)",
                Function("func".to_owned(),
                         vec![Text("expr".to_owned()), Text(" expr".to_owned())]));

    parse_test!(parse_func_rec_params,
                "$i($f())",
                Function("i".to_owned(), vec![Function("f".to_owned(), vec![])]));

    // Multi token tests
    parse_test!(parse_text_and_variable,
                "Hello %name%",
                Text("Hello ".to_owned()),
                Variable("name".to_owned()));
    parse_test!(parse_text_var_func,
                "Hello %name% $f()",
                Text("Hello ".to_owned()),
                Variable("name".to_owned()),
                Text(" ".to_owned()),
                Function("f".to_owned(), vec![]));

    parse_fail_test!(parse_fail_miss_matched_variable,
                     "%hello",
                     ParseError::VariableMissingClosing);
    parse_fail_test!(parse_fail_unknown_escape, "\\?", ParseError::UnknownEscape(_));

    parse_fail_test!(parse_fail_end_with_escape,
                     "\\",
                     ParseError::EscapeAtEndOfQuery);
    parse_fail_test!(parse_fail_func_no_para,
                     "$func",
                     ParseError::FuncMissingParameter);
    parse_fail_test!(parse_fail_func_no_close_para,
                     "$func(",
                     ParseError::FuncParameterNotClosed);

}
