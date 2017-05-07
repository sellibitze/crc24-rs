#![feature(test)]

extern crate crc24;
extern crate test;

use crc24::hash_raw;
use test::Bencher;

#[bench]
fn bench_hash_raw(b: &mut Bencher) {
    b.iter(|| hash_raw(&[0xA5u8; 10_000]));
}
