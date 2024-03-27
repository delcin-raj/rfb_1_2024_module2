#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused)]

#[derive(Debug)]
enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        todo!()
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        todo!()
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        todo!()
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        todo!()
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        todo!()
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        todo!()
    }
}

// Add unit tests below