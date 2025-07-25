
#[cfg(test)]
mod tests {
    use std::collections::{HashMap};

    #[test]
    fn test_empty_object() {
        assert_eq!(
            serde_json::from_str::<HashMap<i32, i32>>("{}").unwrap(),
            HashMap::new()
        );
    }
}
