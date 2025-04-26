// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    // See: https://doc.rust-lang.org/std/primitive.str.html
    string_slice("blue");

    // See: https://doc.rust-lang.org/std/primitive.str.html#method.to_string
    string("red".to_string());

    // See: https://doc.rust-lang.org/std/string/struct.String.html#impl-From%3C%26str%3E-for-String
    // In this case, `&str` is converted to `String` through the `From` trait.
    // See: https://doc.rust-lang.org/rust-by-example/conversion/from_into.html#from
    string(String::from("hi"));

    // See: https://doc.rust-lang.org/std/primitive.str.html#method.to_owned
    string("rust is fun!".to_owned());

    // See: https://doc.rust-lang.org/std/primitive.str.html#method.into
    // In this case, `&str` is converted to `String` through the `Into` trait.
    // Note that in this case, the compiler will automatically determine the type 
    // to convert into. You can use either 'string' or 'string_slice' here as 
    // it can convert into its own type as well, but the complier will complain.
    // See: https://doc.rust-lang.org/rust-by-example/conversion/from_into.html#into
    string("nice weather".into());

    // See: https://doc.rust-lang.org/std/macro.format.html
    // From the documentation, `format!` returns a `String`.
    // Note that the first argument is a format string, which can contain the `{}` placeholder.
    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    // See: https://doc.rust-lang.org/std/primitive.str.html#method.trim
    // This is because the old string literal is not modified, but a new one is created.
    string_slice("  hello there ".trim());

    // See: https://doc.rust-lang.org/std/primitive.str.html#method.replace
    // This is because the old string literal is not modified, but a new one is created.
    string("Happy Monday!".replace("Mon", "Tues"));

    // See: https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase
    // This is because the old string literal is not modified, but a new one is created.
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
