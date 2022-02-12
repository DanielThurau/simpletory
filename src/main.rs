use crate::inventory_heap::{Inventory, InventoryHeap, MinHeap};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

mod inventory_heap;

#[derive(Eq, PartialEq)]
enum TransactionType {
    Produce,
    Consume,
}

struct Transaction {
    transaction_type: TransactionType,
    inventory_id: String,
    quantity: usize,
    total_cost: Option<Decimal>,
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
    fn transact(&mut self, t: Transaction) -> Result<(), WarehouseError> {
        self.validate_transaction(&t)?;

        match t.transaction_type {
            TransactionType::Produce => {
                self.produce(&t)?;
            }
            TransactionType::Consume => {
                self.consume(&t)?;
            }
        }

        self.transaction_history.history.push(t);

        Ok(())
    }

    fn validate_transaction(&self, t: &Transaction) -> Result<(), WarehouseError> {
        if t.transaction_type == TransactionType::Produce {
            if t.total_cost.is_none() {
                println!("cost_total should be not be None if TransactionType is Produce");
                return Err(WarehouseError);
            }
        } else if t.transaction_type == TransactionType::Consume {
            if t.total_cost.is_some() {
                println!("cost_total should be not be Some(_) if TransactionType is Consume");
                return Err(WarehouseError);
            }
        }

        Ok(())
    }

    fn produce(&mut self, t: &Transaction) -> Result<(), WarehouseError> {
        let id = self.inventory_id_map.get_inventory_key(&t.inventory_id)?;

        let inventory = Inventory {
            price_per_item: t.total_cost.unwrap() / Decimal::new(t.quantity as i64, 0),
            quantity: t.quantity,
        };

        self.inventory_heaps
            .entry(id)
            .and_modify(|heap| heap.insert(inventory))
            .or_insert_with(|| {
                let mut heap = T::new();
                heap.insert(inventory);
                heap
            });

        println!("Processed a produce transaction for product '{}' with quantity {} and price per item {}",
                 t.inventory_id, inventory.quantity, inventory.price_per_item);

        Ok(())
    }

    fn consume(&mut self, t: &Transaction) -> Result<(), WarehouseError> {
        let id = self.inventory_id_map.get_inventory_key(&t.inventory_id)?;

        let inventory_view = match self.inventory_heaps.get_mut(&id) {
            Some(heap) => Ok(heap.extract()),
            None => {
                println!(
                    "Trying to consume inventory({}) that doesn't exist",
                    t.inventory_id
                );
                Err(WarehouseError)
            }
        }?;

        println!(
            "Processed a consume transaction for product '{}'",
            t.inventory_id
        );
        for inventory_block in inventory_view.inventory {
            println!(
                "Consumed quantity ({}) at price ({})",
                inventory_block.quantity, inventory_block.price_per_item
            );
        }

        Ok(())
    }
}


fn create_transaction(
    inventory_id: String,
    total_cost: Option<Decimal>,
    transaction_type: TransactionType,
    quantity: usize
) -> Transaction {
    Transaction {
        transaction_type,
        inventory_id,
        quantity,
        total_cost
    }
}

fn main() {
    let mut warehouse: Warehouse<InventoryHeap> = Warehouse::default();

    let t = create_transaction(String::from("Acrylic Box"), Some(dec!(10.00)), TransactionType::Produce, 9);

    match warehouse.transact(t) {
        Err(e) => panic!("Ooops {}", e),
        _ => (),
    }

    let t2 = create_transaction(String::from("Acrylic Box"), None, TransactionType::Consume, 1);

    match warehouse.transact(t2) {
        Err(e) => panic!("Ooops {}", e),
        _ => (),
    }


}
