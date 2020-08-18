#![feature(test)]

extern crate test;
use test::{Bencher};
use search_insert::{search_linear, search_quick};

#[bench]
fn bench_search_linear(b: &mut Bencher) {
    b.iter(|| {
        // Inner closure, the actual test
        for _i in 1..100 {
            let a = vec![1, 2, 3, 4, 5];
            let target = 3;
            search_linear(a, target);
        }
    });
}


#[bench]
fn bench_search_quicksort(b: &mut Bencher) {
    b.iter(|| {
        // Inner closure, the actual test
        for _i in 1..100 {
            let a = vec![1, 2, 3, 4, 5];
            let target = 3;
            search_quick(a, target);
        }
    });
}
