use std::fmt::{self, Display};

/// A minimal, singly linked list that avoids using `Vec`.
///
/// This list supports adding elements from any iterable while preserving
/// their original order. It implements `Display` for easy printing and
/// provides an iterator for traversal.
#[derive(Clone)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    /// Creates a new, empty list.
    ///
    /// # Example
    /// ```
    /// use vecless::List;
    /// let list: List<i32> = List::new();
    /// ```
    pub fn new() -> Self {
        List { head: None }
    }

    /// Adds multiple elements to the list, preserving their original order.
    ///
    /// Accepts any type that implements `IntoIterator`, such as arrays,
    /// vectors, ranges, or iterators.
    ///
    /// # Example
    /// ```
    /// use vecless::List;
    /// let list = List::new().add(["a", "b", "c"]);
    /// assert_eq!(format!("{}", list), "[a, b, c]");
    /// ```
    pub fn add(self, items: impl IntoIterator<Item = T>) -> Self {
        let mut temp = List::new();
        for item in items {
            temp = temp.push(item);
        }
        self.append(temp.reverse())
    }

    /// Pushes a single element to the front of the list.
    ///
    /// This method reverses the order of elements if used repeatedly.
    ///
    /// # Example
    /// ```
    /// use vecless::List;
    /// let list = List::new().push(1).push(2);
    /// assert_eq!(format!("{}", list), "[2, 1]");
    /// ```
    pub fn push(mut self, elem: T) -> Self {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self
    }

    /// Appends another list to the end of this one.
    ///
    /// This is used internally by `.add()` to preserve order.
    fn append(mut self, mut other: Self) -> Self {
        if self.head.is_none() {
            return other;
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = other.head.take();
                break;
            }
            current = &mut node.next;
        }
        self
    }

    /// Reverses the list.
    ///
    /// This is used internally to restore the original order of added items.
    pub fn reverse(mut self) -> Self {
        let mut reversed = List::new();
        while let Some(node) = self.head.take() {
            reversed = reversed.push(node.elem);
            self.head = node.next;
        }
        reversed
    }

    /// Returns the number of elements in the list.
    ///
    /// # Example
    /// ```
    /// use vecless::List;
    /// let list = List::new().add([1, 2, 3]);
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns an iterator over the list's elements.
    ///
    /// # Example
    /// ```
    /// use vecless::List;
    /// let list = List::new().add([1, 2, 3]);
    /// let sum: i32 = list.iter().copied().sum();
    /// assert_eq!(sum, 6);
    /// ```
    pub fn iter(&self) -> ListIter<'_, T> {
        ListIter {
            next: self.head.as_deref(),
        }
    }
}

/// An iterator over references to the elements of a `List`.
pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T: Display> Display for List<T> {
    /// Formats the list using the standard list syntax: `[a, b, c]`.
    ///
    /// # Example
    /// ```
    /// use vecless::List;
    /// let list = List::new().add(["x", "y"]);
    /// assert_eq!(format!("{}", list), "[x, y]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        write!(f, "[")?;
        for item in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
            first = false;
        }
        write!(f, "]")
    }
}
