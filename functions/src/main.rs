fn main() {
    // Set the value of a variable to a function. Note, if the function just said "let x = 3", it would not work because it doesn't return the value.
    let y = {
        let x = 3;
        x + 1
    };

    // Functions with return values - usage
    let x = five();
    println!("The value of x is: {x}");
}

// Functions with Return Values
fn five() -> i32 {
    5
}