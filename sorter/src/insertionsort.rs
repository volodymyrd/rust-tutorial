use crate::Sorter;

struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(data: &mut [T])
    where
        T: Ord,
    {
        for i in 1..data.len() {
            let mut j = i;
            while j > 0 && data[j - 1] > data[j] {
                data.swap(j - 1, j);
                j-=1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Sorter;
    use crate::insertionsort::InsertionSort;

    #[test]
    fn test_one() {
        let mut slice = [1];
        InsertionSort::sort(&mut slice);

        assert_eq!(slice, [1]);
    }

    #[test]
    fn test_sorted() {
        let mut slice = [1, 2, 3, 4];
        InsertionSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_unsorted() {
        let mut slice = [4, 3, 1, 2];
        InsertionSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_reversed() {
        let mut slice = [4, 3, 2, 1];
        InsertionSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }
}
