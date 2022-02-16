/// Temporary performance tests.
/// Full and detailed tests will be done later.
use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use weakheap::WeakHeap;
use std::collections::BinaryHeap;

fn basics(elements: Vec<String>) -> Vec<String> {
    let mut heap: BinaryHeap<String> = BinaryHeap::with_capacity(elements.len());
    for x in elements {
        heap.push(x);
    }
    let mut res = Vec::with_capacity(heap.len());
    while !heap.is_empty() {
        res.push(heap.pop().unwrap());
    }
    res
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| {
            basics(black_box(vec![
                String::from("truhdkiufyhnsryfsjnf"),
                String::from("gdsuhdjjjidjsdjdsjud"),
                String::from("5783uhffffffffffffffffff"),
                String::from("fhjsjfj"),
                String::from("gdsuhdjjj744748748"),
                String::from("46yfhjnamzndgansjehan"),
                String::from("truhdkiufyhnsryfsjnf"),
                String::from("gdsuhdjjjidjsdjdsjud"),
                String::from("5783uhffffffffffffffffff"),
                String::from("fhjsjfj"),
                String::from("gdsuhdjjj744748748"),
                String::from("46yfhjnamzndgansjehan"),
                String::from("truhdkiufyhnsryfsjnf"),
                String::from("gdsuhdjjjidjsdjdsjud"),
                String::from("5783uhffffffffffffffffff"),
                String::from("fhjsjfj"),
                String::from("gdsuhdjjj744748748"),
                String::from("46yfhjnamzndgansjehan"),
                String::from("truhdkiufyhnsryfsjnf"),
                String::from("gdsuhdjjjidjsdjdsjud"),
                String::from("5783uhffffffffffffffffff"),
                String::from("fhjsjfj"),
                String::from("gdsuhdjjj744748748"),
                String::from("46yfhjnamzndgansjehan"),
                String::from("truhdkiufyhnsryfsjnf"),
                String::from("gdsuhdjjjidjsdjdsjud"),
                String::from("5783uhffffffffffffffffff"),
                String::from("fhjsjfj"),
                String::from("gdsuhdjjj744748748"),
                String::from("46yfhjnamzndgansjehan"),
                String::from("truhdkiufyhnsryfsjnf"),
                String::from("gdsuhdjjjidjsdjdsjud"),
                String::from("5783uhffffffffffffffffff"),
                String::from("fhjsjfj"),
                String::from("gdsuhdjjj744748748"),
                String::from("46yfhjnamzndgansjehan"),
                String::from("truhdkiufyhnsryfsjnf"),
                String::from("gdsuhdjjjidjsdjdsjud"),
                String::from("5783uhffffffffffffffffff"),
                String::from("fhjsjfj"),
                String::from("gdsuhdjjj744748748"),
                String::from("46yfhjnamzndgansjehan"),
                String::from("truhdkiufyhnsryfsjnf"),
                String::from("gdsuhdjjjidjsdjdsjud"),
                String::from("5783uhffffffffffffffffff"),
                String::from("fhjsjfj"),
                String::from("gdsuhdjjj744748748"),
                String::from("46yfhjnamzndgansjehan"),
                String::from("truhdkiufyhnsryfsjnf"),
                String::from("gdsuhdjjjidjsdjdsjud"),
                String::from("5783uhffffffffffffffffff"),
                String::from("fhjsjfj"),
                String::from("gdsuhdjjj744748748"),
                String::from("46yfhjnamzndgansjehan"),
            ]))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
