// Tuples group together the values of different types
// Max 12 element

pub fn run(){
    let person: (&str, &str, i8) = ("Bhanu", "Kushwah", 21);

    println!("{} {} is {} Now.", person.0, person.1, person.2)
}