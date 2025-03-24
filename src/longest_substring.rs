/* Given a string s, find the length of the longest substring without duplicate characters.

Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.
Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.
*/

pub fn longest_substring(s: &str) -> i32 {
    let mut seen = [false; 256];
    let mut left = 0;
    let mut max_length = 0;
    let chars: Vec<char> = s.chars().collect();

    for right in 0..chars.len() {
        while seen[chars[right] as usize] {
            seen[chars[left] as usize] = false;
            left += 1;
        }

        seen[chars[right] as usize] = true;
        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
}
