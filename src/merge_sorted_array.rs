use std::{isize, usize};

pub fn merge_sorted_array(nums1: &mut Vec<i32>, m: usize, nums2: &mut Vec<i32>, n: usize) {
    let mut i = m as isize - 1;
    let mut j = n as isize - 1;
    let mut k = (m + n) as isize - 1;

    while j >= 0 {
        if i >= 0 && nums1[i as usize] >= nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sorted_array() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let m = 3;
        let n = 3;

        merge_sorted_array(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
