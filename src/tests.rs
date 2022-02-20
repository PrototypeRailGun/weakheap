use crate::{WeakHeap, WeakHeapPeekMut};
use rand::{thread_rng, Rng};
use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;

#[test]
fn test_creation() {
    // WeakHeap::new()
    let mut heap: WeakHeap<i32> = WeakHeap::new();
    assert_eq!(heap.len(), 0);
    assert!(heap.is_empty());

    // WeakHeap::with_capacity()
    heap.push(1);
    assert_eq!(heap.len(), 1);
    assert!(!heap.is_empty());

    let mut heap: WeakHeap<i32> = WeakHeap::with_capacity(5);
    assert_eq!(heap.len(), 0);
    assert!(heap.is_empty());

    heap.push(1);
    assert_eq!(heap.len(), 1);
    assert!(!heap.is_empty());

    // WeakHeap::default()
    let mut heap: WeakHeap<i32> = WeakHeap::default();
    assert_eq!(heap.len(), 0);
    assert!(heap.is_empty());

    heap.push(1);
    assert_eq!(heap.len(), 1);
    assert!(!heap.is_empty());
}

#[test]
fn test_from() {
    // From Vec<T>
    let heap_from_vec = WeakHeap::from(vec![3, 2, 5, 1, 4]);
    assert_eq!(heap_from_vec.clone().into_sorted_vec(), vec![1, 2, 3, 4, 5]);

    // From array
    let mut heap_from_arr = WeakHeap::from([2, 1, 5, 4, 3]);
    let mut temp_heap = heap_from_vec.clone();
    while let Some((a, b)) = temp_heap.pop().zip(heap_from_arr.pop()) {
        assert_eq!(a, b);
    }
    assert!(heap_from_arr.is_empty());

    // From iter
    let mut heap_from_iter: WeakHeap<i32> = [3, 2, 5, 4, 1].into_iter().collect();
    let mut temp_heap = heap_from_vec.clone();
    while let Some((a, b)) = temp_heap.pop().zip(heap_from_iter.pop()) {
        assert_eq!(a, b);
    }
    assert!(heap_from_iter.is_empty());

    let mut heap_from_iter = WeakHeap::from_iter([3, 2, 5, 4, 1].into_iter());
    let mut temp_heap = heap_from_vec.clone();
    while let Some((a, b)) = temp_heap.pop().zip(heap_from_iter.pop()) {
        assert_eq!(a, b);
    }
    assert!(heap_from_iter.is_empty());
}

#[test]
fn test_into_sorted_vec() {
    // Edge cases
    let elements: Vec<i32> = vec![];
    assert_eq!(WeakHeap::from(elements).into_sorted_vec(), vec![],);

    let elements: Vec<i32> = vec![1];
    assert_eq!(WeakHeap::from(elements).into_sorted_vec(), vec![1],);

    // Fixed tests
    let elements = [7, 1, 4, 5, 3, 2, 2, 7, 6, 9, 1];
    assert_eq!(
        WeakHeap::from(elements).into_sorted_vec(),
        vec![1, 1, 2, 2, 3, 4, 5, 6, 7, 7, 9],
    );

    let elements: Vec<i32> = vec![1, 1, 2, 1, 1];
    assert_eq!(
        WeakHeap::from(elements).into_sorted_vec(),
        vec![1, 1, 1, 1, 2],
    );

    // Random tests
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let weak_heap = WeakHeap::from(elements.clone());
        assert_eq!(weak_heap.len(), size); // Testing `from`.

        elements.sort();
        assert_eq!(weak_heap.into_sorted_vec(), elements);
    }
}

#[test]
fn test_push() {
    // Fixed tests
    let mut weak_heap = WeakHeap::new();
    weak_heap.push(2);
    assert_eq!(weak_heap.len(), 1);
    assert_eq!(weak_heap.peek(), Some(&2));

    weak_heap.push(1);
    assert_eq!(weak_heap.len(), 2);
    assert_eq!(weak_heap.peek(), Some(&2));

    weak_heap.push(3);
    assert_eq!(weak_heap.len(), 3);
    assert_eq!(weak_heap.peek(), Some(&3));

    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut binary_hep: BinaryHeap<i64> = BinaryHeap::new();

        let mut weak_heap: WeakHeap<i64> = WeakHeap::new();
        assert_eq!(weak_heap.len(), 0);

        for x in elements.iter() {
            binary_hep.push(*x);
            weak_heap.push(*x);
            assert_eq!(weak_heap.len(), binary_hep.len());
            assert_eq!(weak_heap.peek(), binary_hep.peek());
        }

        elements.sort();
        assert_eq!(weak_heap.into_sorted_vec(), elements);
    }
}

#[test]
fn test_pop() {
    // Fixed tests
    let mut weak_heap: WeakHeap<i32> = WeakHeap::new();
    assert_eq!(weak_heap.pop(), None);
    assert_eq!(weak_heap.len(), 0);

    let mut weak_heap = WeakHeap::from(vec![4, 2]);
    assert_eq!(weak_heap.pop(), Some(4));
    assert_eq!(weak_heap.len(), 1);
    assert_eq!(weak_heap.pop(), Some(2));
    assert_eq!(weak_heap.len(), 0);
    assert_eq!(weak_heap.pop(), None);
    assert_eq!(weak_heap.len(), 0);

    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut binary_hep: BinaryHeap<i64> = BinaryHeap::from(elements.clone());
        let mut weak_heap: WeakHeap<i64> = WeakHeap::from(elements);

        while !binary_hep.is_empty() {
            assert_eq!(weak_heap.pop(), binary_hep.pop());
            assert_eq!(weak_heap.len(), binary_hep.len());
        }
        assert!(binary_hep.is_empty());
    }
}

#[test]
fn test_pop_with_push() {
    // Let's make sure that push and pop do not interfere with each other's work.

    // Fixed tests
    let mut weak_heap = WeakHeap::new();
    weak_heap.push(2);
    assert_eq!(weak_heap.peek(), Some(&2));
    assert_eq!(weak_heap.len(), 1);
    weak_heap.push(4);
    assert_eq!(weak_heap.peek(), Some(&4));
    assert_eq!(weak_heap.len(), 2);
    assert_eq!(weak_heap.pop(), Some(4));
    assert_eq!(weak_heap.len(), 1);
    assert_eq!(weak_heap.pop(), Some(2));
    assert_eq!(weak_heap.len(), 0);
    assert_eq!(weak_heap.pop(), None);
    assert_eq!(weak_heap.len(), 0);

    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut binary_hep: BinaryHeap<i64> = BinaryHeap::new();
        let mut weak_heap: WeakHeap<i64> = WeakHeap::new();

        for x in elements {
            binary_hep.push(x);
            weak_heap.push(x);
            if x % 2 == 0 {
                assert_eq!(weak_heap.pop(), binary_hep.pop());
                assert_eq!(weak_heap.len(), binary_hep.len());
            }
        }
        assert_eq!(weak_heap.into_sorted_vec(), binary_hep.into_sorted_vec());
    }
}

#[test]
fn test_clone() {
    let h1 = WeakHeap::from(vec![7, 5, 9, 0, 2]);
    let h2 = h1.clone();
    let mut h3 = WeakHeap::<i32>::new();
    h3.clone_from(&h1);
    let res = h1.into_sorted_vec();
    assert_eq!(h2.into_sorted_vec(), res);
    assert_eq!(h3.into_sorted_vec(), res);
}

#[test]
fn test_peek() {
    let mut heap = WeakHeap::new();
    assert_eq!(heap.peek(), None);

    heap.push(1);
    assert_eq!(heap.peek(), Some(&1));

    heap.push(5);
    assert_eq!(heap.peek(), Some(&5));

    heap.pop();
    assert_eq!(heap.peek(), Some(&1));
    heap.pop();
    assert_eq!(heap.peek(), None);
}

#[test]
fn test_capacity() {
    let mut heap: WeakHeap<i32> = WeakHeap::new();
    assert_eq!(heap.capacity(), 0);
    heap.push(1);
    assert_eq!(heap.capacity(), 4);

    let mut heap: WeakHeap<i32> = WeakHeap::with_capacity(2);
    assert_eq!(heap.capacity(), 2);

    heap.push(1);
    heap.push(2);
    assert_eq!(heap.capacity(), 2);

    heap.push(3);
    assert_eq!(heap.capacity(), 4);
}

#[test]
fn test_reserve() {
    let mut heap = WeakHeap::from([3, 4]);
    assert_eq!(heap.capacity(), 2);
    heap.reserve(100);
    assert!(heap.capacity() >= 102);
}

#[test]
fn test_reserve_exact() {
    let mut heap = WeakHeap::from([3, 4]);
    assert_eq!(heap.capacity(), 2);
    heap.reserve_exact(100);
    assert!(heap.capacity() >= 102);
}

#[test]
fn test_shrink_to() {
    let mut heap: WeakHeap<i32> = WeakHeap::with_capacity(20);
    assert_eq!(heap.capacity(), 20);

    heap.shrink_to(100);
    assert_eq!(heap.capacity(), 20);

    heap.shrink_to(10);
    assert_eq!(heap.capacity(), 10);
}

#[test]
fn test_shrink_to_fit() {
    let mut heap: WeakHeap<i32> = WeakHeap::with_capacity(10);
    heap.shrink_to_fit();
    assert_eq!(heap.capacity(), 0);

    heap.push(1);
    heap.push(2);
    heap.push(3);
    heap.shrink_to_fit();
    assert_eq!(heap.capacity(), 3);
}

#[test]
fn test_peek_mut() {
    let mut heap: WeakHeap<i32> = WeakHeap::new();
    assert!(heap.peek_mut().is_none());

    heap.push(3);
    {
        let mut top = heap.peek_mut().unwrap();
        *top = 4;
    }
    assert_eq!(heap.peek(), Some(&4));

    heap.push(1);
    heap.push(6);
    assert_eq!(heap.peek(), Some(&6));
    {
        let mut top = heap.peek_mut().unwrap();
        *top = 0;
    }
    assert_eq!(heap.peek(), Some(&4));

    {
        let top = heap.peek_mut().unwrap();
        assert_eq!(WeakHeapPeekMut::pop(top), 4);
    }
    assert_eq!(heap.peek(), Some(&1));

    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 1..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut binary_heap: BinaryHeap<i64> = BinaryHeap::from(elements.clone());
        let mut weak_heap: WeakHeap<i64> = WeakHeap::from(elements);

        for _ in 0..size * 2 {
            {
                let new_val: i64 = rng.gen_range(-50..=50);
                let mut bin_val = binary_heap.peek_mut().unwrap();
                let mut weak_val = weak_heap.peek_mut().unwrap();
                *bin_val = new_val;
                *weak_val = new_val;
            }
            assert_eq!(weak_heap.peek(), binary_heap.peek());
        }

        assert_eq!(
            weak_heap.clone().into_sorted_vec(),
            binary_heap.clone().into_sorted_vec()
        );

        for _ in 0..size {
            {
                let bin_val = binary_heap.peek_mut().unwrap();
                let weak_val = weak_heap.peek_mut().unwrap();
                assert_eq!(WeakHeapPeekMut::pop(weak_val), PeekMut::pop(bin_val));
            }
            assert_eq!(weak_heap.peek(), binary_heap.peek());
        }
        assert!(weak_heap.is_empty());
    }
}

#[test]
fn test_pushpop() {
    let mut heap: WeakHeap<i64> = WeakHeap::new();
    assert_eq!(heap.pushpop(5), 5);
    assert_eq!(heap.len(), 0);

    heap.push(3);
    assert_eq!(heap.pushpop(2), 3);
    assert_eq!(heap.peek(), Some(&2));
    assert_eq!(heap.len(), 1);

    assert_eq!(heap.pushpop(4), 4);
    assert_eq!(heap.peek(), Some(&2));
    assert_eq!(heap.len(), 1);

    // Random tests against push and pop
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut heap1 = WeakHeap::from(elements); // pushpop
        let mut heap2 = heap1.clone(); //push and pop

        for _ in 0..size * 2 {
            let item = rng.gen_range(-50..50);
            heap2.push(item);
            assert_eq!(heap1.pushpop(item), heap2.pop().unwrap());
            assert_eq!(heap1.len(), heap2.len());
            assert_eq!(heap1.peek(), heap2.peek());
        }

        assert_eq!(heap1.into_sorted_vec(), heap2.into_sorted_vec());
    }
}

#[test]
fn test_append() {
    let mut h1: WeakHeap<i64> = WeakHeap::new();
    let mut h2: WeakHeap<i64> = WeakHeap::new();
    h1.append(&mut h2);
    assert_eq!(h1.into_sorted_vec(), vec![]);

    // Random tests against BinaryHeap
    let mut rng = thread_rng();
    for size1 in 0..100 {
        let mut elements1: Vec<i64> = Vec::with_capacity(size1);
        for _ in 0..size1 {
            elements1.push(rng.gen_range(-30..=30));
        }

        let weak_heap = WeakHeap::from(elements1.clone());
        let bin_heap = BinaryHeap::from(elements1);

        for size2 in 0..100 {
            let mut elements2: Vec<i64> = Vec::with_capacity(size2);
            for _ in 0..size2 {
                elements2.push(rng.gen_range(-30..=30));
            }

            let mut wh2 = WeakHeap::from(elements2.clone());
            let mut bh2 = BinaryHeap::from(elements2);

            let mut wh1 = weak_heap.clone();
            let mut bh1 = bin_heap.clone();

            wh1.append(&mut wh2);
            bh1.append(&mut bh2);

            assert_eq!(wh1.peek(), bh1.peek());
            assert_eq!(wh1.len(), bh1.len());
            assert!(wh2.is_empty());
            assert_eq!(wh1.into_sorted_vec(), bh1.into_sorted_vec());
        }
    }
}