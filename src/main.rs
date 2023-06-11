#![allow(non_snake_case)]
use std::cell::RefCell;
use std::collections::LinkedList;
use std::env;
use std::time::Instant;

const CHAR_TO_NUM: [u64; 256] = {
    let mut map = [0; 256];
    map[b'a' as usize] = 0;
    map[b'b' as usize] = 1;
    map[b'c' as usize] = 2;
    map[b'd' as usize] = 3;
    map
};

#[derive(PartialEq, Eq)]
struct HashTable<'a> {
    map: Vec<RefCell<LinkedList<&'a str>>>,
    size: usize,
}

impl<'a> HashTable<'a> {
    fn hashCreate(n: usize) -> Self {
        Self {
            map: vec![RefCell::default(); n],
            size: n,
        }
    }

    fn hashInsert(&mut self, x: &'a str) -> bool {
        let index = self.hash(x);
        let Some(list) = self.map.get(index) else {
            unreachable!()
        };

        if list.borrow().contains(&x) {
            false
        } else {
            list.borrow_mut().push_back(x);
            true
        }
    }

    fn hashRemove(&mut self, x: &'a str) -> bool {
        let index = self.hash(x);
        let Some(list) = self.map.get(index) else {
            unreachable!()
        };

        let mut state = false;
        let mut new_list = LinkedList::new();

        for y in list.borrow().iter() {
            if &x == y {
                state = true;
            } else {
                new_list.push_back(*y);
            }
        }
        self.map[index] = RefCell::new(new_list);
        state
    }

    fn hashSearch(&mut self, x: &'a str) -> bool {
        let index = self.hash(x);
        let Some(list) = self.map.get(index) else {
            unreachable!()
        };

        list.borrow().contains(&x)
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

fn main() {
    let mut args = env::args();
    args.next();

    let Some(path) = args.next() else {
        panic!("Missing file path!");
    };

    let mut map: HashTable = HashTable::hashCreate(8192);
    let now = Instant::now();
    std::fs::read_to_string(path)
        .expect("Should have been able to read the file!")
        .lines()
        .skip(1)
        .for_each(|line| {
            let split: Vec<&str> = line.split(' ').collect();

            match split[0] {
                "ins" => {
                    println!("{} {}", split[0], map.hashInsert(split[1]));
                }
                "del" => {
                    println!("{} {}", split[0], map.hashRemove(split[1]));
                }
                "search" => {
                    println!("{} {}", split[0], map.hashSearch(split[1]));
                }
                _ => unreachable!(),
            }
        });

    println!("Time: {:?}", now.elapsed());
}

//./task/pubInst/b08192.txt = Time: ~28ms HashSet
//            "             = Time: ~35ms BTreeSet
//            "             = Time: ~380ms Custom Hashing
//            "             = Time: ~30ms Custom Hashing with bytes
//            "             = Time: ~29ms Custom Hashing + HashTable
