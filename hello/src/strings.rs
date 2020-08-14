// Primitive str = Immutable fixd length string somewhere in memory
// String = Growable, heap allocated data structure - use when you need to modify or own string data

pub fn run(){
    
    // Primitive
    let  hello = "Hello";
    println!("{} there", hello);

    // String
    let mut person = String::from("Bhanu ");
    println!("Hello {}", person);

    // METHODS
     
    // Get length of string
    println!("length of hello: {}", hello.len());

    // push a character to a string
    person.push('K');

    // push a string
    person.push_str("ushwah");

    println!("Hello {}", person);

}