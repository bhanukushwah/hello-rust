// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {

    // this variable is immutable so you can't reassign this variable.
    let name = "Bhanu Kushwah";

    // To declare a mutable variable we have to bind with mut
    let mut age = 21;
    println!("I am {} and I am {}", name, age);

    age = 22;

    println!("I am {} and I am {}", name, age);

    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign Multiple variables
    let ( my_name, my_age ) = ("Bhanu", 21);
    println!("I am {} and I am {}", my_name, my_age);

}