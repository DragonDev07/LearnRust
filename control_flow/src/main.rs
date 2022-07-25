fn main() {
    // if expressions
    let mut number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }
    // You can also use if statements to set a variable, in 1 line
    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // Reptitions with Loops
    // To break out of a loop, use the "break" keyword.
    // To skip the rest of the loop and go to the next iteration, use the "continue" keyword.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // If you have nested loops, the break will break out of the innermost loop. Same as continue.
    // You can use loop labels to specify which loop to break out of.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loops
    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    // For loops
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}