pub struct List<T> {
    len: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, _element: T) {
        self.len += 1;
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

    #[test]
    fn test_push_increases_len() {
        let mut list: List<i32> = List::new();

        list.push(1);

        assert_eq!(list.len(), 1);
    }
}
