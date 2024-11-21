use core::{ops::Index, ptr};

use alloc::{
    boxed::Box,
    fmt,
    string::{String, ToString},
    vec::Vec,
};

mod iter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    value: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    const fn new(value: T) -> Self {
        Self {
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

/// A doubly-linked list implementation with reference-counted nodes.
///
/// This list allows adding and removing elements efficiently at both ends.
///
/// # Type Parameters
///
/// - `T`: The type of elements stored in the list.
///
/// # Examples
///
/// ```
/// use libx::collections::list::doubly_linked::List;
///
/// // Create a new list
/// let mut list: List<u32> = List::new();
///
/// // Check if the list is empty
/// assert!(list.is_empty());
///
/// // Push elements to the front of the list
/// list.push_front(1);
/// list.push_front(2);
///
/// // Push elements to the back of the list
/// list.push_back(3);
/// list.push_back(4);
///
/// // Pop elements from the front of the list
/// assert_eq!(list.pop_front(), Some(2));
/// assert_eq!(list.pop_front(), Some(1));
///
/// // Pop elements from the back of the list
/// assert_eq!(list.pop_back(), Some(4));
/// assert_eq!(list.pop_back(), Some(3));
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
    length: usize,
    capacity: usize,
}

impl<T> List<T> {
    /// Creates a new empty list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let list: List<u32> = List::new();
    /// assert!(list.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
            capacity: 0,
        }
    }

    /// Creates a new empty list with the specified capacity.
    ///
    /// The `capacity` parameter determines the maximum number of elements
    /// the list can hold.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let list: List<u32> = List::with_capacity(10);
    /// assert_eq!(list.capacity(), 10);
    /// ```
    #[must_use]
    pub const fn with_capacity(capacity: usize) -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
            capacity,
        }
    }

    /// Pushes an element to the front of the list.
    ///
    /// If the length of the list exceeds its capacity, the list will be resized.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the front of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// assert_eq!(list.pop_front(), Some(2));
    /// ```
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::into_raw(Box::new(Node::new(value)));

        if self.length >= self.capacity {
            // Perform resizing or handle capacity overflow error
            // For simplicity, let's double the capacity if it's reached
            self.capacity *= 2;
        }

        if let Some(old_head) = self.head.take() {
            unsafe {
                (*old_head).prev = new_node;

                (*new_node).next = old_head;
            }
            self.head = Some(new_node);
        } else {
            self.head = Some(new_node);
            self.tail = Some(new_node);
        }

        self.length += 1;
    }

    /// Pushes an element to the back of the list.
    ///
    /// If the length of the list exceeds its capacity, the list will be resized.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the back of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// ```
    pub fn push_back(&mut self, value: T) {
        let new_node = Box::into_raw(Box::new(Node::new(value)));

        if self.capacity == 0 {
            self.capacity = 4;
        }

        if self.length >= self.capacity {
            // Perform resizing or handle capacity overflow error
            // For simplicity, let's double the capacity if it's reached
            self.capacity *= 2;
        }

        if let Some(old_tail) = self.tail.take() {
            unsafe {
                (*old_tail).next = new_node;
                (*new_node).prev = old_tail;
            }
        } else {
            self.head = Some(new_node);
        }

        self.tail = Some(new_node);

        self.length += 1;
    }

    /// Pushes an element to the front of the list if the capacity is not reached.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the front of the list.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the maximum capacity is reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::with_capacity(2);
    /// assert_eq!(list.push_front_within_capacity(1), Ok(()));
    /// assert_eq!(list.push_front_within_capacity(2), Ok(()));
    /// assert_eq!(list.push_front_within_capacity(3), Err("Maximum capacity reached".to_string()));
    /// ```
    pub fn push_front_within_capacity(&mut self, value: T) -> Result<(), String> {
        if self.length >= self.capacity {
            return Err("Maximum capacity reached".to_string());
        }

        let new_node = Box::into_raw(Box::new(Node::new(value)));

        if let Some(old_head) = self.head.take() {
            unsafe {
                (*old_head).prev = new_node;
                (*new_node).next = old_head;
            }

            self.head = Some(new_node);
        } else {
            self.head = Some(new_node);
            self.tail = Some(new_node);
        }

        self.length += 1;
        Ok(())
    }

    /// Pushes an element to the back of the list if the capacity is not reached.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the back of the list.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the maximum capacity is reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::with_capacity(2);
    /// assert_eq!(list.push_back_within_capacity(1), Ok(()));
    /// assert_eq!(list.push_back_within_capacity(2), Ok(()));
    /// assert_eq!(list.push_back_within_capacity(3), Err("Maximum capacity reached".to_string()));
    /// ```
    pub fn push_back_within_capacity(&mut self, value: T) -> Result<(), String> {
        if self.length >= self.capacity {
            return Err("Maximum capacity reached".to_string());
        }

        let new_node = Box::into_raw(Box::new(Node::new(value)));

        if let Some(old_tail) = self.tail.take() {
            unsafe {
                (*old_tail).next = new_node;
                (*new_node).prev = old_tail;
            }
        } else {
            self.head = Some(new_node);
        }

        self.tail = Some(new_node);

        self.length += 1;
        Ok(())
    }

    /// Removes and returns the element from the front of the list.
    ///
    /// # Returns
    ///
    /// Returns `Some(element)` if the list is not empty.
    /// Returns `None` if the list is empty.
    ///
    /// # Panics
    ///
    /// Panics if the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(old_head) => {
                if unsafe { !(*old_head).next.is_null() } {
                    unsafe {
                        (*(*old_head).next).prev = ptr::null_mut();
                        self.head = Some((*old_head).next);
                    }
                } else {
                    self.tail = None;
                }

                self.length -= 1;
                unsafe { Some(Box::from_raw(old_head).value) }
            }
            None => None,
        }
    }

    /// Removes and returns the element from the back of the list.
    ///
    /// # Returns
    ///
    /// Returns `Some(element)` if the list is not empty.
    /// Returns `None` if the list is empty.
    ///
    /// # Panics
    ///
    /// Panics if the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(old_tail) => {
                if unsafe { !(*old_tail).prev.is_null() } {
                    unsafe {
                        (*(*old_tail).prev).next = ptr::null_mut();
                        self.tail = Some((*old_tail).prev);
                    }
                } else {
                    self.head = None;
                }

                self.length -= 1;
                unsafe { Some(Box::from_raw(old_tail).value) }
            }
            None => None,
        }
    }

    /// Returns the number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }

    /// Checks if the list is empty.
    ///
    /// # Returns
    ///
    /// Returns `true` if the list is empty, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::new();
    /// assert_eq!(list.is_empty(), true);
    ///
    /// list.push_back(1);
    /// assert_eq!(list.is_empty(), false);
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the capacity of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let list: List<u32> = List::with_capacity(10);
    /// assert_eq!(list.capacity(), 10);
    /// ```
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Sets the capacity of the list.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The new capacity for the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::<i32>::new();
    /// list.set_capacity(20);
    /// assert_eq!(list.capacity(), 20);
    /// ```
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }

    /// Returns the value of the element at the front of the list, without removing it.
    ///
    /// This method returns `Some(value)` if the list is not empty, where `value` is a clone
    /// of the value stored in the front element. If the list is empty, `None` is returned.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type of elements stored in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::new();
    /// list.push_front(1);
    /// list.push_front(2);
    ///
    /// assert_eq!(list.front(), Some(2));
    /// assert_eq!(list.len(), 2);
    /// ```
    #[must_use]
    pub fn front(&self) -> Option<T>
    where
        T: Clone,
    {
        self.head.map(|head| unsafe { (*head).value.clone() })
    }

    /// Returns the value of the element at the back of the list, without removing it.
    ///
    /// This method returns `Some(value)` if the list is not empty, where `value` is a clone
    /// of the value stored in the back element. If the list is empty, `None` is returned.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type of elements stored in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    ///
    /// assert_eq!(list.back(), Some(2));
    /// assert_eq!(list.len(), 2);
    /// ```
    #[must_use]
    pub fn back(&self) -> Option<T>
    where
        T: Clone,
    {
        self.tail.map(|tail| unsafe { (*tail).value.clone() })
    }

    /// Removes all elements from the list.
    ///
    /// This method removes all elements from the list by repeatedly calling the `pop_back` method
    /// until the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list: List<i32> = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// list.clear();
    ///
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        while !self.is_empty() {
            self.pop_back();
        }
    }

    /// Inserts an element at the specified index in the list.
    ///
    /// The element is inserted at the specified index in the list, shifting all elements after it
    /// to the right. If the index is equal to the length of the list, the element is inserted at
    /// the end of the list.
    ///
    /// # Panics
    ///
    /// This method panics if the index is out of bounds. An index is considered out of bounds if it
    /// is greater than the length of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list: List<i32> = List::new();
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// list.insert(1, 4);
    ///
    /// assert_eq!(list.len(), 4);
    /// assert_eq!(list[1], 4);
    /// assert_eq!(list[2], 2);
    /// ```
    pub fn insert(&mut self, index: usize, value: T) {
        // Check if the index is within bounds
        assert!(index <= self.len(), "index out of bounds");

        if index == 0 {
            self.push_front(value);
        } else if index == self.length {
            self.push_back(value);
        } else {
            let new_node = Box::into_raw(Box::new(Node::new(value)));

            let mut current_index = 0;
            let mut current_node = self.head.expect("head is None");

            while current_index < index {
                unsafe {
                    current_node = (*current_node).next;
                    current_index += 1;
                }
            }

            unsafe {
                let prev_node = (*current_node).prev;

                (*new_node).prev = prev_node;
                (*new_node).next = current_node;

                (*prev_node).next = new_node;
                (*current_node).prev = new_node;
            }

            self.length += 1;
        }
    }

    /// Inserts elements from an iterator at the specified index in the list.
    ///
    /// The elements from the iterator are inserted at the specified index in the list, shifting all
    /// elements after it to the right. If the index is equal to the length of the list, the elements
    /// are inserted at the end of the list.
    ///
    /// # Panics
    ///
    /// This method panics if the index is out of bounds. An index is considered out of bounds if it
    /// is greater than the length of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list: List<i32> = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// let iterable = vec![4, 5, 6];
    /// list.insert_from_iterator(1, iterable);
    ///
    /// assert_eq!(list.len(), 6);
    /// assert_eq!(list[1], 4);
    /// assert_eq!(list[2], 5);
    /// assert_eq!(list[3], 6);
    /// ```
    pub fn insert_from_iterator<I>(&mut self, index: usize, iter: I)
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: DoubleEndedIterator,
        T: Clone,
    {
        // Check if the index is within bounds
        assert!(index <= self.len(), "index out of bounds");

        if index == 0 {
            for value in iter {
                self.push_front(value);
            }
        } else if index == self.length {
            for value in iter.into_iter().rev() {
                self.push_back(value);
            }
        } else {
            let mut current_index = 0;
            let mut current_node = self.head.expect("head is None");

            while current_index < index {
                unsafe {
                    current_node = (*current_node).next;
                    current_index += 1;
                }
            }

            for value in iter.into_iter().rev() {
                let new_node = Box::into_raw(Box::new(Node::new(value)));

                unsafe {
                    let prev_node = (*current_node).prev;

                    (*new_node).prev = prev_node;
                    (*new_node).next = current_node;

                    (*prev_node).next = new_node;
                    (*current_node).prev = new_node;
                }

                current_node = new_node;
                self.length += 1;
            }
        }
    }

    /// Removes the node at the given index from the list.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node to remove.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// list.remove_by_index(1);
    ///
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(3));
    /// ```
    pub fn remove_by_index(&mut self, index: usize) -> Option<T> {
        assert!(index < self.length, "Index out of bounds");

        let mut current_node = self.head;
        let mut current_index = 0;

        while let Some(node) = current_node {
            let next_node = unsafe { (*node).next };

            if current_index == index {
                let removed_element = self.unlink_node(node);
                return Some(removed_element);
            }

            current_node = Some(next_node);
            current_index += 1;
        }

        None
    }

    /// Removes elements from the list within the specified range and returns them as a slice.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of indices to remove from the list.
    ///
    /// # Returns
    ///
    /// Returns a slice containing the removed elements.
    ///
    /// # Panics
    ///
    /// This function panics if the range is out of bounds.
    pub fn removed_by_range<'a>(&mut self, range: core::ops::Range<usize>) -> &'a [T]
    where
        T: Clone,
    {
        let start = range.start;
        let end = range.end;

        // Check if the range is within bounds
        assert!(start <= end && end <= self.len(), "range out of bounds");

        // Find the starting node to remove
        let mut current_index = 0;
        let mut current_node = self.head;

        while current_index < start {
            if let Some(node) = current_node {
                unsafe {
                    current_node = Some((*node).next);
                }
                current_index += 1;
            } else {
                break;
            }
        }

        // Remove the elements within the range
        let mut removed_elements = Vec::new();

        while current_index < end {
            if let Some(node) = current_node {
                let next_node;

                unsafe {
                    next_node = (*node).next;
                    let elements = self.unlink_node(node);
                    removed_elements.push(elements);
                }

                current_node = Some(next_node);
                current_index += 1;
            } else {
                break;
            }
        }

        // Update the head and tail pointers
        if start == 0 {
            self.head = current_node;
        }
        if end == self.length {
            self.tail = current_node;
        }

        removed_elements.leak()
    }

    /// Returns an iterator over the elements of the list.
    ///
    /// The iterator visits the elements of the list in the order they appear, starting from the front
    /// and moving towards the back. The iterator element type is a reference to the elements of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use libx::collections::list::doubly_linked::List;
    ///
    /// let mut list: List<i32> = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// let mut iterator = list.iter();
    ///
    /// assert_eq!(iterator.next(), Some(1));
    /// assert_eq!(iterator.next(), Some(2));
    /// assert_eq!(iterator.next(), Some(3));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[must_use]
    pub const fn iter(&self) -> iter::Iter<'_, T>
    where
        T: Clone,
    {
        iter::Iter {
            stack: self,
            index: 0,
        }
    }

    fn unlink_node(&mut self, node: *mut Node<T>) -> T {
        let prev_node = unsafe { (*node).prev };
        let next_node = unsafe { (*node).next };

        if prev_node.is_null() {
            self.head = Some(next_node);
        } else {
            unsafe {
                (*prev_node).next = next_node;
            }
        }

        if next_node.is_null() {
            self.tail = Some(prev_node);
        } else {
            unsafe {
                (*next_node).prev = prev_node;
            }
        }

        self.length -= 1;

        unsafe { Box::from_raw(node).value }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> Default for List<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        // Check if the index is within bounds
        assert!(index < self.length, "Index out of bounds");

        // Traverse the list to find the node at the specified index
        let mut current = self.head.expect("head is None");
        for _ in 0..index {
            unsafe {
                let next = (*current).next;

                current = next;
            }
        }

        // Return a reference to the value inside the node
        unsafe { &(*current).value }
    }
}

impl<'a, T> IntoIterator for &'a List<T>
where
    T: Clone,
{
    type Item = T;

    type IntoIter = iter::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

unsafe impl<T> Send for List<T> {}
unsafe impl<T> Sync for List<T> {}

impl<T> fmt::Debug for List<T>
where
    T: fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

pub macro list {
    () => {
        $crate::list::doubly_linked::List::new()
    },

    ($($x:expr),*) => {
        {
            let mut temp_list = $crate::collections::list::doubly_linked::List::new();
            $(
                temp_list.push_back($x);
            )*
            temp_list
        }
    }
}

#[cfg(test)]
mod tests {

    use alloc::vec;

    use super::*;

    #[test]
    fn test_new_list_is_empty() {
        let list: List<u32> = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_with_capacity() {
        let list: List<u32> = List::with_capacity(10);
        assert_eq!(list.capacity(), 10);
    }

    #[test]
    fn test_push_front() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
    }

    #[test]
    fn test_push_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
    }

    #[test]
    fn test_push_front_within_capacity() {
        let mut list = List::with_capacity(2);
        assert_eq!(list.push_front_within_capacity(1), Ok(()));
        assert_eq!(list.push_front_within_capacity(2), Ok(()));
        assert_eq!(
            list.push_front_within_capacity(3),
            Err("Maximum capacity reached".to_string())
        );
    }

    #[test]
    fn test_push_back_within_capacity() {
        let mut list = List::with_capacity(2);
        assert_eq!(list.push_back_within_capacity(1), Ok(()));
        assert_eq!(list.push_back_within_capacity(2), Ok(()));
        assert_eq!(
            list.push_back_within_capacity(3),
            Err("Maximum capacity reached".to_string())
        );
    }

    #[test]
    fn test_pop_front() {
        let mut list = list![1, 2];
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_pop_back() {
        let mut list = list![1, 2];
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_len() {
        let list = list![1, 2];
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut list = List::new();
        assert!(list.is_empty());

        list.push_back(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_capacity() {
        let list: List<u32> = List::with_capacity(10);
        assert_eq!(list.capacity(), 10);
    }

    #[test]
    fn test_set_capacity() {
        let mut list = List::<i32>::new();
        list.set_capacity(20);
        assert_eq!(list.capacity(), 20);
    }

    #[test]
    fn test_front() {
        let list = list![1, 2];
        assert_eq!(list.front(), Some(1));
    }

    #[test]
    fn test_back() {
        let list = list![1, 2];
        assert_eq!(list.back(), Some(2));
    }

    #[test]
    fn test_clear() {
        let mut list = list![1, 2];
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut list = list![1, 2];
        list.insert(1, 3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
    }

    #[test]
    fn test_insert_from_iterator() {
        let mut list = list![1, 2];
        list.insert_from_iterator(1, vec![3, 4, 5]);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(2));
    }

    #[test]
    #[should_panic = "Index out of bounds"]
    fn test_remove_by_index_empty_list() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.len(), 0);

        list.remove_by_index(0);
    }

    #[test]
    fn test_remove_by_index_single_item_list() {
        let mut list = List::new();
        list.push_back(1);
        assert_eq!(list.len(), 1);

        // Removing the single item should result in an empty list
        list.remove_by_index(0);
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_by_index_front() {
        let mut list = list![1, 2, 3];

        list.remove_by_index(0);

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn test_remove_by_index_middle() {
        let mut list = list![1, 2, 3];

        list.remove_by_index(1);

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn test_remove_by_index_back() {
        let mut list = list![1, 2, 3];

        list.remove_by_index(2);

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_removed_by_range() {
        let mut list: List<i32> = list![10, 20, 30, 40, 50];

        // Remove elements by range
        let range = list.removed_by_range(1..4); // Remove elements at indices 1, 2, 3

        unsafe { Vec::from_raw_parts(range.as_ptr().cast_mut(), range.len(), range.len()) };

        // Validate the list after removal
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], 10);
        assert_eq!(list[1], 50);
    }
}
