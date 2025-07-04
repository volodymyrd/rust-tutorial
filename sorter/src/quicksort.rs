use crate::Sorter;
use std::fmt::Debug;

struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(data: &mut [T])
    where
        T: Ord + Debug,
    {
        if data.len() < 2 {
            return;
        }

        quick_sort(data, 0, data.len() - 1);
    }
}

fn quick_sort<T: Ord + Debug>(data: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let pivot_index = partition(data, low, high);

    if pivot_index > low {
        // Ensure we don't underflow for the left subarray
        quick_sort(data, low, pivot_index - 1);
    }
    if pivot_index < high {
        // Ensure we don't overflow for the right subarray (though less likely here)
        quick_sort(data, pivot_index + 1, high);
    }
}

fn partition<T: Ord + Debug>(data: &mut [T], low: usize, high: usize) -> usize {
    let mut i = low; // Index of smaller element

    for j in low..high {
        if data[j] <= data[high] {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, high); // Put the pivot in its correct sorted position
    i // Return the pivot's final index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut slice = [1];
        QuickSort::sort(&mut slice);

        assert_eq!(slice, [1]);
    }

    #[test]
    fn test_sorted() {
        let mut slice = [1, 2, 3, 4];
        QuickSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_unsorted() {
        let mut slice = [4, 3, 1, 2];
        QuickSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }

    #[test]
    fn test_reversed() {
        let mut slice = [4, 3, 2, 1];
        QuickSort::sort(&mut slice);

        assert_eq!(slice, [1, 2, 3, 4]);
    }
}
