use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

use super::item::Item;

pub struct Table {
    table_id: u32,
    items: HashMap<u32, Item>,
    rng: StdRng,
}

impl Table {
    pub fn new(tid: u32) -> Table {
        Table {
            table_id: tid,
            items: HashMap::new(),
            rng: StdRng::from_entropy(),
        }
    }

    #[cfg(test)]
    pub fn id(&self) -> u32 {
        self.table_id
    }

    #[cfg(test)]
    pub fn items_size(&self) -> usize {
        self.items.len()
    }

    pub fn add_item(&mut self, item_id: u32) {
        let item = Item::new(item_id, self.table_id, self.rng.gen_range(5..15));
        self.items.insert(item_id, item);
    }

    pub fn check_item(&self, item_id: u32) -> Option<&Item> {
        self.items.get(&item_id)
    }

    pub fn remove_item(&mut self, item_id: u32) -> Option<Item> {
        self.items.remove(&item_id)
    }

    pub fn print_item(&self, item_id: u32) -> String {
        let item = self.check_item(item_id);

        match item {
            Some(item) => {
                return item.print();
            }
            None => {
                return "{ msg: \"not found\"}".to_owned();
            }
        }
    }

    pub fn print_items(&self) -> String {
        let mut output = String::from("[");

        for (_, item) in self.items.iter() {
            let s = format!("{}", item.print());
            output += &s;
            output += ", ";
        }
        // pop the last ", "
        output.pop();
        output.pop();

        output += "]";

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_add_item() -> Result<(), String> {
        let table_id = 12;

        let mut t = Table::new(table_id);

        let item_id = 4;

        t.add_item(item_id);

        t.items.get(&item_id).unwrap();

        Ok(())
    }

    #[test]
    fn test_table_check_item() -> Result<(), String> {
        let mut t = Table::new(1);

        let item_id = 7;

        t.add_item(item_id);
        let i = t.check_item(item_id).unwrap();
        assert_eq!(i.id(), item_id);

        let i2 = t.check_item(123);
        assert_eq!(i2, None);

        Ok(())
    }

    #[test]
    fn test_table_remove_item() -> Result<(), String> {
        let mut t = Table::new(1);

        let item_id = 11;

        t.add_item(item_id);

        let i = t.remove_item(item_id).unwrap();
        assert_eq!(i.id(), item_id);

        let i2 = t.remove_item(item_id);
        assert_eq!(i2, None);

        Ok(())
    }
}
