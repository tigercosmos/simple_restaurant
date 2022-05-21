use super::restaurant::Restaurant;

pub fn add_item(tid: u32, item_data: &str, restaurant: Restaurant) -> String {
    let data = item_data.split(",").collect::<Vec<&str>>();
    let iid = data[0].parse::<u32>().unwrap();

    let t = restaurant.get_table(tid);
    t.lock().unwrap().add_item(iid);

    "{ msg: \"success\"}".to_owned()
}
pub fn remove_item(tid: u32, iid: u32, restaurant: Restaurant) -> String {
    let t = restaurant.get_table(tid);
    let result = t.lock().unwrap().remove_item(iid);
    match result {
        Some(_) => "{ msg: \"success\"}".to_owned(),
        None => {
            return "{ msg: \"cannot remove, not exist\"}".to_owned();
        }
    }
}
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

    #[test]
    fn test_api_remove_item() {
        let item_amount = 5;
        let item_id = 1;

        let r = create_restaurant(1, item_amount);
        let r2 = r.clone();
        let r3 = r.clone();

        let output = remove_item(0, item_id, r);
        assert_eq!(output.contains("success"), true);

        assert_eq!(
            r2.get_table(0).lock().unwrap().items_size(),
            item_amount - 1
        );

        let output2 = remove_item(0, item_id, r3);
        assert_eq!(output2.contains("cannot remove"), true);
    }

    #[test]
    fn test_api_add_item() {
        let item_amount = 5;

        let r = create_restaurant(1, item_amount);

        add_item(0, "999,", r.clone());

        assert_eq!(
            r.clone().get_table(0).lock().unwrap().items_size(),
            item_amount + 1
        );

        add_item(0, "777", r.clone());

        assert_eq!(
            r.clone().get_table(0).lock().unwrap().items_size(),
            item_amount + 2
        );
    }
}
