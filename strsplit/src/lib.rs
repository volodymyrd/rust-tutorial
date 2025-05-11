#[derive(Debug)]
pub struct StrSplit<'r, 'd> {
    remainder: Option<&'r str>,
    delimiter: &'d str,
}

impl<'r, 'd> StrSplit<'r, 'd> {
    pub fn new(haystack: &'r str, delimiter: &'d str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'r, 'd> Iterator for StrSplit<'r, 'd> {
    type Item = &'r str;
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

pub fn until_char<'r>(s: &'r str, c: char) -> &'r str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always gives al least one result.")
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

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("Hello World", 'r'), "Hello Wo");
    }
}
