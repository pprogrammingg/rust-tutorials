// Example demonstrating Rust traits, trait bounds, and trait objects

// Define a simple trait
trait Greet {
    fn greet(&self);
}

// Implement the trait for a specific type
struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

// Function that uses the trait
pub fn run_traits_example() {
    // Create an instance of Person
    let person = Person {
        name: String::from("John"),
    };

    // Greet the person using the trait
    person.greet();
}
