#![allow(non_snake_case)]
use std::env;
use std::time::Instant;
pub mod hashtable;
use crate::hashtable::HashTable;

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
                    println!("{} {}", split[0], map.hashInsert(split[1].to_string()));
                }
                "del" => {
                    println!("{} {}", split[0], map.hashRemove(split[1].to_string()));
                }
                "search" => {
                    println!("{} {}", split[0], map.hashSearch(split[1].to_string()));
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
//            "             = Time: ~30ms Custom Hashing + HashTable + List
