/*
You are given an integer array nums. You are initially positioned at the array's first index,
and each element in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.

Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.

*/

pub fn jump_game(arr: &mut Vec<i32>) -> bool {
    let mut farthest = 0;

    for i in 0..arr.len() {
        if i > farthest {
            return false;
        }

        farthest = farthest.max(i + arr[i] as usize);
        if farthest >= arr.len() - 1 {
            return true;
        }
    }
    return true;
}
