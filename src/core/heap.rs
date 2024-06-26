use std::collections::{BinaryHeap as CoreBinaryHeap, HashSet};
use std::hash::Hash;

/// The standard library has a priority queue implemented using a binary heap
/// however the heap itself does not have a `contains` method, so an adapter is
/// needed to mimic that behaviour with an additional hashset under the hood
pub struct BinaryHeap<T> {
    open: CoreBinaryHeap<T>,
    set: HashSet<T>,
}

impl<T> BinaryHeap<T>
where
    T: Ord + Hash + Copy,
{
    pub fn new() -> Self {
        BinaryHeap {
            open: CoreBinaryHeap::new(),
            set: HashSet::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.open.push(item);
        self.set.insert(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        let result: Option<T> = self.open.pop();

        match result {
            Some(item) => {
                self.set.remove(&item);
            },
            None => ()
        }

        result
    }

    pub fn is_empty(&self) -> bool {
        self.open.is_empty()
    }

    pub fn contains(&self, item: T) -> bool {
        self.set.contains(&item)
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::BinaryHeap;

    #[test]
    fn test_push_pop() {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();

        heap.push(2);
        heap.push(3);
        heap.push(1);

        assert_eq!(true, heap.contains(3));

        heap.pop();

        // note that 1 was the last item we inserted but the heap would have
        // removed 3. this is because it's a binary heap.
        assert_eq!(false, heap.contains(3));

        heap.pop();

        assert_eq!(false, heap.contains(2));
        assert_eq!(true, heap.contains(1));
    }
}
