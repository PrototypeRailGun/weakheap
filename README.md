# Weak Heap

* [Documentation](https://docs.rs/weakheap/)
* [Crate](https://crates.io/crates/weakheap)

## Description

A priority queue implemented with a weak heap.

Insertion and popping the largest element have *O*(log(*n*)) time complexity.
Checking the largest element is *O*(1). Converting a vector to a weak heap
can be done in-place, and has *O*(*n*) complexity. A weak heap can also be
converted to a sorted vector in-place, allowing it to be used for an *O*(*n* * log(*n*))
in-place weak-heapsort.

The main purpose of using a weak heap is to minimize the number of comparisons
required for `push` and `pop` operations or sorting, which is why it is especially
useful in cases where comparing elements is an expensive operation, for example, string collation.
For the classical comparison of numbers, it is still preferable to use a standard binary heap,
since operations with a weak heap require additional numerical operations compared
to a conventional binary heap.

This create presents an implementation of the weak heap - `WeakHeap`, which has an identical interface
with [`BinaryHeap`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html)
from `std::collections`, and at the same time it has several new useful methods.

## Read about weak heap:
* [Wikipedia](https://en.wikipedia.org/wiki/Weak_heap)
* [The weak-heap data structure: Variants and applications](https://www.sciencedirect.com/science/article/pii/S1570866712000792)


## Usage

As a library

```rust
use weakheap::WeakHeap;

// Type inference lets us omit an explicit type signature (which
// would be `WeakHeap<i32>` in this example).
let mut heap = WeakHeap::new();

// We can use peek to look at the next item in the heap. In this case,
// there's no items in there yet so we get None.
assert_eq!(heap.peek(), None);

// Let's add some scores...
heap.push(1);
heap.push(5);
heap.push(2);

// Now peek shows the most important item in the heap.
assert_eq!(heap.peek(), Some(&5));

// We can check the length of a heap.
assert_eq!(heap.len(), 3);

// We can iterate over the items in the heap, although they are returned in
// a random order.
for x in heap.iter() {
   println!("{}", x);
}

// If we instead pop these scores, they should come back in order.
assert_eq!(heap.pop(), Some(5));
assert_eq!(heap.pop(), Some(2));
assert_eq!(heap.pop(), Some(1));
assert_eq!(heap.pop(), None);

// We can clear the heap of any remaining items.
heap.clear();

// The heap should now be empty.
assert!(heap.is_empty())
```

## Benchmarks
All tests were performed using the same data - the words from the excerpt of the novel "Martin Eden".
The `input` axis shows the number of rows used in this bench. The `Append' operation is the merging of two heaps.

![Typing SVG](/benches/reports/push_pop/lines.svg)
![Typing SVG](/benches/reports/append/lines.svg)
![Typing SVG](/benches/reports/sorting/lines.svg)

As can be seen from the graphs presented, a weak heap works faster than a binary one when the elements are strings, however, the `quicksort` algorithm, presented in `Rust` by the `Vec::sort_unstable` method, copes best with sorting.

If you have any comments or suggestions, or you suddenly found an error, please start a new issue or pool request.
