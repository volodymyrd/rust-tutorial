use std::fmt::Debug;

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;

trait Sorter {
    fn sort<T>(data: &mut [T])
    where
        T: Ord +Debug;
}

#[cfg(test)]
mod tests {
    use crate::Sorter;

    struct StdSorter;

    impl Sorter for StdSorter {
        fn sort<T>(data: &mut [T])
        where
            T: Ord,
        {
            data.sort();
        }
    }

    #[test]
    fn test_one() {
        let mut slice = [1];
        StdSorter::sort(&mut slice);

        assert_eq!(slice, [1]);
    }

    #[test]
    fn test_sorted() {
        let mut slice = [1, 2, 3, 4];
        StdSorter::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_unsorted() {
        let mut slice = [4, 3, 1, 2];
        StdSorter::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_reversed() {
        let mut slice = [4, 3, 2, 1];
        StdSorter::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }
}
