// To define a struct, type keyword "struct" followed by the name of the struct
// Then, define names of data, along with what types of data those items are.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// You can also use tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
struct AlwaysEqual;


fn main() {
    // Define a variable using a struct
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("username1243"),
        active: true,
        sign_in_count: 10,
    };

    // Get an element of a variable (user1)
    let user_email = &user1.email;
    println!("Email: {user_email}");

    // Change an element of a variable if it is mutable
    user1.email = String::from("user2@example.com");
    println!("Email: {}", &user1.email);

    // Call the build_user function
    let another_user = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // You can create users from other users using the Struct Update Syntax
    // This way ypu can do what is written below instead of having to define every variable individually with "user1.email" etc.
    // However, this way, you can no longer use "user1" because the values were moved since they were strings
    // If you were to only use the user1 data for "sign_in_count" and "active", you could still use the User1 variable because the Strings were not moved, and the other data was copied. instead of moved.
    let user2 = User {
        email: String::from("someemail@something.com"),
        ..user1
    };

    // Use the tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Use the Unit-Like Struct
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // Since the name of the variable "email" and the name of the struct feild are the same, we dont have to say "email: email,". This is called the Feild Init Shorthand syntax
        username,
        active: true,
        sign_in_count: 1,
    }
}