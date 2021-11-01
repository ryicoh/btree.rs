#![feature(test)]
extern crate test;

use btree::Btree;
use rand::{prelude::ThreadRng, Rng};
use std::collections;
use test::Bencher;

const BATCH_SIZE: usize = 100000;

fn gen_items() -> Vec<i64> {
    let mut rng: ThreadRng = rand::thread_rng();
    let v: Vec<i64> = (0..BATCH_SIZE as i64).collect();
    v.iter().map(|_| rng.gen()).collect()
}

#[bench]
fn bench_put_when_capacity_is_5(b: &mut Bencher) {
    let items = gen_items();
    b.iter(|| {
        let mut btree = Btree::<i64>::new(5);
        for i in 0..BATCH_SIZE {
            btree.put(items[i]);
        }
    })
}

#[bench]
fn bench_put_when_capacity_is_100(b: &mut Bencher) {
    let items = gen_items();
    b.iter(|| {
        let mut btree = Btree::<i64>::new(100);
        for i in 0..BATCH_SIZE {
            btree.put(items[i]);
        }
    })
}

#[bench]
fn bench_put_when_capacity_is_1000(b: &mut Bencher) {
    let items = gen_items();
    b.iter(|| {
        let mut btree = Btree::<i64>::new(1000);
        for i in 0..BATCH_SIZE {
            btree.put(items[i]);
        }
    })
}

#[bench]
fn bench_put_when_capacity_is_63(b: &mut Bencher) {
    let items = gen_items();
    b.iter(|| {
        let mut btree = Btree::<i64>::new(63);
        for i in 0..BATCH_SIZE {
            btree.put(items[i]);
        }
    })
}

#[bench]
fn bench_std_put_when_capacity_is(b: &mut Bencher) {
    let items = gen_items();
    b.iter(|| {
        let mut btree = collections::BTreeSet::new();
        for i in 0..BATCH_SIZE {
            btree.insert(items[i]);
        }
    })
}
