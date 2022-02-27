mod data;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use data::{long_comp_strings, WORDS};
use std::collections::BinaryHeap;
use weakheap::WeakHeap;

fn get_words(count: usize) -> Vec<String> {
    WORDS[0..count].iter().map(|&w| w.to_string()).collect()
}

fn weakheap_sort(size: usize) -> Vec<String> {
    let heap = WeakHeap::from(get_words(size));
    heap.into_sorted_vec()
}

fn weakheap_push_pop(size: usize) -> Vec<String> {
    let mut heap = WeakHeap::with_capacity(size * 2);
    let data = get_words(size);
    for w in data {
        heap.push(w);
        let x = heap.pop().unwrap();
        heap.push(x);
    }
    let mut sorted = Vec::with_capacity(size);
    while let Some(w) = heap.pop() {
        sorted.push(w);
    }
    sorted
}

fn weakheap_append(size: usize) -> Vec<String> {
    let mut heap1 = WeakHeap::from(get_words(size / 2));
    let mut heap2 = WeakHeap::from(get_words(size / 2));
    heap1.append(&mut heap2);
    heap1.into_vec()
}

fn weakheap_longcomp(_size: usize) -> Vec<String> {
    let heap = WeakHeap::from(long_comp_strings());
    heap.into_sorted_vec()
}

fn binheap_sort(size: usize) -> Vec<String> {
    let heap = BinaryHeap::from(get_words(size));
    heap.into_sorted_vec()
}

fn binheap_push_pop(size: usize) -> Vec<String> {
    let mut heap = BinaryHeap::with_capacity(size * 2);
    let data = get_words(size);
    for w in data {
        heap.push(w);
        let x = heap.pop().unwrap();
        heap.push(x);
    }
    let mut sorted = Vec::with_capacity(size);
    while let Some(w) = heap.pop() {
        sorted.push(w);
    }
    sorted
}

fn binheap_append(size: usize) -> Vec<String> {
    let mut heap1 = BinaryHeap::from(get_words(size / 2));
    let mut heap2 = BinaryHeap::from(get_words(size / 2));
    heap1.append(&mut heap2);
    heap1.into_vec()
}

fn binheap_longcomp(_size: usize) -> Vec<String> {
    let heap = BinaryHeap::from(long_comp_strings());
    heap.into_sorted_vec()
}

fn quicksort(size: usize) -> Vec<String> {
    let mut vec = get_words(size);
    vec.sort_unstable();
    vec
}

fn bench_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting");

    for i in 1..=9 {
        let size = i * 100;
        group.bench_with_input(BenchmarkId::new("Binary Heap", size), &size, |b, s| {
            b.iter(|| binheap_sort(*s))
        });
        group.bench_with_input(BenchmarkId::new("Weak Heap", size), &size, |b, s| {
            b.iter(|| weakheap_sort(*s))
        });
        group.bench_with_input(BenchmarkId::new("Quicksort", size), &size, |b, s| {
            b.iter(|| quicksort(*s))
        });
    }

    for i in 2..=8 {
        let size = i * 500;
        group.bench_with_input(BenchmarkId::new("Binary Heap", size), &size, |b, s| {
            b.iter(|| binheap_sort(*s))
        });
        group.bench_with_input(BenchmarkId::new("Weak Heap", size), &size, |b, s| {
            b.iter(|| weakheap_sort(*s))
        });
        group.bench_with_input(BenchmarkId::new("Quicksort", size), &size, |b, s| {
            b.iter(|| quicksort(*s))
        });
    }

    group.finish();
}

fn bench_basics(c: &mut Criterion) {
    let mut group = c.benchmark_group("Push & Pop");

    for i in 1..=9 {
        let size = i * 100;
        group.bench_with_input(BenchmarkId::new("Binary Heap", size), &size, |b, s| {
            b.iter(|| binheap_push_pop(*s))
        });
        group.bench_with_input(BenchmarkId::new("Weak Heap", size), &size, |b, s| {
            b.iter(|| weakheap_push_pop(*s))
        });
    }

    for i in 2..=8 {
        let size = i * 500;
        group.bench_with_input(BenchmarkId::new("Binary Heap", size), &size, |b, s| {
            b.iter(|| binheap_push_pop(*s))
        });
        group.bench_with_input(BenchmarkId::new("Weak Heap", size), &size, |b, s| {
            b.iter(|| weakheap_push_pop(*s))
        });
    }

    group.finish();
}

fn bench_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("Append");

    for i in 1..=9 {
        let size = i * 100;
        group.bench_with_input(BenchmarkId::new("Binary Heap", size), &size, |b, s| {
            b.iter(|| binheap_append(*s))
        });
        group.bench_with_input(BenchmarkId::new("Weak Heap", size), &size, |b, s| {
            b.iter(|| weakheap_append(*s))
        });
    }

    for i in 2..=8 {
        let size = i * 500;
        group.bench_with_input(BenchmarkId::new("Binary Heap", size), &size, |b, s| {
            b.iter(|| binheap_append(*s))
        });
        group.bench_with_input(BenchmarkId::new("Weak Heap", size), &size, |b, s| {
            b.iter(|| weakheap_append(*s))
        });
    }

    group.finish();
}

fn bench_long_comp(c: &mut Criterion) {
    let mut group = c.benchmark_group("Strings with long comparison");
    let size = 54;
    group.bench_with_input(BenchmarkId::new("Binary Heap", size), &size, |b, s| {
        b.iter(|| binheap_longcomp(*s))
    });
    group.bench_with_input(BenchmarkId::new("Weak Heap", size), &size, |b, s| {
        b.iter(|| weakheap_longcomp(*s))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_sorting,
    bench_basics,
    bench_append,
    bench_long_comp
);
criterion_main!(benches);
