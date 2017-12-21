use std::borrow::Cow;

use webcore::try_from::TryInto;

extern crate serde_json;
use serde_json::Value as JsonValue;

struct StateFn< 'a > {
    f: &'a Fn( &StateFn, &str, &JsonValue, &Vec< String > ) -> Cow< 'a, String >
}

// undefined is not supported, but "undefined" is. This is because serde_json does not implement an
// Undefined enum (only Null)
pub fn build_query_string( jvalue: &JsonValue ) -> String {
    let d_state = StateFn {
        f: &|d_state, key, jvalue, args| {
            // using a clone because jvalue is being *consumed* by iterator methods and thus won't
            // fit the type checks (throwing a "cannot move out of borrowed content" error)
            let tmp_jvalue = jvalue.clone();

            match tmp_jvalue {
                JsonValue::Array( tmp_jvalue ) => {
                    let v_len = tmp_jvalue.len();
                    let mut tmp_args = args.clone();

                    for i in 0..v_len {
                        tmp_args.push(
                            (d_state.f)( d_state,  &format!( "{}[{}]", key, i ), &tmp_jvalue[i], args ).to_string()
                        );
                    }

                    return Cow::Owned(tmp_args.join("&")) as Cow< String >;
                },
                JsonValue::Object( tmp_jvalue ) => {
                    let mut tmp_args = args.clone();

                    for (k, v) in tmp_jvalue {
                        tmp_args.push(
                            (d_state.f)( d_state, &format!( "{}[{}]",  key, k ), &v, args ).to_string()
                        );
                    }

                    return Cow::Owned(tmp_args.join("&")) as Cow< String >;
                },

                // TODO: cleanup match block...
                JsonValue::Bool( tmp_jvalue ) => {
                    let val_string: String = format!("{}", tmp_jvalue);
                    let return_val: String = encode_kv(key, &val_string);

                    return Cow::Owned(return_val) as Cow< String >;
                },

                JsonValue::Number( tmp_jvalue ) => {
                    let val_string: String = format!("{}", tmp_jvalue);
                    let return_val: String = encode_kv(key, &val_string);

                    return Cow::Owned(return_val) as Cow< String >;
                },

                JsonValue::String( tmp_jvalue ) => {
                    return Cow::Owned(encode_kv(key, &tmp_jvalue)) as Cow< String >;
                },
                _ => {
                    let return_val: String = format!("{}", key.to_string());
                    return Cow::Owned(return_val) as Cow< String >
                }
            };
        }
    };

    // using a clone because jvalue is being *consumed* by iterator methods and thus won't
    // fit the type checks (throwing a "cannot move out of borrowed content" error)
    let tmp_jvalue = jvalue.clone();
    let mut return_val: String = "".to_string();

    // force jvalue to be an object, thus representing a valid JSON blob
    match tmp_jvalue {
        JsonValue::Object( tmp_jvalue ) => {
            let mut strings2 = vec![];

            for (key, value) in tmp_jvalue.into_iter() {
                let strings = vec![];
                let result_str = (d_state.f)(&d_state, &key, &value, &strings);
                strings2.push( result_str.to_string() );
            }

            return_val = strings2.join("&");
        },
        _ => ()
    };

    return return_val;
}

fn encode_kv(key: &str, val_string: &str) -> String {
    let s: String = js!(
        return encodeURIComponent( @{key} ) + "=" + encodeURIComponent( @{val_string} );
    ).try_into().unwrap();

    return s;
}

