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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(),
            0
        )
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
        assert_eq!(
            flatten(vec![vec!["a", "b"], vec!["c", "d"]]).count(),
            4
        )
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }

    #[test]
    fn eight() {
        assert_eq!(
            flatten(
                vec![
                    vec!["a", "b"],
                    vec!["c", "d"],
                    vec!["e", "f"],
                    vec!["g", "h"]
                ]
            )
            .count(),
            8
        )
    }
}
