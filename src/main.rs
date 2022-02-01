use crate::inventory_heap::{InventoryHeap, MinHeap};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

mod inventory_heap;

enum TransactionType {
    Produce,
    Consume,
}

struct Transaction {
    transaction_type: TransactionType,
    inventory_id: String,
    total: Decimal,
    quantity: usize,
}

#[derive(Default)]
struct TransactionHistory {
    history: Vec<Transaction>,
}

#[derive(Default)]
struct InventoryIdMap {
    product_strings_to_ids: HashMap<String, u64>,
    next_id: u64,
}

impl InventoryIdMap {
    fn get_inventory_key(&mut self, inventory: &String) -> Result<u64, WarehouseError> {
        if !self.product_strings_to_ids.contains_key(inventory) {
            self.insert_new_key(inventory);
        }

        match self.product_strings_to_ids.get(inventory) {
            Some(key) => Ok(key.clone()),
            None => Err(WarehouseError),
        }
    }

    fn insert_new_key(&mut self, inventory: &String) {
        self.product_strings_to_ids
            .insert(inventory.clone(), self.next_id);
        self.next_id += 1;
    }
}

#[derive(Debug, Clone)]
struct WarehouseError;

impl fmt::Display for WarehouseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error encountered when fucking with the warehouse. Piss off"
        )
    }
}

#[derive(Default)]
struct Warehouse<T>
where
    T: MinHeap,
{
    inventory_id_map: InventoryIdMap,
    inventory_heaps: HashMap<u64, T>,
    transaction_history: TransactionHistory,
}

impl<T: MinHeap> Warehouse<T> {
    fn transact(&self, t: Transaction) -> Result<(), WarehouseError> {
        // 1. Check validity of the transaction
        // 1.5. Check
        // 2. If produce, call internal produce_method
        // 3. If consume, call internal consume_method
        // 4. If operation succeeds, add to the the transaction_history

        Ok(())
    }
}

fn main() {
    let warehouse: Warehouse<InventoryHeap> = Warehouse::default();
}
