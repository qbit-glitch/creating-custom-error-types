use core::num;
use std::io;
use std::fmt;
use std::error::Error;
fn main() {
    match divide(8.0, 0.0){
        Ok(result) => println!("Division Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Handling possible errors for might_return_error
    match might_return_error(){
        Ok(value) => println!("Got value: {}", value),
        Err(e) => match e{
            MyError::Type1 => println!("Error of Type 1 occured"),
            MyError:: Type2 => println!("Error of Type 2 occured"),
        },
    }

    // Handling Rocket Errors
    handle_error(RocketError::OutOfFuel);
    handle_error(RocketError::AlienInvasion);

    // Handling custom I/O error
    match read_file("non_existent_file.txt"){
        Ok(contents) => println!("File Contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }


    //Example of borrowing a book
    let mut library = Library::new(); //Assuming Library is defined elsewhere
     match borrow_book(1, &mut library){
        Ok(_) => println!("Book borrowed successfully."),
        Err(e) => println!("Error borrowing book: {}", e),
     }
    
}

//Improving error clarity
fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str>{
    if denominator == 0.0{
        Err("You can't divide by zero, silly!")
    }else { 
        Ok(numerator / denominator)
    }
}

//Creating Specific Responses
enum RocketError{
    OutOfFuel,
    NavigationSystemFailure,
    AlienInvasion,
}

// Code to handle each of the above errors
fn handle_error(error: RocketError){
    match error{
        RocketError::AlienInvasion => {
            println!("We are invaded by aliens");
        },
        RocketError::NavigationSystemFailure => {
            println!("Houston, We have lost our path, Navigation System Failure");
        },
        RocketError::OutOfFuel => {
            println!("Houston, We are out of Fuel");
        }
    }
}


// Creating a Custom Error Type
#[derive(Debug)]
enum MyCustomError{
    Io(io::Error),
    Parse(num::ParseIntError),
    Other(String),
}

impl fmt::Display for MyCustomError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            MyCustomError::Io(err) => write!(f, "I/O error : {}", err),
            MyCustomError::Parse(err) => write!(f, "I/O error : {}", err),
            MyCustomError::Other(message) => write!(f,"Other Error: {}", message),
        }
    }
}

impl std::error::Error for MyCustomError{}

fn read_file(filename: &str) -> Result<String, MyCustomError>{
    // Simulate an I/O error
    if filename == "non_existent_file.txt"{
        return Err(MyCustomError::Io(io::Error::new(io::ErrorKind::NotFound, "File not Found")));
    }
    Ok(String::from("File Content"))
}


// Returning Custom Errors
#[derive(Debug)]
pub struct BookError{
    pub message: String,
}

impl fmt::Display for BookError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BookError{}

pub struct Library{
    // Define your library structure here
}

impl Library{
    pub fn new() -> Self{
        // Initialize your library
        Library{}
    }

    pub fn is_book_borrowed(&self, _book_id: i32) -> bool {
        // Dummy Implementation 
        false
    }

    pub fn borrow_book(&mut self, _book_id: i32){
        // Dummy implementation
        println!("Book borrowed");
    }
}


fn borrow_book(book_id: i32, library: &mut Library) -> Result<(), BookError>{
    if library.is_book_borrowed(book_id){
        Err(BookError{
            message: String::from("Sorry, this book is already borrowed!"),
        })
    }
    else {
        library.borrow_book(book_id);
        Ok(())
    }
    
}


// Defining enum MyError
enum MyError{
    Type1,
    Type2,
}

// Dummy implementation for might_return_error
fn might_return_error() -> Result<i32, MyError> {
    // Simulating a Type2 Error
    Err(MyError::Type2)
}
