#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delimiter) = remainder.find(self.delimiter) {
            let until_delimeter = &remainder[..next_delimiter];
            *remainder = &remainder[(next_delimiter + self.delimiter.len())..];
            Some(until_delimeter)
        } else {
            self.remainder.take()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let splitted = StrSplit::new(haystack, " ");
        assert!(splitted.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    }

    #[test]
    fn tail() {
        let haystack = "a b c d e ";
        let splitted: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(splitted, vec!["a", "b", "c", "d", "e", ""]);
    }
}
