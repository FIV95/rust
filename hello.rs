//! A simple Hello World program that demonstrates various Rust features
//! to test syntax highlighting and LSP capabilities.

/// A custom error type for demonstration purposes
#[derive(Debug)]
enum MyError {
    InvalidValue,
    OutOfRange,
}

/// A simple struct to demonstrate struct syntax
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u32,
}

/// A trait to demonstrate trait syntax
trait Greet {
    fn greet(&self) -> String;

    fn greet_loudly(&self) -> String {
        format!("{}!!!", self.greet().to_uppercase())
    }
}

// Implement the Greet trait for Person
impl<'a> Greet for Person<'a> {
    fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old", self.name, self.age)
    }
}

// Generic function with a trait bound
fn print_greeting<T: Greet>(entity: &T) {
    println!("{}", entity.greet());
}

// Function that returns a Result
fn safe_division(dividend: f64, divisor: f64) -> Result<f64, MyError> {
    if divisor == 0.0 {
        Err(MyError::InvalidValue)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() -> Result<(), MyError> {
    // Basic println with string formatting
    println!("Hello, World!");

    // Using a struct with a lifetime
    let person = Person { name: "Rustacean", age: 5 };
    println!("Person: {:?}", person);

    // Using the trait implementation
    print_greeting(&person);
    println!("Loud greeting: {}", person.greet_loudly());

    // Using pattern matching with an Option
    let maybe_value: Option<i32> = Some(42);
    match maybe_value {
        Some(value) => println!("The value is: {}", value),
        None => println!("No value found"),
    }

    // Using if let for concise pattern matching
    if let Some(value) = maybe_value {
        println!("Found value: {}", value);
    }

    // Using a closure
    let double = |x: i32| x * 2;
    println!("Double of 5 is: {}", double(5));

    // Using iterators and closures
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().map(|&n| n * 2).sum();
    println!("Sum of doubled numbers: {}", sum);

    // Error handling with Result
    match safe_division(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    // Using the ? operator for error propagation
    let result = safe_division(10.0, 2.0)?;
    println!("Result with ? operator: {}", result);

    // Using a block expression
    let value = {
        let x = 5;
        let y = 10;
        x + y  // Note: no semicolon means this is the return value
    };
    println!("Block expression result: {}", value);

    println!("All Rust features demonstrated successfully!");
    Ok(())
}

