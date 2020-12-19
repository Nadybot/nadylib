use lazy_static::lazy_static;
use serde_json::from_slice;

use std::collections::HashMap;

const MMDB_DATA: &[u8] = include_bytes!("../data/mmdb.json");

lazy_static! {
    static ref MMDB: HashMap<u32, HashMap<u32, String>> = from_slice(MMDB_DATA).unwrap();
}

pub fn get_message(category_id: u32, instance_id: u32) -> Option<String> {
    MMDB.get(&category_id)
        .map(|i| i.get(&instance_id))
        .flatten()
        .cloned()
}

// pub fn format_string<A>(string: &str, arguments: A) -> String
// where
//    A: FormatArgs,
// {
//    // TODO
//    string
// }

#[test]
fn test_mmdb_loading() {
    assert_eq!(MMDB.len(), 52);
}

#[test]
fn test_can_find_offline_msg() {
    let string = get_message(20000, 172363154);
    assert!(string.is_some());
}
