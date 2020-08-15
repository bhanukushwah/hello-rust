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
    
    // Capacity in bytes
    println!("Capacity: {}", person.capacity());

    // isEmpty check
    println!("isEmpty: {}", person.is_empty());

    // Contains 
    println!("Contains 'Kushwah' {}", person.contains("Kushwah"));

    // Replace
    println!("Replace: {}", person.replace("Bhanu", "Shivam"));

    // Loop through string by whitespace
    for word in person.split_whitespace() {
        println!("{}", word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);


}