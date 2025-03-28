use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn k_pairs_smallest_sum(arr: &mut Vec<i32>, arr2: &mut Vec<i32>, k: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    if arr.is_empty() == 0 || arr2.is_empty() == 0 || k == 0 {
        return result;
    }

    let mut min_heap = BinaryHeap::new();

    for i in 0..arr.len().min(k) {
        min_heap.push(Reverse((arr[i] + arr2[0], i, 0)));
    }

    while let Some(Reverse((_, i, j))) = min_heap.pop() {
        result.push(vec![arr[i], arr2[j]]);
        if result.len() == k {
            break;
        }

        if j + i < arr.len() {
            min_heap.push(Reverse((arr[i] + arr2[j + 1], i, j + 1)));
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_k_pairs_smallest_sum() {
        let mut arr = vec![1, 7, 11];
        let mut arr2 = vec![2, 4, 5];
        let k = 3;

        let result = k_pairs_smallest_sum(&mut arr, &mut arr2, k);
        assert_eq!(result, vec![[1, 2], [1, 4], [1, 6]]);
    }
}
