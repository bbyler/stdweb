use std::collections::BTreeMap;

use webcore::try_from::TryInto;
use webcore::value::Value;

extern crate serde_json;

#[macro_use]
use serde_json::Value as JsonValue;

pub fn build_query_string( jvalue: &JsonValue ) -> String {
    let mut args: Vec< String > = vec![];

    // force jvalue to be an object, thus representing a valid JSON blob
    match jvalue {
        &JsonValue::Object( jvalue ) => {
            for (key, value) in jvalue.into_iter() {
                args.push( destructure(&key, &value) );
                // have to reverse args since the args.push op is done when unwinding the recursive function
                // call stack
                // TODO: optimize?
                args.reverse();
            }

            return args.join("&");
        },
        _ => ()
    };

    return "".to_string();
}

fn destructure(key: &str, jvalue: &JsonValue ) -> String {
    let result = match jvalue {
        &JsonValue::Array( jvalue ) => {
            let mut vector: Vec< Value > = Vec::new();

            vector.reserve( jvalue.len() );
            for element in jvalue.into_iter() {
                if let Ok( element ) = element.try_into() {
                    vector.push( element );
                }
            }

            let v_tmp = Value::Array( vector.into() );
            let v_len = js!( return @{v_tmp}.length; ).try_into().unwrap();

            for i in 0..v_len {
                let val = js!(
                    var v = @{v_tmp.as_ref()};
                    return v[i];
                ).try_into().unwrap();

                destructure( &format!( "{}[{}]", key, i ), &val );
            }
        },
        &JsonValue::Object( value ) => {
            let mut map: BTreeMap< String, Value > = BTreeMap::new();
            for (key, value) in jvalue.into_iter() {
                if let Ok( value ) = value.try_into() {
                    map.insert( key, value );
                }
            }

            let v_tmp = Value::Object( map.into() );

            for (k, v) in v_tmp() {
                destructure( &format!( "{}[{}]",  key, k ), &v );
            }
        },

        // TODO: cleanup match block...
        &JsonValue::Bool( jvalue ) => {
            js!(
                var value = @{jvalue};
                return encodeURIComponent(@{key}) + (value !== null && value !== "" ? "=" + encodeURIComponent(value) : "")
            ).try_into().unwrap()
        },

        &JsonValue::Number( jvalue ) => {
            js!(
                var value = @{jvalue};
                return encodeURIComponent(@{key}) + (value !== null && value !== "" ? "=" + encodeURIComponent(value) : "")
            ).try_into().unwrap()
        },

        &JsonValue::String( jvalue ) => {
            js!(
                var value = @{jvalue};
                return encodeURIComponent(@{key}) + (value !== null && value !== "" ? "=" + encodeURIComponent(value) : "")
            ).try_into().unwrap()
        },
        _ => ()
    };

    return result;
}

/*
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_basic_hash() {
        let params: HashMap< &str, Value > = [
            ("a", Value::String( "b".into() )),
            ("c", Value::Number( 1.into() ))
        ].iter().cloned().collect();

        assert!(
            &build_query_string(params) == "a=b&c=1"
        );
    }

    #[test]
    fn test_encoding() {
        let params: HashMap< &str, Value > = [
            (";:@&=+$,/?%#", Value::String( ";:@&=+$,/?%#".into() ))
        ].iter().cloned().collect();

        assert!(
            &build_query_string(params) == "%3B%3A%40%26%3D%2B%24%2C%2F%3F%25%23=%3B%3A%40%26%3D%2B%24%2C%2F%3F%25%23"
        );

        let params = [
            ("ö", Value::String( "ö".into() ))
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);
        let expected = "%C3%B6=%C3%B6";

        assert!( query_s == expected );
    }

    #[test]
    fn test_build_query_string() {
        // nested object
        let params: HashMap< &str, Value > = [
            ("a", js!( return {"b": 1, "c": 2}; ).try_into().unwrap())
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);

        assert!( query_s == &"a%5Bb%5D=1&a%5Bc%5D=2" );

        // deeper nested object
        let params = [
            ("a", js!( return {"b": {"c": 1, "d": 2}}; ).try_into().unwrap())
        ].iter().cloned().collect();

        assert!(
            &build_query_string(params) == "a%5Bb%5D%5Bc%5D=1&a%5Bb%5D%5Bd%5D=2"
        );

        // nested array
        let params = [
            ("a", js!( return [["x", "y"]]; ).try_into().unwrap())
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);
        let expected = "a%5B0%5D%5B0%5D=x&a%5B0%5D%5B1%5D=y";

        assert!( query_s == expected );

        // nested array in object
        let params = [
            ("a", js!( return {"b": ["x", "y"]}; ).try_into().unwrap())
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);
        let expected = "a%5Bb%5D%5B0%5D=x&a%5Bb%5D%5B1%5D=y";

        assert!( query_s == expected );

        // deeper nested object in array
        let params = [
            ("a", js!( return [{"b": 1, "c": 2}]; ).try_into().unwrap())
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);
        let expected = "a%5B0%5D%5Bb%5D=1&a%5B0%5D%5Bc%5D=2";

        assert!( query_s == expected );

        // date
        let params = [
            ("a", js!( return new Date(0); ).try_into().unwrap())
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);

        let ds: String = js!(
            return encodeURIComponent(new Date(0).toString());
        ).try_into().unwrap();

        let expected = &format!( "a={}", ds );

        assert!( query_s == expected );

        // nulls
        let params = [
            ("a", Value::Null)
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);
        let expected = "a";

        assert!( query_s == expected );

        // undefined
        let params = [
            ("a", Value::Undefined)
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);
        let expected = "a";

        assert!( query_s == expected );

        // 0 (trivial)
        let params = [
            ("a", Value::Number( 0.into() ))
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);
        let expected = "a=0";

        assert!( query_s == expected );

        // false (i.e. Boolean)
        let params = [
            ("a", Value::Bool( false.into() ))
        ].iter().cloned().collect();

        let query_s = &build_query_string(params);
        let expected = "a=false";

        assert!( query_s == expected );
    }
}
*/

