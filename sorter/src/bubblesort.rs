use crate::Sorter;

struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(data: &mut [T])
    where
        T: Ord,
    {
        if data.len() < 2 {
            return;
        }

        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..data.len() - 1 {
                if data[i] > data[i + 1] {
                    swapped = true;
                    data.swap(i, i + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Sorter;
    use crate::bubblesort::BubbleSort;

    #[test]
    fn test_one() {
        let mut slice = [1];
        BubbleSort::sort(&mut slice);

        assert_eq!(slice, [1]);
    }

    #[test]
    fn test_sorted() {
        let mut slice = [1, 2, 3, 4];
        BubbleSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_unsorted() {
        let mut slice = [4, 3, 1, 2];
        BubbleSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_reversed() {
        let mut slice = [4, 3, 2, 1];
        BubbleSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }
}
