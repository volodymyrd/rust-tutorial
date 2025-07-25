use std::collections::HashMap;
use std::error::Error;

pub fn from_str<T: Deserialize<T>>(input: &str) -> Result<T, Box<dyn Error>> {
    Ok(T::deserialize(input)?)
}

pub trait Deserialize<T> {
    fn deserialize(input: &str) -> Result<T, Box<dyn Error>>;
}

impl<K, V> Deserialize<HashMap<K, V>> for HashMap<K, V> {
    fn deserialize(input: &str) -> Result<HashMap<K, V>, Box<dyn Error>> {
        for c in input.chars() {
            if c == '{' {

            }
        }

        Ok(HashMap::new())
    }
}


impl<V> Deserialize<Vec<V>> for Vec<V> {
    fn deserialize(input: &str) -> Result<Vec<V>, Box<dyn Error>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use crate::deser::from_str;
    use std::collections::HashMap;

    #[test]
    fn test_empty_object() {
        assert_eq!(from_str::<HashMap<i32, i32>>("{}").unwrap(), HashMap::new());
        assert_eq!(
            from_str::<HashMap<i32, String>>("{}").unwrap(),
            HashMap::new()
        );
        assert_eq!(
            from_str::<HashMap<String, String>>("{}").unwrap(),
            HashMap::new()
        );

        assert_eq!(from_str::<Vec<i32>>("[]").unwrap(), vec![]);
        assert_eq!(from_str::<Vec<String>>("[]").unwrap(), Vec::<String>::new());
    }

    // #[test]
    // fn test_object() {
    //     let expected = HashMap::from([("key", 1)]);
    //     assert_eq!(
    //         from_str::<HashMap<&str, i32>>(r#"{"key":1}"#).unwrap(),
    //         expected
    //     );
    // }

    // #[test]
    // fn test() {
    //     #[derive(Debug, PartialEq)]
    //     struct Test {
    //         int: i32,
    //         string: String,
    //     }
    //     impl Deserialize<Test> for Test {
    //         fn deserialize() -> Result<Test, Box<dyn Error>> {
    //             Ok(Test {
    //                 int: 1,
    //                 string: "test".to_string(),
    //             })
    //         }
    //     }
    //
    //     let json = r#"{"int":1,"string":"test"}"#;
    //     let expected = Test {
    //         int: 1,
    //         string: "test".to_string(),
    //     };
    //
    //     assert_eq!(from_str::<Test>(json).unwrap(), expected);
    // }
}
