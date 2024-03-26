#[derive(Debug)]
pub enum List<T> {
    Empty,
    Node {
        data: T,
        next: Box<List<T>>
    }
}

impl <T> List<T> {
    pub fn empty() -> List<T> {
        List::Empty
    }
    pub fn singleton(value: T) -> List<T> {
        List::Node { data: value, next: Box::new(List::Empty) }
    }
    // Implement utility methods that you see fit

    pub fn push(self, value: T) -> List<T> {
        List::Node { data: value, next: Box::new(self) }
    }

    // Create List of items of type T from an Iterator of type T
    pub fn from_iterator(iter: impl Iterator<Item = T>) -> List<T> {
        todo!()
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    // fn next(&mut self) -> Option<T> even this is valid
    // but the below signature is the idiomatic way
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}