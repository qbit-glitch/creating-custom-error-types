# Rust Error Handling Example

This project demonstrates various error handling techniques in Rust, including custom error types, error enums, and handling specific error cases.

## Features

- **Division Function**: Safely divides two numbers with error handling for division by zero.
- **Rocket Errors**: Handles custom errors related to rocket operations (e.g., out of fuel, navigation failure).
- **File Reading Simulation**: Simulates reading a file and handles potential I/O errors.
- **Library System**: Demonstrates borrowing books with error handling for already borrowed books.

## Usage

To run the project, ensure you have Rust installed. You can run the program using the following command:

```bash
cargo run
```

## Code Overview

### Error Handling

- **Division Function**: The `divide` function returns an error message when attempting to divide by zero.

    ```rust
    fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
        if denominator == 0.0 {
            Err("You can't divide by zero, silly!")
        } else {
            Ok(numerator / denominator)
        }
    }
    ```

- **RocketError Enum**: Represents various errors that can occur during rocket operations.

    ```rust
    enum RocketError {
        OutOfFuel,
        NavigationSystemFailure,
        AlienInvasion,
    }
    ```

- **Custom Error Types**: 
    - `MyCustomError`: A custom error type for I/O errors.
    - `BookError`: Represents errors that can occur while borrowing books.

### Library System

The library system allows users to check if a book is borrowed and to borrow a book if it is available.

```rust
pub struct Library {
    // Define your library structure here
}

impl Library {
    pub fn new() -> Self {
        Library {}
    }

    pub fn is_book_borrowed(&self, _book_id: i32) -> bool {
        false // Dummy implementation
    }

    pub fn borrow_book(&mut self, _book_id: i32) {
        println!("Book borrowed");
    }
}

fn borrow_book(book_id: i32, library: &mut Library) -> Result<(), BookError> {
    if library.is_book_borrowed(book_id) {
        Err(BookError { message: String::from("Sorry, this book is already borrowed!") })
    } else {
        library.borrow_book(book_id);
        Ok(())
    }
}
```

## Error Handling Example

The `main` function showcases the error handling in action, including division, file reading, and borrowing a book.

```rust
fn main() {
    // Example usages
    // ...
}
```

## Conclusion

This project provides a clear illustration of error handling in Rust, showcasing how to create and manage custom error types, handle various error conditions, and structure a simple library system.

Feel free to modify and expand upon this project as needed!

### Notes:
- This `README.md` provides a clear overview of the project's purpose, features, and usage instructions.
- It includes code snippets to illustrate key functionalities.
- You can adjust the content or formatting as needed to better fit your project's style!
