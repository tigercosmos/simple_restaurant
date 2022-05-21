use super::restaurant::Restaurant;

pub fn add_item() {}
pub fn remove_item() {}
pub fn query_all(tid: u32, restaurant: Restaurant) -> String {
    let t = restaurant.get_table(tid);
    let output = t.lock().unwrap().print_items();

    output
}
pub fn query_one(tid: u32, iid: u32, restaurant: Restaurant) -> String {
    let t = restaurant.get_table(tid);
    let s = t.lock().unwrap().print_item(iid);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_restaurant(table_n: usize, item_n: usize) -> Restaurant {
        let r = Restaurant::new(table_n);
        let t = r.get_table(0);
        for i in 0..item_n {
            t.lock().unwrap().add_item(i as u32);
        }
        r
    }

    #[test]
    fn test_api_query_all() {
        let r = create_restaurant(1, 2);

        let output = query_all(0, r);

        assert_eq!(output.contains("item_id: 0"), true);
        assert_eq!(output.contains("item_id: 1"), true);
    }

    #[test]
    fn test_api_query_one() {
        let r = create_restaurant(1, 2);
        let r2 = r.clone();

        let output = query_one(0, 1, r);
        assert_eq!(output.contains("item_id: 1"), true);

        let output2 = query_one(0, 3, r2);
        assert_eq!(output2.contains("{ msg: \"not found\"}"), true);
    }
}
