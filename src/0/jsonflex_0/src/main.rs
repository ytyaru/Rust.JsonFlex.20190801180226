#[warn(unused_imports)]
use json_flex::{JFObject, Unwrap};
use std::collections::HashMap;

fn main() {
    let array = json_flex::decode(r#"[1,2,3,4]"#.to_owned());
    println!("{:?}", array);
    println!("{:?}", array.to_json());

    let array = json_flex::decode(r#"["1","2","3","4"]"#.to_owned());
    println!("{:?}", array[0].into_string());
    println!("{:?}", array.to_json());

    let obj = json_flex::decode(r#"{"k1": "text", "k2": 100}"#.to_owned());
    println!("{:?}", obj);
//    println!("{:?}", obj.k1); // アクセスできず……
//    println!("{:?}", obj.k2); // アクセスできず……
    println!("{:?}", obj.into_hashmap());
    if let Some(map) = obj.into_hashmap() {
        println!("{:?}", map);
        println!("{:?}", map.get("k1"));
        println!("{:?}", map.get("k2"));
        println!("{:?}", map.get("k1").unwrap().unwrap_string()); // "text" クォーテーションついたまま……
        println!("{:?}", map.get("k2").unwrap().unwrap_i64()); // 1000
    }
    println!("{:?}", obj.to_json());
}
