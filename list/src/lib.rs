pub struct List<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        0
    }

    pub fn is_empty(&self) -> bool {
        true
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list_is_empty_and_has_zero_len() {
        let list: List<i32> = List::new();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty())
    }
}
