use crate::error::{Error, Result};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{AddAssign, MulAssign};

pub fn from_str<'se, T: Serializable<'se>>(input: &'se str) -> Result<T> {
    let mut deserializer = Deserializer::new(input);
    T::deserialize(&mut deserializer)
}

pub trait Serializable<'se>: Sized {
    fn deserialize(deserializer: &mut Deserializer<'se>) -> Result<Self>;
}

// Impl for String
impl<'se> Serializable<'se> for String {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
        deserializer.parse_string().map(|s| s.to_string())
    }
}

impl<'se> Serializable<'se> for &'se str {
    fn deserialize(deserializer: &mut Deserializer<'se>) -> Result<Self> {
        deserializer.parse_string()
    }
}

// Impl for bool
impl<'se> Serializable<'se> for bool {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
        deserializer.parse_bool()
    }
}

// Impl for u32
impl<'se> Serializable<'se> for u32 {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
        deserializer.parse_unsigned()
    }
}

// Impl for u64
impl<'se> Serializable<'se> for u64 {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
        deserializer.parse_unsigned()
    }
}

impl<'se, K, V> Serializable<'se> for HashMap<K, V>
where
    K: Serializable<'se> + Eq + Hash,
    V: Serializable<'se>,
{
    fn deserialize(deserializer: &mut Deserializer<'se>) -> Result<Self> {
        let mut map = HashMap::new();

        // Parse the opening brace of the map.
        if deserializer.next_char()? != '{' {
            return Err(Error::ExpectedMap);
        }
        let mut first = true;
        loop {
            // Check for closing brace
            if deserializer.peek_char()? == '}' {
                deserializer.next_char()?; // Consume '}'
                break;
            }

            // Comma handling
            if !first {
                if deserializer.next_char()? != ',' {
                    return Err(Error::ExpectedMapComma);
                }
            } else {
                first = false;
            }

            // Deserialize key
            let key = K::deserialize(deserializer)?; // Deserialize K

            // Colon
            if deserializer.next_char()? != ':' {
                return Err(Error::ExpectedMapColon);
            }

            // Deserialize value
            let value = V::deserialize(deserializer)?; // Deserialize V

            map.insert(key, value);
        }
        Ok(map)
    }
}

impl<'se, V> Serializable<'se> for Vec<V>
where
    V: Serializable<'se>,
{
    fn deserialize(deserializer: &mut Deserializer<'se>) -> Result<Self> {
        let mut vec = Vec::new();

        // Parse the opening bracket of the array.
        if deserializer.next_char()? != '[' {
            return Err(Error::ExpectedArray);
        }

        let mut first = true;
        loop {
            // Check for closing bracket
            if deserializer.peek_char()? == ']' {
                deserializer.next_char()?; // Consume ']'
                break;
            }

            // Comma handling
            if !first {
                if deserializer.next_char()? != ',' {
                    return Err(Error::ExpectedArrayComma);
                }
            } else {
                first = false;
            }

            // Deserialize element
            let element = V::deserialize(deserializer)?;
            vec.push(element);
        }
        Ok(vec)
    }
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    fn new(input: &'de str) -> Self {
        Self { input }
    }

    fn peek_char(&self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    fn next_char(&mut self) -> Result<char> {
        let c = self.peek_char()?;
        self.input = &self.input[c.len_utf8()..];
        Ok(c)
    }

    // Parse the JSON identifier `true` or `false`.
    fn parse_bool(&mut self) -> Result<bool> {
        if self.input.starts_with("true") {
            self.input = &self.input["true".len()..];
            Ok(true)
        } else if self.input.starts_with("false") {
            self.input = &self.input["false".len()..];
            Ok(false)
        } else {
            Err(Error::ExpectedBoolean)
        }
    }

    // Parse a group of decimal digits as an unsigned integer of type T.
    //
    // This implementation is a bit too lenient, for example `001` is not
    // allowed in JSON. Also the various arithmetic operations can overflow and
    // panic or return bogus data. But it is good enough for example code!
    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8>,
    {
        let mut int = match self.next_char()? {
            ch @ '0'..='9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        loop {
            match self.input.chars().next() {
                Some(ch @ '0'..='9') => {
                    self.input = &self.input[1..];
                    int *= T::from(10);
                    int += T::from(ch as u8 - b'0');
                }
                _ => {
                    return Ok(int);
                }
            }
        }
    }

    // Parses a string until the next '"' character.
    //
    // Makes no attempt to handle escape sequences. What did you expect? This is
    // example code!
    fn parse_string(&mut self) -> Result<&'de str> {
        if self.next_char()? != '"' {
            return Err(Error::ExpectedString);
        }
        match self.input.find('"') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => Err(Error::Eof),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::desert::from_str;
    use std::collections::HashMap;

    #[test]
    fn test_deserialize_empty_hashmap() {
        assert_eq!(
            from_str::<HashMap<String, u32>>("{}").unwrap(),
            HashMap::new()
        );
        assert_eq!(
            from_str::<HashMap<String, String>>("{}").unwrap(),
            HashMap::new()
        );

        assert_eq!(from_str::<Vec<u32>>("[]").unwrap(), vec![]);
        assert_eq!(from_str::<Vec<String>>("[]").unwrap(), Vec::<String>::new());
    }

    #[test]
    fn test_deserialize_hashmap_with_string_key() {
        let expected = HashMap::from([("key".to_string(), 1)]);
        assert_eq!(
            from_str::<HashMap<String, u32>>(r#"{"key":1}"#).unwrap(),
            expected
        );
    }

    #[test]
    fn test_deserialize_hashmap_with_str_key_u32_val() {
        let expected = HashMap::from([("key1", 1)]);
        assert_eq!(
            from_str::<HashMap<&str, u32>>(r#"{"key1":1}"#).unwrap(),
            expected
        );
    }

    #[test]
    fn test_deserialize_hashmap_with_str_key_str_val() {
        let expected = HashMap::from([("key2", "2")]);
        assert_eq!(
            from_str::<HashMap<&str, &str>>(r#"{"key2":"2"}"#).unwrap(),
            expected
        );
    }

    #[test]
    fn test_deserialize_hashmap_with_str_key_bool_val() {
        let expected = HashMap::from([("key3", true)]);
        assert_eq!(
            from_str::<HashMap<&str, bool>>(r#"{"key3":true}"#).unwrap(),
            expected
        );
    }

    #[test]
    fn test_deserialize_vec_i32() {
        let expected = r#"[1,2,3]"#;

        assert_eq!(from_str::<Vec<u32>>(expected).unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_deserialize_empty_vec() {
        let expected = r#"[]"#;

        assert!(from_str::<Vec<u32>>(expected).unwrap().is_empty());
    }
}
