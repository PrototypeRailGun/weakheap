/// Temporary performance tests.
/// Full and detailed tests will be done later.
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use weakheap::WeakHeap;
//use std::collections::BinaryHeap;

fn basics(elements: Vec<String>) -> Vec<String> {
    let mut heap: WeakHeap<String> = WeakHeap::with_capacity(elements.len());
    for x in elements.clone() {
        heap.push(x);
    }
    for x in elements {
        heap.pushpop(x);
    }

    heap.into_sorted_vec()
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
