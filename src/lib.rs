//! A priority queue implemented with a weak heap.
//!
//! Insertion and popping the largest element have *O*(log(*n*)) time complexity.
//! Checking the largest element is *O*(1). Converting a vector to a weak heap
//! can be done in-place, and has *O*(*n*) complexity. A weak heap can also be
//! converted to a sorted vector in-place, allowing it to be used for an *O*(*n* * log(*n*))
//! in-place weak-heapsort.
//!
//! The main purpose of using a weak heap is to minimize the number of comparisons
//! required for `push` and `pop` operations or sorting, which is why it is especially
//! useful in cases where comparing elements is an expensive operation, for example, string collation.
//! For the classical comparison of numbers, it is still preferable to use a standard binary heap,
//! since operations with a weak heap require additional numerical operations compared
//! to a conventional binary heap.
//!
//! This create presents an implementation of the weak heap - `WeakHeap`, which has an identical interface
//! with [`BinaryHeap`]
//! from `std::collections`, and at the same time it has several new useful methods.
//!
//! # Read about weak heap:
//! * [Wikipedia](https://en.wikipedia.org/wiki/Weak_heap)
//! * [The weak-heap data structure: Variants and applications](https://www.sciencedirect.com/science/article/pii/S1570866712000792)
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
//!
use std::fmt;
use std::iter::{FromIterator, FusedIterator};
use std::mem::{swap, ManuallyDrop};
use std::ops::{Deref, DerefMut};
use std::ptr;

/// A priority queue implemented with a weak heap.
///
/// This will be a max-heap.
///
/// # Examples
///
/// ```
/// use weakheap::WeakHeap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `WeakHeap<i32>` in this example).
/// let mut heap = WeakHeap::new();
///
/// // We can use peek to look at the next item in the heap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(heap.peek(), None);
///
/// // Let's add some scores...
/// heap.push(1);
/// heap.push(5);
/// heap.push(2);
///
/// // Now peek shows the most important item in the heap.
/// assert_eq!(heap.peek(), Some(&5));
///
/// // We can check the length of a heap.
/// assert_eq!(heap.len(), 3);
///
/// // We can iterate over the items in the heap, although they are returned in
/// // a random order.
/// for x in heap.iter() {
///     println!("{}", x);
/// }
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), None);
///
/// // We can clear the heap of any remaining items.
/// heap.clear();
///
/// // The heap should now be empty.
/// assert!(heap.is_empty())
/// ```
///
/// A `WeakHeap` with a known list of items can be initialized from an array:
///
/// ```
/// use weakheap::WeakHeap;
///
/// let heap = WeakHeap::from([1, 5, 2]);
/// ```
///
/// ## Min-heap
///
/// Either [`core::cmp::Reverse`] or a custom [`Ord`] implementation can be used to
/// make `WeakHeap` a min-heap. This makes `heap.pop()` return the smallest
/// value instead of the greatest one.
///
/// ```
/// use weakheap::WeakHeap;
/// use std::cmp::Reverse;
///
/// let mut heap = WeakHeap::new();
///
/// // Wrap values in `Reverse`
/// heap.push(Reverse(1));
/// heap.push(Reverse(5));
/// heap.push(Reverse(2));
///
/// // If we pop these scores now, they should come back in the reverse order.
/// assert_eq!(heap.pop(), Some(Reverse(1)));
/// assert_eq!(heap.pop(), Some(Reverse(2)));
/// assert_eq!(heap.pop(), Some(Reverse(5)));
/// assert_eq!(heap.pop(), None);
/// ```
///
/// ## Sorting
///
/// ```
/// use weakheap::WeakHeap;
///
/// let heap = WeakHeap::from([5, 3, 1, 7]);
/// assert_eq!(heap.into_sorted_vec(), vec![1, 3, 5, 7]);
/// ```
///
/// # Time complexity
///
/// | [push]  | [pop]         | [peek]/[peek\_mut] | [into_sorted_vec] |
/// |---------|---------------|--------------------|-------------------|
/// | *O*(1)~ | *O*(log(*n*)) | *O*(1)             | *O*(*n*log(*n*))  |
///
/// The value for `push` is an expected cost; the method documentation gives a
/// more detailed analysis.
///
/// [`core::cmp::Reverse`]: core::cmp::Reverse
/// [`Ord`]: core::cmp::Ord
/// [`Cell`]: core::cell::Cell
/// [`RefCell`]: core::cell::RefCell
/// [push]: WeakHeap::push
/// [pop]: WeakHeap::pop
/// [peek]: WeakHeap::peek
/// [peek\_mut]: WeakHeap::peek_mut
/// [into_sorted_vec]: WeakHeap::into_sorted_vec
pub struct WeakHeap<T> {
    data: Vec<T>,
    bit: Vec<bool>,
}

/// Structure wrapping a mutable reference to the greatest item on a
/// `WeakHeap`.
///
/// This `struct` is created by the [`peek_mut`] method on [`WeakHeap`]. See
/// its documentation for more.
///
/// [`peek_mut`]: WeakHeap::peek_mut
pub struct WeakHeapPeekMut<'a, T: 'a + Ord> {
    heap: &'a mut WeakHeap<T>,
    sift: bool,
}

impl<T: Ord + fmt::Debug> fmt::Debug for WeakHeapPeekMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("WeakHeapPeekMut")
            .field(&self.heap.data[0])
            .finish()
    }
}

impl<T: Ord> Drop for WeakHeapPeekMut<'_, T> {
    fn drop(&mut self) {
        if self.sift {
            // SAFETY: PeekMut is only instantiated for non-empty heaps.
            unsafe { self.heap.sift_down(0) };
        }
    }
}

impl<T: Ord> Deref for WeakHeapPeekMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        debug_assert!(!self.heap.is_empty());
        // SAFE: PeekMut is only instantiated for non-empty heaps
        unsafe { self.heap.data.get_unchecked(0) }
    }
}

impl<T: Ord> DerefMut for WeakHeapPeekMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        debug_assert!(!self.heap.is_empty());
        self.sift = true;
        // SAFE: PeekMut is only instantiated for non-empty heaps
        unsafe { self.heap.data.get_unchecked_mut(0) }
    }
}

impl<'a, T: Ord> WeakHeapPeekMut<'a, T> {
    /// Removes the peeked value from the heap and returns it.
    pub fn pop(mut this: WeakHeapPeekMut<'a, T>) -> T {
        let value = this.heap.pop().unwrap();
        this.sift = false;
        value
    }
}

impl<T: Clone> Clone for WeakHeap<T> {
    fn clone(&self) -> Self {
        WeakHeap {
            data: self.data.clone(),
            bit: self.bit.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
        self.bit.clone_from(&source.bit);
    }
}

impl<T: Ord> Default for WeakHeap<T> {
    /// Creates an empty `WeakHeap` as a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::default();
    /// assert!(heap.is_empty());
    ///
    /// heap.push(4);
    /// assert_eq!(heap.len(), 1);
    /// ```
    #[inline]
    fn default() -> WeakHeap<T> {
        WeakHeap::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for WeakHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.data.iter().zip(self.bit.iter()))
            .finish()
    }
}

impl<T: Ord> WeakHeap<T> {
    /// Creates an empty `WeakHeap` as a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::new();
    /// assert!(heap.is_empty());
    ///
    /// heap.push(4);
    /// assert_eq!(heap.len(), 1);
    /// ```
    #[must_use]
    pub fn new() -> WeakHeap<T> {
        WeakHeap {
            data: vec![],
            bit: vec![],
        }
    }

    /// Creates an empty `WeakHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `WeakHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::with_capacity(10);
    /// heap.push(4);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> WeakHeap<T> {
        WeakHeap {
            data: Vec::with_capacity(capacity),
            bit: Vec::with_capacity(capacity),
        }
    }

    /// Returns a mutable reference to the greatest item in the weak heap, or
    /// `None` if it is empty.
    ///
    /// Note: If the `WeakHeapPeekMut` value is leaked, the heap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::new();
    /// assert!(heap.peek_mut().is_none());
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// {
    ///     let mut val = heap.peek_mut().unwrap();
    ///     *val = 0;
    /// }
    /// assert_eq!(heap.peek(), Some(&2));
    /// ```
    ///
    /// # Time complexity
    ///
    /// If the item is modified then the worst case time complexity is *O*(log(*n*)),
    /// otherwise it's *O*(1).
    pub fn peek_mut(&mut self) -> Option<WeakHeapPeekMut<'_, T>> {
        if self.is_empty() {
            None
        } else {
            Some(WeakHeapPeekMut {
                heap: self,
                sift: false,
            })
        }
    }

    /// Removes the greatest item from the weak heap and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(1));
    /// assert_eq!(heap.pop(), None);
    /// ```
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    ///
    /// Sifting down in a weak heap can be done in *log(2, n)* comparisons,
    /// as opposed to *2log(2, n)* for binary heap.
    pub fn pop(&mut self) -> Option<T> {
        self.bit.pop();
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                swap(&mut item, &mut self.data[0]);
                // SAFETY: !self.is_empty() means that self.len() > 0
                unsafe { self.sift_down(0) };
            }
            item
        })
    }

    /// Pushes an item onto the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::new();
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert_eq!(heap.len(), 3);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    ///
    /// # Time complexity
    ///
    /// The expected cost of `push`, averaged over every possible ordering of
    /// the elements being pushed, and over a sufficiently large number of
    /// pushes, is *O*(1). This is the most meaningful cost metric when pushing
    /// elements that are *not* already in any sorted pattern.
    ///
    /// The time complexity degrades if elements are pushed in predominantly
    /// ascending order. In the worst case, elements are pushed in ascending
    /// sorted order and the amortized cost per push is *O*(log(*n*)) against a heap
    /// containing *n* elements.
    ///
    /// The worst case cost of a *single* call to `push` is *O*(*n*). The worst case
    /// occurs when capacity is exhausted and needs a resize. The resize cost
    /// has been amortized in the previous figures.
    pub fn push(&mut self, item: T) {
        let old_len = self.len();
        self.data.push(item);
        self.bit.push(false);

        if old_len != 0 {
            // SAFETY: Since we pushed a new item it means that
            //  old_len = self.len() - 1 < self.len()
            unsafe { self.sift_up_push(0, old_len) };
        }
    }

    /// Effective equivalent to a sequential `push()` and `pop()` calls.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::new();
    /// assert_eq!(heap.pushpop(5), 5);
    /// assert!(heap.is_empty());
    ///
    /// heap.push(10);
    /// assert_eq!(heap.pushpop(20), 20);
    /// assert_eq!(heap.peek(), Some(&10));
    ///
    /// assert_eq!(heap.pushpop(5), 10);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    ///
    /// # Time complexity
    ///
    /// If the heap is empty or the element being added
    /// is larger (or equal) than the current top of the heap,
    /// then the time complexity will be *O*(1), otherwise *O*(log(*n*)).
    /// And unlike the sequential call of `push()` and `pop()`, the resizing never happens.
    pub fn pushpop(&mut self, mut item: T) -> T {
        if self.len() == 0 {
            return item;
        }

        if self.data[0] < item {
            item
        } else {
            swap(&mut item, &mut self.data[0]);
            // SAFETY: self.len() > 0
            unsafe {
                self.sift_down(0);
            }
            item
        }
    }

    /// Consumes the `WeakHeap` and returns a vector in sorted
    /// (ascending) order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    ///
    /// let mut heap = WeakHeap::from(vec![1, 2, 4, 5, 7]);
    /// heap.push(6);
    /// heap.push(3);
    ///
    /// let vec = heap.into_sorted_vec();
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    ///
    /// # Time complexity
    ///
    /// Operation can be done in *O*(*nlog(n)*) like conventional **heapsort**,
    /// but sorting by a weak heap produces significantly fewer comparisons.
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut end = self.len();
        while end > 1 {
            end -= 1;
            // SAFETY: `end` goes from `self.len() - 1` to 1 (both included),
            //  so it's always a valid index to access.
            //  It is safe to access index 0 (i.e. `ptr`), because
            //  1 <= end < self.len(), which means self.len() >= 2.
            unsafe {
                let ptr = self.data.as_mut_ptr();
                std::ptr::swap(ptr, ptr.add(end));
            }
            // SAFETY: `end` goes from `self.len() - 1` to 1 (both included) so:
            //  0 < 1 <= end <= self.len() - 1 < self.len()
            //  Which means 0 < end and end < self.len().
            unsafe { self.sift_down_range(0, end) };
        }

        self.into_vec()
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len() && self.len() > 1`.
    unsafe fn sift_up(&mut self, start: usize, pos: usize) {
        let len = self.data.len();

        // Climb up the tree in search of the first
        // element for which `pos` is in the right subtree.
        let mut cur = pos;
        let mut ancestor = cur / 2;
        while ancestor > start && (cur % 2 == *self.bit.get_unchecked(ancestor) as usize) {
            cur /= 2;
            ancestor /= 2;
        }

        // SAFETY: `start <= ancestor < pos < self.len()`
        if self.data.get_unchecked(ancestor) < self.data.get_unchecked(pos) {
            // The pos element has both children.
            if 2 * pos - 1 < len {
                *self.bit.get_unchecked_mut(pos) ^= true;
            }
            let ptr = self.data.as_mut_ptr();
            std::ptr::swap_nonoverlapping(ptr.add(ancestor), ptr.add(pos), 1);
        }
    }

    // `sift_up` works correctly only when you need to build a heap from scratch.
    // Therefore, to maintain the invariant of the heap after adding one element,
    // a little "longer" sifting is needed.

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len() && self.len() > 1`.
    unsafe fn sift_up_push(&mut self, start: usize, pos: usize) -> usize {
        let len = self.data.len();
        let mut hole = Hole::new(&mut self.data, pos);

        // Raise the `pos` element to the start until it is guaranteed
        // to be less than (or equal to) its ancestor.
        let mut cur = pos;
        while cur > start {
            // Climb up the tree in search of the first
            // element for which pos is in the right subtree.
            let mut ancestor = cur / 2;
            while ancestor > start && (cur % 2 == *self.bit.get_unchecked(ancestor) as usize) {
                cur /= 2;
                ancestor /= 2;
            }

            if hole.get(ancestor) < hole.element() {
                // The pos element has both children.
                if 2 * pos - 1 < len {
                    *self.bit.get_unchecked_mut(pos) ^= true;
                }
                hole.move_to(ancestor);
            } else {
                break; // Heap property restored.
            }

            cur = ancestor;
        }

        hole.pos()
    }

    // Sifting down in a weak heap can be done in *log(2, n)* comparisons,
    // as opposed to *2log(2, n)* for binary heap.

    /// Take an element at `pos` and move it down the heap,
    /// restoring the heap property.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `start < end <= self.len()`.
    unsafe fn sift_down_range(&mut self, start: usize, end: usize) {
        if end == 1 {
            return;
        }

        let mut pos = start.max(1);

        // We go down the left descendants as low as possible.
        while pos * 2 + (*self.bit.get_unchecked(pos) as usize) < end {
            pos = 2 * pos + (*self.bit.get_unchecked(pos) as usize);
        }

        while pos > start {
            if self.data.get_unchecked(start) < self.data.get_unchecked(pos) {
                *self.bit.get_unchecked_mut(pos) ^= true;
                let ptr = self.data.as_mut_ptr();
                std::ptr::swap_nonoverlapping(ptr.add(start), ptr.add(pos), 1);
            }
            pos /= 2;
        }
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_down(&mut self, pos: usize) {
        let len = self.len();
        // SAFETY: pos < len is guaranteed by the caller and
        //  obviously len = self.len() <= self.len().
        self.sift_down_range(pos, len);
    }

    // Building a heap. Time complexity: O(self.len()).
    fn rebuild(&mut self) {
        for n in (1..self.len()).rev() {
            // SAFETY: n starts from self.len()-1 and goes down to 1.
            unsafe {
                self.sift_up(0, n);
            }
        }
    }

    /// Rebuild assuming data[0..start] is still a proper heap.
    fn rebuild_tail(&mut self, start: usize) {
        if start == self.len() {
            return;
        }

        for i in start..self.len() {
            // SAFETY: self.len() > 1 and index `i` is always less than self.len();
            unsafe {
                self.sift_up_push(0, i);
            }
        }
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    ///
    /// let v = vec![-10, 1, 2, 3, 3];
    /// let mut a = WeakHeap::from(v);
    ///
    /// let v = vec![-20, 5, 43];
    /// let mut b = WeakHeap::from(v);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(b.is_empty());
    /// ```
    ///
    /// # Time complexity
    ///
    /// Operation can be done in *O*(*nlog(n)*) in worst case, but
    /// average time complexity is *O*(*n*), where *n* = self.len() + other.len().
    pub fn append(&mut self, other: &mut Self) {
        if self.len() < other.len() {
            swap(self, other);
        }

        let start = self.data.len();

        self.data.append(&mut other.data);
        self.bit.append(&mut other.bit);

        self.rebuild_tail(start);
    }

    /// Moves all the elements of vector `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    ///
    /// let mut a = WeakHeap::from(vec![-10, 1, 2, 3, 3]);
    ///
    /// let mut v = vec![-20, 5, 43];
    /// a.append_vec(&mut v);
    ///
    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(v.is_empty());
    /// ```
    ///
    /// # Time complexity
    ///
    /// Operation can be done in *O*(*nlog(n)*) in worst case, but
    /// average time complexity is *O*(*n*), where *n* = self.len() + other.len().
    pub fn append_vec(&mut self, other: &mut Vec<T>) {
        let start = self.len();

        self.bit.append(&mut vec![false; other.len()]);
        self.data.append(other);

        self.rebuild_tail(start);
    }
}

impl<T> WeakHeap<T> {
    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let heap = WeakHeap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.iter() {
    ///     println!("{}", x);
    /// }
    ///
    /// assert_eq!(heap.into_sorted_vec(), vec![1, 2, 3, 4]);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.data.iter(),
        }
    }

    /// Returns the greatest item in the weak heap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::new();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// assert_eq!(heap.peek(), Some(&5));
    ///
    /// ```
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Returns the number of elements the weak heap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::with_capacity(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to be inserted in the
    /// given `WeakHeap`. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer [`reserve`] if future
    /// insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::new();
    /// heap.reserve_exact(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    ///
    /// [`reserve`]: WeakHeap::reserve
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
        self.bit.reserve_exact(additional);
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the
    /// `WeakHeap`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::new();
    /// heap.reserve(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
        self.bit.reserve(additional);
    }

    /// Discards as much additional capacity as possible.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap: WeakHeap<i32> = WeakHeap::with_capacity(100);
    ///
    /// assert!(heap.capacity() >= 100);
    /// heap.shrink_to_fit();
    /// assert!(heap.capacity() == 0);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
        self.bit.shrink_to_fit();
    }

    /// Discards capacity with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap: WeakHeap<i32> = WeakHeap::with_capacity(100);
    ///
    /// assert!(heap.capacity() >= 100);
    /// heap.shrink_to(10);
    /// assert!(heap.capacity() >= 10);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity);
        self.bit.shrink_to(min_capacity);
    }

    /// Consumes the `WeakHeap<T>` and returns the underlying vector Vec<T>
    /// in arbitrary order.
    ///
    /// The results of `WeakHeap::into_vec()` and `BinaryHeap::into_vec()` are likely to differ.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let heap = WeakHeap::from(vec![1, 2, 3, 4, 5, 6, 7]);
    /// let vec = heap.into_vec();
    ///
    /// // Will print in some order
    /// for x in vec {
    ///     println!("{}", x);
    /// }
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Returns the length of the weak heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let heap = WeakHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.len(), 2);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the weak heap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::new();
    ///
    /// assert!(heap.is_empty());
    ///
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert!(!heap.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the weak heap, returning an iterator over the removed elements.
    ///
    /// The elements are removed in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::from(vec![1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// for x in heap.drain() {
    ///     println!("{}", x);
    /// }
    ///
    /// assert!(heap.is_empty());
    /// ```
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, T> {
        self.bit.clear();
        Drain {
            iter: self.data.drain(..),
        }
    }

    /// Drops all items from the weak heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let mut heap = WeakHeap::from(vec![1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// heap.clear();
    ///
    /// assert!(heap.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.drain();
    }
}

/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a, T: 'a> {
    data: &'a mut [T],
    elt: ManuallyDrop<T>,
    pos: usize,
}

impl<'a, T> Hole<'a, T> {
    /// Create a new `Hole` at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
        debug_assert!(pos < data.len());
        // SAFE: pos should be inside the slice
        let elt = ptr::read(data.get_unchecked(pos));
        Hole {
            data,
            elt: ManuallyDrop::new(elt),
            pos,
        }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    /// Returns a reference to the element removed.
    #[inline]
    fn element(&self) -> &T {
        &self.elt
    }

    /// Returns a reference to the element at `index`.
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        self.data.get_unchecked(index)
    }

    /// Move hole to new location
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn move_to(&mut self, index: usize) {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        let ptr = self.data.as_mut_ptr();
        let index_ptr: *const _ = ptr.add(index);
        let hole_ptr = ptr.add(self.pos);
        ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        self.pos = index;
    }
}

impl<T> Drop for Hole<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
        }
    }
}

impl<T: Ord> From<Vec<T>> for WeakHeap<T> {
    /// Converts a `Vec<T>` into a `WeakHeap<T>`.
    ///
    /// This conversion happens in-place, and has *O*(*n*) time complexity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let heap = WeakHeap::from(vec![5, 3, 2, 4, 1]);
    /// assert_eq!(heap.into_sorted_vec(), vec![1, 2, 3, 4, 5]);
    /// ```
    fn from(vec: Vec<T>) -> WeakHeap<T> {
        let n = vec.len();
        let mut heap = WeakHeap {
            data: vec,
            bit: vec![false; n],
        };
        heap.rebuild();
        heap
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for WeakHeap<T> {
    /// Converts a `[T, N]` into a `WeakHeap<T>`.
    ///
    /// This conversion has *O*(*n*) time complexity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    ///
    /// let mut h1 = WeakHeap::from([1, 4, 2, 3]);
    /// let mut h2: WeakHeap<_> = [1, 4, 2, 3].into();
    /// while let Some((a, b)) = h1.pop().zip(h2.pop()) {
    ///     assert_eq!(a, b);
    /// }
    /// ```
    fn from(arr: [T; N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<T> From<WeakHeap<T>> for Vec<T> {
    /// Converts a `WeakHeap<T>` into a `Vec<T>`.
    ///
    /// This conversion requires no data movement or allocation, and has
    /// constant time complexity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    ///
    /// let mut heap = WeakHeap::from([1, 3, 2]);
    /// let vec: Vec<i32> = heap.into();
    /// assert_eq!(vec, vec![3, 2, 1]);
    /// ```
    fn from(heap: WeakHeap<T>) -> Vec<T> {
        heap.data
    }
}

impl<T: Ord> FromIterator<T> for WeakHeap<T> {
    /// Building WeakHeap from iterator.
    ///
    /// This conversion has *O*(*n*) time complexity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    ///
    /// let mut h1 = WeakHeap::from([1, 4, 2, 3]);
    /// let mut h2: WeakHeap<i32> = [1, 4, 2, 3].into_iter().collect();
    /// while let Some((a, b)) = h1.pop().zip(h2.pop()) {
    ///     assert_eq!(a, b);
    /// }
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> WeakHeap<T> {
        WeakHeap::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T: Ord> Extend<T> for WeakHeap<T> {
    /// Extend WeakHeap with elements from the iterator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    ///
    /// let mut heap = WeakHeap::new();
    /// heap.extend(vec![7, 1, 0, 4, 5, 3]);
    /// assert_eq!(heap.into_sorted_vec(), [0, 1, 3, 4, 5, 7]);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x);
        }
    }
}

impl<'a, T: 'a + Ord + Copy> Extend<&'a T> for WeakHeap<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T> IntoIterator for WeakHeap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the weak heap in arbitrary order. The weak heap cannot be used
    /// after calling this.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let heap = WeakHeap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.into_iter() {
    ///     // x has type i32, not &i32
    ///     println!("{}", x);
    /// }
    /// ```
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a WeakHeap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use weakheap::WeakHeap;
    /// let heap = WeakHeap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in &heap {
    ///     // x has type &i32
    ///     println!("{}", x);
    /// }
    ///
    /// assert_eq!(heap.into_sorted_vec(), vec![1, 2, 3, 4]);
    /// ```
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

/// An iterator over the elements of a `WeakHeap`.
///
/// This `struct` is created by [`WeakHeap::iter()`]. See its
/// documentation for more.
///
/// [`iter`]: WeakHeap::iter
#[derive(Clone)]
pub struct Iter<'a, T: 'a> {
    iter: std::slice::Iter<'a, T>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.iter.as_slice()).finish()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<&'a T> {
        self.iter.last()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}

impl<T> FusedIterator for Iter<'_, T> {}

/// An owning iterator over the elements of a `WeakHeap`.
///
/// This `struct` is created by [`WeakHeap::into_iter()`]
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
///
/// [`into_iter`]: WeakHeap::into_iter
/// [`IntoIterator`]: core::iter::IntoIterator

#[derive(Clone)]
pub struct IntoIter<T> {
    iter: std::vec::IntoIter<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter")
            .field(&self.iter.as_slice())
            .finish()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

/// A draining iterator over the elements of a `WeakHeap`.
///
/// This `struct` is created by [`WeakHeap::drain()`]. See its
/// documentation for more.
///
/// [`drain`]: WeakHeap::drain
#[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    iter: std::vec::Drain<'a, T>,
}

/// A draining iterator over the elements of a `WeakHeap`.
///
/// This `struct` is created by [`WeakHeap::drain()`]. See its
/// documentation for more.
///
/// [`drain`]: WeakHeap::drain
impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for Drain<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> FusedIterator for Drain<'_, T> {}

#[cfg(test)]
mod tests;
