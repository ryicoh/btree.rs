#![feature(test)]
extern crate test;

use btree::Node;
use test::Bencher;

#[bench]
fn bench_split(b: &mut Bencher) {
    b.iter(|| {
        let mut node = Node::<i64>::new(999);
        node.items = (1..1000).collect();
        let (_, __, mut right) = node.split_three_items();
        for i in 0..500 {
            right.push(i);
        }
    });
}
