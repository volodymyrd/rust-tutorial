#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delimiter) = self.remainder.find(self.delimiter) {
            let until_delimeter = &self.remainder[..next_delimiter];
            self.remainder = &self.remainder[next_delimiter + self.delimiter.len()..];
            Some(until_delimeter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
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
        //for e in splitted {
        //    println!("splitted: {e}");
        //}
        assert!(splitted.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    }
}
