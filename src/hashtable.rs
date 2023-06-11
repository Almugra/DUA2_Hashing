#![allow(non_snake_case)]
use std::{cell::RefCell, ptr};

const CHAR_TO_NUM: [u64; 256] = {
    let mut map = [0; 256];
    map[b'a' as usize] = 0;
    map[b'b' as usize] = 1;
    map[b'c' as usize] = 2;
    map[b'd' as usize] = 3;
    map
};

#[derive(PartialEq, Eq)]
pub struct HashTable {
    pub map: Vec<RefCell<LinkedList<String>>>,
    pub size: usize,
}

impl HashTable {
    pub fn hashCreate(n: usize) -> Self {
        Self {
            map: vec![RefCell::default(); n],
            size: n,
        }
    }

    pub fn hashInsert(&mut self, x: String) -> bool {
        let index = self.hash(&x);
        let Some(list) = self.map.get(index) else {
            unreachable!()
        };

        let state = !list.borrow().iter().any(|y| y == &x);

        if state {
            list.borrow_mut().push(x);
        }

        state
    }

    pub fn hashRemove(&mut self, x: String) -> bool {
        let index = self.hash(&x);
        let Some(list) = self.map.get(index) else {
            unreachable!()
        };

        let mut state = false;
        let mut new_list = LinkedList::new();

        for y in list.borrow().iter() {
            if &x == y {
                state = true;
            } else {
                new_list.push(y.to_string());
            }
        }
        self.map[index] = RefCell::new(new_list);
        state
    }

    pub fn hashSearch(&mut self, x: String) -> bool {
        let index = self.hash(&x);
        let Some(list) = self.map.get(index) else {
            unreachable!()
        };

        list.borrow().iter().any(|y| y == &x)
    }

    fn hash(&self, key: &str) -> usize {
        let mut value = 0u64;
        for byte in key.bytes() {
            value *= 4;
            value += CHAR_TO_NUM[byte as usize];
        }
        value as usize % self.size
    }
}

type Link<T> = *mut Node<T>;

#[derive(PartialEq, Eq, Clone)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

#[derive(PartialEq, Eq, Clone)]
struct Node<T> {
    element: T,
    next: Link<T>,
}

pub struct IntoIter<T>(LinkedList<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, element: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                element,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.element
            })
        }
    }
}
