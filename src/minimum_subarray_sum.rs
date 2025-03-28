use std::{collections::HashMap, usize};

pub fn min_sub_array_sum(arr: &mut Vec<i32>, target: i32) -> Result<Vec<i32>, i32> {
    let mut sum_map: HashMap<usize, (i32, Vec<i32>)> = HashMap::new();

    for i in 0..arr.len() {
        let mut sum = 0;
        let mut subarray = Vec::new();
        for j in i..arr.len() {
            sum += arr[j];
            subarray.push(arr[j]);
        }
        sum_map.insert(arr.len() - i, (sum, subarray));
    }

    let mut min_elements = usize::MAX;
    let mut best_subarray: Option<Vec<i32>> = None;

    for (&num_elements, &(sum, ref subarray)) in &sum_map {
        if sum >= target && num_elements < min_elements {
            min_elements = num_elements;
            best_subarray = Option(subarray.clone());
        }
    }

    if let Some(subarray) = best_subarray {
        Ok(subarray)
    } else {
        Err(0)
    }
}
