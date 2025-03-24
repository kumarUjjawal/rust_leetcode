mod jump_game;
mod longest_substring;
fn main() {
    let mut arr = vec![3, 2, 1, 0, 4];
    let result = jump_game::jump_game(&mut arr);
    println!("Result of [2,3,1,1,4] is {}", result);

    let s = "abcabdksf";
    let longest = longest_substring::longest_substring(s);
    println!(
        "Longest substring without repeating characters: {}",
        longest
    );
}
