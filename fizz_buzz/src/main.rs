// Print "fizz" if a number is divisible by 3
// Print "buzz" if a number is divisible by 5
// Print "FizzBuzz" if a number is divisible by both 3 and 5

fn main() {
    let mut num = 0;
    let amount = 15;

    while num != amount + 1{
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{num}");
        }

        num += 1;
    }
}
