use std::collections::HashMap;

use webcore::try_from::TryInto;
use webcore::value::{Value, value_type_name};

pub fn build_query_string( object: HashMap< &str, Value > ) -> String {
    let mut args: Vec< String > = vec![];

    for (key, value) in object {
        let done: bool = destructure(&key, value);

        if done {
            args.push(
                js!(
                    var value = @{value};
                    return encodeURIComponent(key) + (value !== null && value !== "" ? "=" + encodeURIComponent(value) : "")
            ).try_into().unwrap());
        }
    }

    return args.join("&");
}

fn destructure(key: &str, value: Value ) -> bool {
    let value_type = value_type_name(&value);

    if value_type == "Array" {
        let v_len = js!( return @{value}.length; ).try_into().unwrap();

        for i in 0..v_len {
            let val = js!(
                var v = @{value.clone()};
                return v[i];
            ).try_into().unwrap();

            destructure( &format!( "{}[{}]", key, i ), val );
        }

        return false;
    } else if value_type == "Object" {
        let value: HashMap< String, Value > = js!( return @{value}; ).try_into().unwrap();

        for (k, v) in value {
            destructure( &format!( "{}[{}]",  key, k ), v );
        }

        return false;
    } else {
        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_build_query_string() {
        let mut params: HashMap< &str, Value > = [
            ("a", Value::String( "b".into() )),
            ("c", Value::Number( 1.into() ))
        ].iter().cloned().collect();

        let mut queryS: &str = &build_query_string(params);
        let mut expected: &str = "a=b&c=1";

        assert!( queryS == expected );

        let params = [
            (";:@&=+$,/?%#", Value::String( ";:@&=+$,/?%#".into() ))
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "%3B%3A%40%26%3D%2B%24%2C%2F%3F%25%23=%3B%3A%40%26%3D%2B%24%2C%2F%3F%25%23";

        assert!( queryS == expected );

        let params = [
            ("ö", Value::String( "ö".into() ))
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "%C3%B6=%C3%B6";

        assert!( queryS == expected );

        // nested object
        let params = [
            ("a", js!( return {"b": 1, "c": 2}; ).try_into().unwrap())
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "a%5Bb%5D=1&a%5Bc%5D=2";

        assert!( queryS == expected );

        // deeper nested object
        let params = [
            ("a", js!( return {"b": {"c": 1, "d": 2}}; ).try_into().unwrap())
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "a%5Bb%5D%5Bc%5D=1&a%5Bb%5D%5Bd%5D=2";

        assert!( queryS == expected );

        // nested array
        let params = [
            ("a", js!( return [["x", "y"]]; ).try_into().unwrap())
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "a%5B0%5D%5B0%5D=x&a%5B0%5D%5B1%5D=y";

        assert!( queryS == expected );

        // nested array in object
        let params = [
            ("a", js!( return {"b": ["x", "y"]}; ).try_into().unwrap())
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "a%5Bb%5D%5B0%5D=x&a%5Bb%5D%5B1%5D=y";

        assert!( queryS == expected );

        // deeper nested object in array
        let params = [
            ("a", js!( return [{"b": 1, "c": 2}]; ).try_into().unwrap())
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "a%5B0%5D%5Bb%5D=1&a%5B0%5D%5Bc%5D=2";

        assert!( queryS == expected );

        // date
        let params = [
            ("a", js!( return new Date(0); ).try_into().unwrap())
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);

        let ds: String = js!(
            return encodeURIComponent(new Date(0).toString());
        ).try_into().unwrap();

        let expected = &format!( "a={}", ds );

        assert!( queryS == expected );

        // nulls
        let params = [
            ("a", Value::Null)
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "a";

        assert!( queryS == expected );

        // undefined
        let params = [
            ("a", Value::Undefined)
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "a";

        assert!( queryS == expected );

        // 0 (trivial)
        let params = [
            ("a", Value::Number( 0.into() ))
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "a=0";

        assert!( queryS == expected );

        // false (i.e. Boolean)
        let params = [
            ("a", Value::Bool( false.into() ))
        ].iter().cloned().collect();

        let queryS = &build_query_string(params);
        let expected = "a=false";

        assert!( queryS == expected );
    }
}

