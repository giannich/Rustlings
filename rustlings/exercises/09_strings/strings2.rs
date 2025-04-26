// TODO: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    // In this case, the expected type is a string literal (&str) allocated in the stack,
    // but we're passing a mutable String object, which is allocated in the heap.
    // By using the & operator, we can dereference the string literal from the String object.
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
