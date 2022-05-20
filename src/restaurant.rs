use std::sync::{Arc, Mutex};

use super::table::Table;

type TablePtr = Arc<Mutex<Table>>;

#[derive(Clone)]
pub struct Restaurant {
    tables: Vec<TablePtr>,
}

impl Restaurant {
    pub fn new(table_size: usize) -> Restaurant {
        let mut tables = Vec::new();

        tables.reserve(table_size);

        for tid in 0..table_size as u32 {
            tables.push(Arc::new(Mutex::new(Table::new(tid))));
        }

        Restaurant { tables: tables }
    }

    pub fn get_table(self, table_id: u32) -> TablePtr {
        Arc::clone(&self.tables[table_id as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_restaurant_get_table() {
        let r = Restaurant::new(10);

        for test_id in 0..4 {
            let r2 = r.clone();
            thread::spawn(move || {
                let t = r2.get_table(test_id);
                let id = t.lock().unwrap().id();

                assert_eq!(id, test_id)
            });
        }
    }

    #[test]
    fn test_restaurant_get_table_then_do_something() {
        let r = Restaurant::new(10);

        let desire_table_id = 0;

        let add_amount: usize = 1000;

        let mut handles = vec![];

        for test_val in 0..add_amount as u32 {
            let r2 = r.clone();
            let handle = thread::spawn(move || {
                let t = r2.get_table(desire_table_id); // same table

                t.lock().unwrap().add_item(test_val);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let t = r.get_table(desire_table_id);
        let len = t.lock().unwrap().items_size();
        assert_eq!(len, add_amount);
    }
}
