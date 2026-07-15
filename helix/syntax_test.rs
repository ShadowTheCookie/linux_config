#Syntax highlighting text written by gpt


#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fmt::{self, Debug};

/// A sample trait with documentation markup.
pub trait Describe {
    fn describe(&self) -> String;
}

// Generic struct with lifetimes and traits
#[derive(Debug, Clone)]
pub struct Person<'a, T>
where
    T: Debug + Clone,
{
    pub name: &'a str,
    pub age: u32,
    pub data: T,
    active: bool,
}

// Enum variants
#[derive(Debug)]
enum Status {
    Ready,
    Working(u32),
    Failed { reason: String },
}

// Type alias
type ResultMap<K, V> = HashMap<K, V>;

impl<'a, T> Person<'a, T>
where
    T: Debug + Clone,
{
    pub fn new(name: &'a str, age: u32, data: T) -> Self {
        Self {
            name,
            age,
            data,
            active: true,
        }
    }

    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

impl<'a, T> Describe for Person<'a, T>
where
    T: Debug + Clone,
{
    fn describe(&self) -> String {
        format!(
            "{} is {} years old",
            self.name,
            self.age
        )
    }
}

// Macro example
macro_rules! announce {
    ($msg:expr) => {
        println!("Announcement: {}", $msg);
    };
}

// Async function
async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = format!("Fetching {}", url);
    Ok(response)
}

fn main() {
    // Constants
    const MAX_USERS: usize = 100;
    static VERSION: &str = "1.0.0";

    // Primitive types
    let integer: i32 = 42;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = '🦀';

    // Strings
    let message = "Hello, Rust!";
    let raw = r#"Raw string with "quotes""#;

    // Arrays and tuples
    let numbers = [1, 2, 3, 4, 5];
    let tuple: (&str, i32) = ("answer", 42);

    // Vector
    let values: Vec<i32> = vec![10, 20, 30];

    // HashMap
    let mut users: ResultMap<&str, u32> = HashMap::new();
    users.insert("Alice", 30);

    // Struct usage
    let person = Person::new("Alice", 30, vec![1, 2, 3]);

    println!("{}", person.greet());

    // Pattern matching
    let status = Status::Working(50);

    match status {
        Status::Ready => {
            println!("Ready");
        }
        Status::Working(progress) if progress > 0 => {
            println!("Progress: {}%", progress);
        }
        Status::Failed { reason } => {
            eprintln!("Error: {}", reason);
        }
        _ => {}
    }

    // Closures
    let multiply = |x: i32, y: i32| -> i32 {
        x * y
    };

    let result = multiply(4, 5);

    // Iterator chain
    let doubled: Vec<_> = values
        .iter()
        .map(|x| x * 2)
        .filter(|x| *x > 20)
        .collect();

    // Loop examples
    for item in doubled {
        println!("{}", item);
    }

    let mut counter = 0;

    while counter < 5 {
        counter += 1;
    }

    loop {
        break;
    }

    // Option and Result
    let maybe_value: Option<i32> = Some(100);

    if let Some(value) = maybe_value {
        println!("Value: {}", value);
    }

    let result: Result<i32, &str> = Ok(200);

    match result {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }

    // Unsafe block
    unsafe {
        let ptr: *const i32 = &integer;
        println!("{:?}", ptr);
    }

    announce!("Syntax test complete");
}
