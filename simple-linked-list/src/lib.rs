use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> Node<T> {

    fn len(&self) -> usize {
        match &self.next {
            Some(n) => 1 + n.len(),
            None => 1
        }
    }

    fn unwind(mut self) -> Vec<T> {
        let mut vec = self.next.take().map_or(vec!(), |next| next.unwind());
        vec.push(self.data);
        vec
    }

    fn rev(self, mut lst:SimpleLinkedList<T>) -> SimpleLinkedList<T> {
        lst.push(self.data);
        match self.next {
            Some(next) => {
                next.rev(lst)
            },
            _ => lst
        }
    }

}

impl<T> SimpleLinkedList<T> {

    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.head.as_ref().map_or(0, |n| n.len())
    }

    pub fn push(&mut self, _element: T) {
        let next = &mut self.head;
        self.head = Some(Box::new(Node {
            data: _element,
            next: next.take()
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        self.head.map_or(SimpleLinkedList::new(), |head| head.rev(SimpleLinkedList::new()))
    }

}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in _iter {
            list.push(i)
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        self.head.map_or(vec!(), |node| node.unwind())
    }
}


