/*
Given an input string s, reverse the order of the words.

A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

Return a string of the words in reverse order concatenated by a single space.

Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.



Example 1:

Input: s = "the sky is blue"
Output: "blue is sky the"
*/
pub fn reverse_words_string_manual(s: &str) -> String {
    let mut stack: Vec<String> = Vec::new();
    let mut current_word = String::new();

    for char in s.chars() {
        if char.is_whitespace() {
            if !current_word.is_empty() {
                stack.push(current_word);
                current_word = String::new();
            }
        } else {
            current_word.push(char);
        }
    }

    if !current_word.is_empty() {
        stack.push(current_word);
    }
}
