use webcore::try_from::{TryFrom, TryInto};
use webcore::value::{Reference, Value, ConversionError, FromReferenceUnchecked};
use webcore::serialization::{JsSerializable, deserialize_array};

/// A type representing a JavaScript array.
#[derive(Clone, PartialEq, Debug)]
pub struct Array( Reference );

impl FromReferenceUnchecked for Array {
    unsafe fn from_reference_unchecked( reference: Reference ) -> Self {
        Array( reference )
    }
}

impl From< Array > for Reference {
    fn from( array: Array ) -> Self {
        array.0.clone()
    }
}

impl Array {
    #[inline]
    pub(crate) fn as_reference( &self ) -> &Reference {
        &self.0
    }

    /// Returns the number of elements in this particular array.
    pub fn len( &self ) -> usize {
        js!(
            return @{self}.length;
        ).try_into().unwrap()
    }
}

impl From< Array > for Vec< Value > {
    fn from( array: Array ) -> Self {
        deserialize_array( &array.0, |iter| iter.collect() )
    }
}

impl< 'a > From< &'a Array > for Vec< Value > {
    fn from( array: &'a Array ) -> Self {
        deserialize_array( &array.0, |iter| iter.collect() )
    }
}

impl< 'a > From< &'a mut Array > for Vec< Value > {
    fn from( array: &'a mut Array ) -> Self {
        deserialize_array( &array.0, |iter| iter.collect() )
    }
}

impl< V > From< Vec< V > > for Array where V: JsSerializable {
    #[inline]
    fn from( value: Vec< V > ) -> Self {
        let value: &[V] = &value;
        value.into()
    }
}

impl< 'a, V > From< &'a Vec< V > > for Array where V: JsSerializable {
    #[inline]
    fn from( value: &'a Vec< V > ) -> Self {
        let value: &[V] = &value;
        value.into()
    }
}

impl< 'a, V > From< &'a mut Vec< V > > for Array where V: JsSerializable {
    #[inline]
    fn from( value: &'a mut Vec< V > ) -> Self {
        let value: &[V] = &value;
        value.into()
    }
}

impl< 'a, V > From< &'a [V] > for Array where V: JsSerializable {
    #[inline]
    fn from( value: &'a [V] ) -> Self {
        // TODO: Do something more efficient here?
        let value = js! {
            return @{value};
        };

        match value {
            Value::Array( array ) => return array,
            _ => unreachable!()
        }
    }
}

impl< 'a, V > From< &'a mut [V] > for Array where V: JsSerializable {
    #[inline]
    fn from( value: &'a mut [V] ) -> Self {
        let value: &[V] = value;
        value.into()
    }
}

impl< E: Into< ConversionError >, V: TryFrom< Value, Error = E > > TryFrom< Array > for Vec< V > {
    type Error = ConversionError;

    fn try_from( array: Array ) -> Result< Self, Self::Error > {
        deserialize_array( array.as_reference(), |deserializer| {
            let mut output = Vec::with_capacity( deserializer.len() );
            for value in deserializer {
                let result: Result< _, E > = value.try_into();
                let value = match result {
                    Ok( value ) => value,
                    Err( error ) => {
                        return Err( ConversionError::value_conversion_error( error.into() ) );
                    }
                };

                output.push( value );
            }
            Ok( output )
        })
    }
}
