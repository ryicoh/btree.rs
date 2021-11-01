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

fn put_items(btree: &mut Btree<i64>) -> Vec<i64> {
    let items = gen_items();
    for i in 0..BATCH_SIZE {
        btree.put(items[i].clone());
    }
    items
}

#[bench]
fn bench_get_when_capacity_is_5(b: &mut Bencher) {
    let mut btree = Btree::<i64>::new(5);
    let items = put_items(&mut btree);

    b.iter(|| {
        for i in 0..BATCH_SIZE {
            btree.get(&items[i]);
        }
    })
}

#[bench]
fn bench_get_when_capacity_is_100(b: &mut Bencher) {
    let mut btree = Btree::<i64>::new(100);
    let items = put_items(&mut btree);

    b.iter(|| {
        for i in 0..BATCH_SIZE {
            btree.get(&items[i]);
        }
    })
}

#[bench]
fn bench_get_when_capacity_is_1000(b: &mut Bencher) {
    let mut btree = Btree::<i64>::new(1000);
    let items = put_items(&mut btree);

    b.iter(|| {
        for i in 0..BATCH_SIZE {
            btree.get(&items[i]);
        }
    })
}

#[bench]
fn bench_get_when_capacity_is_63(b: &mut Bencher) {
    let mut btree = Btree::<i64>::new(63);
    let items = put_items(&mut btree);

    b.iter(|| {
        for i in 0..BATCH_SIZE {
            btree.get(&items[i]);
        }
    })
}

#[bench]
fn bench_std_get(b: &mut Bencher) {
    let mut btree = collections::BTreeSet::new();
    let items = gen_items();
    for i in 0..BATCH_SIZE {
        btree.insert(items[i].clone());
    }

    b.iter(|| {
        for i in 0..BATCH_SIZE {
            btree.get(&items[i]);
        }
    })
}
