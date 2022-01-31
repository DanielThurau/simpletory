use rust_decimal::prelude::*;
use std::cmp::Ordering;

#[derive(Default, Clone, Copy, Eq)]
pub struct Inventory {
    price_per_item: Decimal,
    quantity: u64,
}

pub struct InventoryView {
    price_per_item: Decimal,
}

trait MinHeap {
    fn heapify(&mut self, index: usize);

    fn insert(&mut self, inventory: Inventory);

    fn delete(&mut self);

    /// Returns the value of an item and then decrements its quantity from the heap.
    /// Currently this can only extract a single quantity of inventory at a time. In
    /// the future there will be an equivalent batch operation.
    fn extract(&mut self) -> InventoryView;

    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;

    fn get_min(&self) -> InventoryView;

    fn new() -> Self;
}

/// Heavily influenced by
/// https://www.journaldev.com/36805/min-heap-binary-tree
struct InventoryHeap {
    heap: Vec<Inventory>,
}

impl MinHeap for InventoryHeap {
    fn heapify(&mut self, index: usize) {
        if self.size() <= 1 {
            return;
        }

        let left = self.left_child(index);
        let right = self.right_child(index);

        let mut smallest = index;

        if left < self.size() && self.heap[left] < self.heap[smallest] {
            smallest = left;
        }

        if right < self.size() && self.heap[right] < self.heap[smallest] {
            smallest = right;
        }

        if smallest != index {
            let tmp = self.heap[index];
            self.heap[index] = self.heap[smallest];
            self.heap[smallest] = tmp;
            self.heapify(smallest);
        }
    }

    fn insert(&mut self, inventory: Inventory) {
        self.heap.push(inventory);
        let mut index = self.size() - 1;
        while index != 0 {
            let parent_index = self.parent(index);
            // Base case that means the last swap brought the node into its
            // correct location in the vector
            if self.heap[parent_index] <= self.heap[index] {
                return;
            }

            let tmp = self.heap[parent_index];
            self.heap[parent_index] = self.heap[index];
            self.heap[index] = tmp;

            index = parent_index;
        }
    }

    fn delete(&mut self) {
        // Nothing to do, heap is empty. Results in a no-op.
        if self.heap.len() == 0 {
            return;
        }

        if self.heap[0].quantity > 1 {
            self.heap[0].quantity -= 1;
            return;
        }

        let last_index = self.size() - 1;
        self.heap[0] = self.heap[last_index];
        self.heap.pop();
        self.heapify(0);
    }

    fn extract(&mut self) -> InventoryView {
        let min = self.get_min();
        self.delete();
        return min;
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize {
        self.heap.len()
    }

    fn get_min(&self) -> InventoryView {
        let inventory = &self.heap[0];

        InventoryView {
            price_per_item: inventory.price_per_item.clone(),
        }
    }

    fn new() -> Self {
        InventoryHeap { heap: vec![] }
    }
}

impl InventoryHeap {
    fn parent(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    fn left_child(&self, index: usize) -> usize {
        (2 * index) + 1
    }

    fn right_child(&self, index: usize) -> usize {
        (2 * index) + 2
    }
}

impl PartialOrd for Inventory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Inventory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price_per_item.cmp(&other.price_per_item)
    }
}

impl PartialEq for Inventory {
    fn eq(&self, other: &Self) -> bool {
        self.price_per_item == other.price_per_item
    }
}

#[cfg(test)]
mod tests {
    use crate::inventory_heap::{Inventory, InventoryHeap, MinHeap};
    use rand::Rng;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[test]
    fn create_heap() {
        let heap = InventoryHeap::new();
        assert_eq!(heap.heap.len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_empty_heap_should_panic() {
        let heap = InventoryHeap::new();
        heap.get_min();
    }

    #[test]
    fn test_inserting_into_heap_should_succeed() {
        let mut heap = InventoryHeap::new();

        let inv = Inventory {
            price_per_item: dec!(1.00),
            quantity: 10,
        };
        heap.insert(inv);
        assert_eq!(heap.get_min().price_per_item, inv.price_per_item)
    }

    #[test]
    fn test_min_heap_property_maintained_during_insertion() {
        let mut heap = InventoryHeap::new();

        for i in (1..5).rev() {
            let inv = Inventory {
                price_per_item: Decimal::new(i, 0),
                quantity: 1,
            };

            heap.insert(inv);
        }

        let min = heap.get_min();

        assert_eq!(min.price_per_item, dec!(1));
    }

    #[test]
    fn test_inventory_with_single_quantity_deletes() {
        let mut heap = InventoryHeap::new();

        let inv = Inventory {
            price_per_item: dec!(1.00),
            quantity: 1,
        };

        heap.insert(inv);
        assert_eq!(heap.size(), 1);

        heap.delete();
        assert_eq!(heap.size(), 0);
    }

    #[test]
    fn test_inventory_with_large_quantity_deletes() {
        let mut heap = InventoryHeap::new();

        let inv = Inventory {
            price_per_item: dec!(1.00),
            quantity: 2,
        };

        heap.insert(inv);
        assert_eq!(heap.size(), 1);

        heap.delete();
        assert_eq!(heap.size(), 1);

        heap.delete();
        assert_eq!(heap.size(), 0);
    }

    #[test]
    fn test_inserting_equivalent_inventories() {
        let mut heap = InventoryHeap::new();

        let inv = Inventory {
            price_per_item: dec!(1.00),
            quantity: 2,
        };

        heap.insert(inv);
        assert_eq!(heap.size(), 1);

        heap.insert(inv);
        assert_eq!(heap.size(), 2);

        heap.delete();
        heap.delete();

        assert_eq!(heap.size(), 1);
    }

    #[test]
    fn test_inserting_and_deleting_random_inventories() {
        let mut heap = InventoryHeap::new();
        let mut rng = rand::thread_rng();

        for _i in 0..10 {
            let inv = Inventory {
                price_per_item: Decimal::new(rng.gen_range(0..100), 0),
                quantity: rng.gen_range(0..5),
            };

            heap.insert(inv);
        }

        let mut smallest = heap.extract().price_per_item;
        while !heap.is_empty() {
            let heap_min = heap.extract().price_per_item;
            assert!(smallest <= heap_min);
            smallest = heap_min;
        }
    }
}
