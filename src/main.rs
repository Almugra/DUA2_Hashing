use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::{env, time::Instant};

#[derive(PartialEq, Eq)]
struct MyStr<'a>(&'a str);

impl<'a> Hash for MyStr<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    let Some(path) = args.next() else {
        panic!("Missing file path!");
    };

    let mut map: HashSet<MyStr> = HashSet::new();
    let now = Instant::now();
    std::fs::read_to_string(path)
        .expect("Should have been able to read the file!")
        .lines()
        .skip(1)
        .for_each(|line| {
            let split: Vec<&str> = line.split(' ').collect();

            match split[0] {
                "ins" => {
                    println!("{} {}", split[0], map.insert(MyStr(split[1])));
                }
                "del" => {
                    println!("{} {}", split[0], map.remove(&MyStr(split[1])));
                }
                "search" => {
                    println!("{} {}", split[0], map.get(&MyStr(split[1])).is_some());
                }
                _ => unreachable!(),
            }
        });

    println!("Time: {:?}", now.elapsed());
    //./task/pubInst/b08192.txt = Time: ~28ms HashSet
    //./task/pubInst/b08192.txt = Time: ~35ms BTreeSet
    //./task/pubInst/b08192.txt = Time: ~380ms CustomHashFunction
}
