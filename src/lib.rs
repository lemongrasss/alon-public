//! A simple implementation of the quicksort algorithm in Rust.

/// Sorts a slice in-place using the quicksort algorithm.
/// 
/// # Arguments
/// 
/// * `arr` - A mutable slice of elements that implement PartialOrd and Copy
/// 
/// # Examples
/// 
/// ```
/// use quicksort::quicksort;
/// 
/// let mut arr = [3, 6, 8, 10, 1, 2, 1];
/// quicksort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 6, 8, 10]);
/// ```
pub fn quicksort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    if arr.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    
    quicksort(left);
    quicksort(&mut right[1..]);
}

/// Partitions the array around a pivot element.
/// Returns the index where the pivot ends up.
fn partition<T>(arr: &mut [T]) -> usize
where
    T: PartialOrd + Copy,
{
    let len = arr.len();
    let pivot_index = len - 1;
    let pivot = arr[pivot_index];
    let mut i = 0;
    
    for j in 0..pivot_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, pivot_index);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: [i32; 0] = [];
        quicksort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_single_element() {
        let mut arr = [42];
        quicksort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = [5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_array() {
        let mut arr = [3, 6, 8, 10, 1, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 3, 6, 8, 10]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = [5, 2, 8, 2, 9, 1, 5, 4];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 4, 5, 5, 8, 9]);
    }

    #[test]
    fn test_with_floats() {
        let mut arr = [3.14, 2.71, 1.41, 1.73];
        quicksort(&mut arr);
        assert_eq!(arr, [1.41, 1.73, 2.71, 3.14]);
    }

    #[test]
    fn test_large_array() {
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        let expected: Vec<i32> = (0..1000).collect();
        
        quicksort(&mut arr);
        assert_eq!(arr, expected);
    }
}