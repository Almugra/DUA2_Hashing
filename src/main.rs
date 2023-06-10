use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::{env, time::Instant};

const CHAR_TO_NUM: [u64; 256] = {
    let mut map = [0; 256];
    map[b'a' as usize] = 0;
    map[b'b' as usize] = 1;
    map[b'c' as usize] = 2;
    map[b'd' as usize] = 3;
    map
};

#[derive(PartialEq, Eq)]
struct MyStr<'a>(&'a str);

impl<'a> Hash for MyStr<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut value = 0u64;
        for byte in self.0.bytes() {
            value *= 4;
            value += CHAR_TO_NUM[byte as usize]
        }
        value.hash(state);
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
    //./task/pubInst/b08192.txt = Time: ~30ms CustomHashByteFunction
}
