pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    // added on the step 3 of improvements
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
    inner_back: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O: Iterator> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(o: O) -> Self {
        Flatten {
            outer: o,
            inner: None,
            inner_back: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        // 1:
        //self.outer.next().and_then(|o| o.into_iter().next())

        //2:
        // let inner_item = self.outer.next()?;
        // let mut inner_item = inner_item.into_iter();
        // inner_item.next()

        //3:
        loop {
            // if let Some(inner_iter) = self.inner.as_mut() {
            // if let Some(inner_iter) = &mut self.inner {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }
                self.inner = None;
            }
            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
            //self.inner.as_mut().unwrap().next()
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner_back {
                if let Some(item) = inner_iter.next_back() {
                    return Some(item);
                }
                self.inner_back = None;
            }
            let next_inner_iter = self.outer.next_back()?.into_iter();
            self.inner_back = Some(next_inner_iter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0)
    }

    #[test]
    fn once() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }

    #[test]
    fn two_4() {
        assert_eq!(flatten(vec![vec!["a", "b"], vec!["c", "d"]]).count(), 4)
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }

    #[test]
    fn eight() {
        assert_eq!(
            flatten(vec![
                vec!["a", "b"],
                vec!["c", "d"],
                vec!["e", "f"],
                vec!["g", "h"]
            ])
            .count(),
            8
        )
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec!(vec!["a"], vec!["b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn deep() {
        assert_eq!(
            flatten(flatten(vec![vec![vec![1, 2, 3]]])).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }
}
