#[cfg(test)]
mod tests {
    use ecosystem::m::querystring::build::build_query_string;

    extern crate serde_json;
    use serde_json::Value as JsonValue;

    #[test]
    fn test_basic_hash() {
        let data: JsonValue = json!({
            "a": "b",
            "c": 1
        });

        let query_s = &build_query_string(&data);

        assert!( query_s == "a=b&c=1" );
    }

    #[test]
    fn test_encoding() {
        let data: JsonValue = json!({
            ";:@&=+$,/?%#": ";:@&=+$,/?%#"
        });

        let query_s = &build_query_string(&data);

        assert!( query_s == "%3B%3A%40%26%3D%2B%24%2C%2F%3F%25%23=%3B%3A%40%26%3D%2B%24%2C%2F%3F%25%23" );

        let data: JsonValue = json!({
            "ö": "ö"
        });

        let query_s = &build_query_string(&data);
        let expected = "%C3%B6=%C3%B6";

        assert!( query_s == expected );
    }

    #[test]
    fn test_nested_objects() {
        // nested object
        let data: JsonValue = json!({
            "a": {
                "b": 1,
                "c": 2
            }
        });

        let query_s = &build_query_string(&data);

        println!("query_s: {}", query_s);

        assert!( query_s == "a%5Bb%5D=1&a%5Bc%5D=2" );

        // deeper nested object
        let data: JsonValue = json!({
            "a": {
                "b": {
                    "c": 1, "d": 2
                }
            }
        });

        let query_s = &build_query_string(&data);

        assert!( query_s == "a%5Bb%5D%5Bc%5D=1&a%5Bb%5D%5Bd%5D=2" );

        // nested array
        let data: JsonValue = json!({
            "a": [["x", "y"]]
        });

        let query_s = &build_query_string(&data);
        let expected = "a%5B0%5D%5B0%5D=x&a%5B0%5D%5B1%5D=y";

        assert!( query_s == expected );

        // nested array in object
        let data: JsonValue = json!({
            "a": {
                "b": ["x", "y"]
            }
        });

        let query_s = &build_query_string(&data);
        let expected = "a%5Bb%5D%5B0%5D=x&a%5Bb%5D%5B1%5D=y";

        assert!( query_s == expected );

        // deeper nested object in array
        let data = json!({
            "a": [
              {
                  "b": 1,
                  "c": 2
              }
            ]
        });

        let query_s = &build_query_string(&data);
        let expected = "a%5B0%5D%5Bb%5D=1&a%5B0%5D%5Bc%5D=2";

        assert!( query_s == expected );
    }

    // revisit this one...
    // #[test]
    // fn test_date() {
    //     let d: Date = js!( return new Date(0); ).try_into().unwrap();

    //     // date
    //     let data: JsonValue = json!({
    //         "a": d
    //     });

    //     let query_s = &build_query_string(&data);

    //     let ds: String = js!(
    //         return encodeURIComponent(@{d.as_ref()}.toString());
    //     ).try_into().unwrap();

    //     let expected = &format!( "a={}", ds );

    //     assert!( query_s == expected );
    // }

    #[test]
    fn test_nulls() {
        // nulls
        let data: JsonValue = json!({
            "a": JsonValue::Null
        });

        let query_s = &build_query_string(&data);
        let expected = "a";

        println!("query_s: {}", query_s);
        assert!( query_s == expected );

        // 0 (trivial)
        let data: JsonValue = json!({
            "a": 0
        });

        let query_s = &build_query_string(&data);
        let expected = "a=0";

        assert!( query_s == expected );

        // false (i.e. Boolean)
        let data: JsonValue = json!({
            "a": false
        });

        let query_s = &build_query_string(&data);
        let expected = "a=false";

        assert!( query_s == expected );
    }

    ///////////////////////////////////////////////////////////
    // parse tests
    ///////////////////////////////////////////////////////////

    #[test]
    fn test_parse() {
        // let data = parse("?aaa=bbb".to_string());
        // let expected: JsonValue = json!({
        //     "aaa": "bbb"
        // });

        // assert!( data == expected );
    }
}

