#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused)]
use std::ops::Add;
/// This is an append at head Singly linked list
/// Which means if 1, 2 and 3 are inserted in that order the result will be
/// similar to [3, 2, 1]
#[derive(Debug, PartialEq, Eq)]
pub enum List<T> {
    Empty,
    Node {
        data: T,
        next: Box<List<T>>
    }
}

// [1,2,3,4]

// The List we have defined above is a recursive data structure, try to use recursion
// to implement the methods below

impl <T> List<T> {
    pub fn empty() -> List<T> {
        List::Empty
    }
    pub fn singleton(value: T) -> List<T> {
        List::Node { data: value, next: Box::new(List::Empty) }
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn prepend(self, value: T) -> List<T> {
        todo!()
    }

    // Create List of items of type T from an Iterator of type T
    // Make sure that the order of the incoming iterator are preserved
    // HINT: DoubleEndedIterator have a method called rev()
    pub fn from_iterator(iter: impl DoubleEndedIterator<Item = T>) -> List<T> {
        todo!()
    }

    /// Removes the first occurrence of a value from the list
    /// If that element do not exists return the list as it is
    // Of course we need to make sure that this method is valid only for lists
    // which holds types that can be checked for equality
    pub fn remove(self, value: T) -> List<T> {
        todo!()
    }


    pub fn iter<'a>(&'a self) -> ListIterator<'a, T> {
        ListIterator { current: self }
    }
    // Implement utility methods that you see fit
}

// Why lifetime parameter here needed is because the current has to point
// to node of the List and it should be movable
pub struct ListIterator<'a, T> {
    current: &'a List<T>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
         match self.current {
            List::Empty => None,
            List::Node { data, next } => {
                self.current = next;
                Some(data)
            },
        }
    }
}


// Bonus
// Operator overloading is just Traits in rust

impl<T> Add for List<T> {
    type Output = List<T>;

    // concatenate two lists
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// Implement tests for List below
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list: List<i32> = List::empty();
        assert_eq!(list, List::Empty);
    }

    #[test]
    fn test_singleton_list() {
        let list = List::singleton(1);
        assert_eq!(list, List::Node { data: 1, next: Box::new(List::Empty) });
    }

    #[test]
    fn test_prepend() {
        let list = List::singleton(1).prepend(2).prepend(3);
        assert_eq!(list, List::Node { data: 3, next: Box::new(List::Node { data: 2, next: Box::new(List::Node { data: 1, next: Box::new(List::Empty) }) }) });
    }

    #[test]
    fn test_from_iterator() {
        let list = List::from_iterator(vec![1, 2, 3].into_iter());
        let expected = List::Node{data: 1, next: Box::new(List::Node{data: 2, next: Box::new(List::Node{data: 3, next: Box::new(List::Empty)})})};
        assert_eq!(list, expected);
    }

    #[test]
    fn test_remove() {
        let list = List::from_iterator(vec![1, 2, 3].into_iter());
        let expected = List::Node{data: 2, next: Box::new(List::Node{data: 3, next: Box::new(List::Empty)})};
        assert_eq!(list.remove(1), expected);
    }

    #[test]
    fn test_iter() {
        let list = List::from_iterator(vec![1, 2, 3].into_iter());
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_list_add() {
        let list1 = List::from_iterator(vec![1, 2, 3].into_iter());
        let list2 = List::from_iterator(vec![4, 5, 6].into_iter());
        let expected = List::from_iterator(vec![1, 2, 3, 4, 5, 6].into_iter());
        assert_eq!(list1 + list2, expected);
    }

    #[test]
    fn test_list_len() {
        let list = List::from_iterator(vec![1, 2, 3].into_iter());
        assert_eq!(list.len(), 3);
    }
}
