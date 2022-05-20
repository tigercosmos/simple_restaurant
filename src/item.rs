#[derive(Debug, PartialEq)]
pub struct Item {
    item_id: u32,
    table_id: u32,
    prepare_time: u32,
}

impl Item {
    pub fn new(p_item_id: u32, p_table_id: u32, p_time: u32) -> Item {
        Item {
            item_id: p_item_id,
            table_id: p_table_id,
            prepare_time: p_time,
        }
    }

    #[cfg(test)]
    pub fn id(&self) -> u32 {
        self.item_id
    }

    pub fn print(&self) -> String {
        let s = format!(
            "{{item_id: {}, table_id: {}, prepare_time: {}}}",
            self.item_id, self.table_id, self.prepare_time
        );

        s
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
}
