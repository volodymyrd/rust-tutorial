use crate::Sorter;

struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(data: &mut [T])
    where
        T: Ord,
    {
        for i in 0..data.len() {
            // let mut min = i;
            // for j in i+1..data.len() {
            //     if data[j] < data[min] {
            //         min = j;
            //     }
            // }
            let min = data[i..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(j, _)| i+j)
                .unwrap_or(i);

            data.swap(i, min);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Sorter;
    use crate::selectionsort::SelectionSort;

    #[test]
    fn test_one() {
        let mut slice = [1];
        SelectionSort::sort(&mut slice);

        assert_eq!(slice, [1]);
    }

    #[test]
    fn test_sorted() {
        let mut slice = [1, 2, 3, 4];
        SelectionSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_unsorted() {
        let mut slice = [4, 3, 1, 2];
        SelectionSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_reversed() {
        let mut slice = [4, 3, 2, 1];
        SelectionSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }
}
