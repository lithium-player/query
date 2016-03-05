#![feature(test)]

extern crate liquery;
extern crate test;

use test::Bencher;
use liquery::Query;

#[bench]
fn bench_parse_simple_text(b: &mut Bencher) {
    b.iter(move || {
        let _ = Query::parse("HelloWorld".to_owned());
    });
}

#[bench]
fn bench_parse_simple_variable(b: &mut Bencher) {
    b.iter(move || {
        let _ = Query::parse("%variable%".to_owned());
    });
}

#[bench]
fn bench_parse_simple_function(b: &mut Bencher) {
    b.iter(move || {
        let _ = Query::parse("$functio()".to_owned());
    });
}

#[bench]
fn bench_parse_nested_function(b: &mut Bencher) {
    b.iter(move || {
        let _ = Query::parse("$func($func($func($func())))".to_owned());
    });
}

#[bench]
fn bench_parse_simple_query(b: &mut Bencher) {
    b.iter(move || {
        let _ = Query::parse("Hello $name() %time%".to_owned());
    });
}
