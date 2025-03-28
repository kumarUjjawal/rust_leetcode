// mod jump_game;
mod merge_sorted_array;
// mod longest_substring;
// mod reverse_words_string;
fn main() {
    // let mut arr = vec![3, 2, 1, 0, 4];
    // let result = jump_game::jump_game(&mut arr);
    // println!("Result of [2,3,1,1,4] is {}", result);
    //
    // let s = "abcabdksf";
    // let longest = longest_substring::longest_substring(s);
    // println!(
    //     "Longest substring without repeating characters: {}",
    //     longest
    // );
    //
    // let target = 4;
    // match minimum_subarray_sum::min_sub_array_sum(&mut arr, target) {
    //     Ok(subarray) => println!("{:?}", subarray),
    //     Err(0) => println!("0"),
    // }
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 2;
    let result = merge_sorted_array::merge_sorted_array(&mut nums1, m, &mut nums2, n);
    println!("{:?}", result);
}
