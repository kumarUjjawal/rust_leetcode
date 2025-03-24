mod jump_game;
fn main() {
    let mut arr = vec![3, 2, 1, 0, 4];
    let result = jump_game::jump_game(&mut arr);
    println!("Result of [2,3,1,1,4] is {}", result);
}
