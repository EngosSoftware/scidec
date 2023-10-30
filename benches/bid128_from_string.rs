//! Benchmarks.

#![feature(test)]

extern crate test;

use scidec::bid128_from_string;
use test::Bencher;

const INPUTS: [&str; 6] = [
  "12",
  "938475E-03",
  "0.00003E-02",
  "9999999999999999999999999999999999",
  "+9999999999999999999999999999999999e+6111",
  "+1000000000000000000000000000000000e-6176",
];

#[bench]
fn bench_bid128_from_string_00(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[0]);
  });
}

#[bench]
fn bench_bid128_from_string_01(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[1]);
  });
}

#[bench]
fn bench_bid128_from_string_02(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[2]);
  });
}

#[bench]
fn bench_bid128_from_string_03(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[3]);
  });
}

#[bench]
fn bench_bid128_from_string_04(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[4]);
  });
}

#[bench]
fn bench_bid128_from_string_05(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[5]);
  });
}
