
use super::table::Table;

pub struct Restaurant {
    tables: Vec<Table>,
}

impl Restaurant {
    pub fn new(table_size: usize) -> Restaurant {

        let mut tables:Vec<Table> = Vec::new();

        tables.reserve(table_size);

        for _ in 0..table_size {
            tables.push(Table::new());
        }

        Restaurant {
            tables: tables
        }
    }

}
