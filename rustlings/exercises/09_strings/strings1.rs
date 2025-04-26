// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    // This creates a new mutable string object from a string literal.
    String::from("blue")
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
