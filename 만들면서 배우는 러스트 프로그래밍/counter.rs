use std::collections::HashMap;

const V_DATA: &str = "C,C,A,A,A,B,D,C,C,F,D,B,B,B,C,B,C,B,A,C,C,B,C,C,C";

fn main() {
    let mut c_map = HashMap::new();

    for w in V_DATA.split(",") {
        if c_map.get(w) == None {
            c_map.insert(w.to_string(), 1);
        } else {
            c_map.insert(w.to_string(), c_map[w] + 1);
        }
    }

    for (key, value) in &c_map {
        println!("{key}: {value}");
    }
}
