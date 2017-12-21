/*
use std::collections::HashMap;

extern crate serde_json;
use serde_json::Value as JsonValue;

use webcore::try_from::TryInto;

pub fn parse( s: String ) -> JsonValue {
    if s == "".to_string() {
        return json!({});
    }

    let entries: Vec< &str > = s.split("&").collect();
    let data: JsonValue = json!({});
    let entries_len = entries.len();
    let mut counters: HashMap< String, u32 >;

    for i in 0..entries_len {
        let entry: Vec< &str > = entries[i].split("=").collect();
        let entry_key = entry[0];

        let key: String = js!(
            var entry = @{entry_key};
            return decodeURIComponent(entry_key);
        ).try_into().unwrap();

        let mut value: String = "".to_string();

        if entry.len() == 2 {
            let entry_val = entry[1];

            value = js!(
                var entry = @{entry_val};
                return decodeURIComponent(entry_val);
            ).try_into().unwrap();
        }

        let parse_str = r#"/\]\[?|\[/"#;
        let mut levels: Vec< &str > = key.split(parse_str).collect();
        let mut cursor: HashMap< String, String >;

        // indexOf
        if key.contains("[" ) {
            levels.pop();
        }

        let levels_len = levels.len();

        for j in 0..levels_len {
            let mut level = levels[j];
            let next_level = levels[j + 1];

            let match_num: bool = match next_level.parse::< i32 >() {
                Ok(_) => true,
                Err(_) => false
            };

            let is_number: bool = next_level == "" || match_num;
            let is_value: bool = j == levels_len - 1;

            if level == "" {
                let key = levels[0..j].join("");

                if counters.contains_key(&key) {
                    counters[&key] = 0;
                };

                let level = &format!("{}", counters[&key] + 1);
            }

            match cursor.get(level) {
                Some(_) => (),
                None => {
                    if is_value {
                        cursor[level] = value;
                    } else if is_number {
                        cursor[level] = r#"[]"#.to_string();
                    } else if !is_number {
                        cursor[level] = r#"{}"#.to_string();
                    }
                }
            };

            // cursor = cursor[level];
        }
    }

    return data;
}

*/
