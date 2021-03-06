#![feature(test)]

extern crate simple_nn;
extern crate test;

use test::Bencher;

use simple_nn::{nn, Matrix};

#[bench]
fn bench_functions_softmax(b: &mut Bencher) {
    let input = Matrix::<f64>::random(200, 1, -5.0, 5.0);
    b.iter(|| nn::functions::softmax(&input));
}

#[bench]
fn bench_functions_log_softmax(b: &mut Bencher) {
    let input = Matrix::<f64>::random(200, 1, -5.0, 5.0);
    b.iter(|| nn::functions::log_softmax(&input));
}
