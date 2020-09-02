#![feature(test)]

extern crate test;
use test::{Bencher};
use perfect_square::{is_perfect_square_rec, is_perfect_square_it};

#[bench]
fn bench_rec(b: &mut Bencher) {
    b.iter(|| {
        // Inner closure, the actual test
        for _i in 1..100 {
            let a = 808201;
            is_perfect_square_rec(a, 1, a);
        }
    });
}

#[bench]
fn bench_it(b: &mut Bencher) {
    b.iter(|| {
        // Inner closure, the actual test
        for _i in 1..100 {
            let a = 808201;
            is_perfect_square_it(a);
        }
    });
}
    
    