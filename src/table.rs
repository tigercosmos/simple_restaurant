use super::lock;
use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
use std::collections::HashMap;

pub struct Table {
    items: HashMap<u32, Item>,
    mutex: lock::Mutex,
    rng: StdRng,
}

#[derive(Debug, PartialEq)]
struct Item {
    item_id: u32,
    table_id: u32,
    prepare_time: u32,
}

impl Table {
    pub fn new() -> Table {
        Table {
            items: HashMap::new(),
            mutex: lock::Mutex::new(),
            rng: StdRng::from_entropy(),
        }
    }

    pub fn add_item(&mut self, item_id: u32, table_id: u32) {
        self.mutex.lock();

        let item = Item::new(item_id, table_id, self.rng.gen_range(5..15));
        self.items.insert(item_id, item);

        self.mutex.unlock();
    }

    pub fn check_item(&self, item_id: u32) -> Option<&Item> {
        self.mutex.lock();
        let check = self.items.get(&item_id);
        self.mutex.unlock();

        check
    }

    pub fn remove_item(&mut self, item_id: u32) -> Option<Item> {
        self.mutex.lock();
        let item = self.items.remove(&item_id);
        self.mutex.unlock();

        item
    }
}

impl Item {
    pub fn new(p_item_id: u32, p_table_id: u32, p_time: u32) -> Item {
        Item {
            item_id: p_item_id,
            table_id: p_table_id,
            prepare_time: p_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() -> Result<(), String> {
        let i = Item::new(1, 2, 3);

        assert_eq!(
            i,
            Item {
                item_id: 1,
                table_id: 2,
                prepare_time: 3,
            }
        );
        Ok(())
    }

    #[test]
    fn test_table_add_item() -> Result<(), String> {
        let mut t = Table::new();

        let item_id = 4;
        let table_id = 5;

        t.add_item(item_id, table_id);

        let i = t.items.get(&item_id).unwrap();

        assert_eq!(i.item_id, 4);
        assert_eq!(i.table_id, 5);
        assert_eq!(i.prepare_time >= 5 && i.prepare_time < 15, true);

        Ok(())
    }

    #[test]
    fn test_table_check_item() -> Result<(), String> {
        let mut t = Table::new();

        let item_id = 7;
        let table_id = 9;

        t.add_item(item_id, table_id);
        let i = t.check_item(item_id).unwrap();
        assert_eq!(i.item_id, item_id);

        let i2 = t.check_item(123);
        assert_eq!(i2, None);

        Ok(())
    }

    #[test]
    fn test_table_remove_item() -> Result<(), String> {
        let mut t = Table::new();

        let item_id = 11;
        let table_id = 99;

        t.add_item(item_id, table_id);

        let i = t.remove_item(item_id).unwrap();
        assert_eq!(i.item_id, item_id);

        let i2 = t.remove_item(item_id);
        assert_eq!(i2, None);

        Ok(())
    }
}
