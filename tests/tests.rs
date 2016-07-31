extern crate liquery;

use liquery::Query;

#[test]
fn parse_query_with_from_str() {
    let _ = "Hello World".parse::<Query>();
}
