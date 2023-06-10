use std::{collections::HashSet, env, time::Instant};

fn main() {
    let mut args = env::args();
    args.next();

    let Some(path) = args.next() else {
        panic!("Missing file path!");
    };

    let mut map: HashSet<&str> = HashSet::new();
    let now = Instant::now();
    std::fs::read_to_string(path)
        .expect("Should have been able to read the file!")
        .lines()
        .skip(1)
        .for_each(|line| {
            let split: Vec<&str> = line.split(' ').collect();

            match split[0] {
                "ins" => {
                    println!("{} {}", split[0], map.insert(split[1]));
                }
                "del" => {
                    println!("{} {}", split[0], map.remove(split[1]));
                }
                "search" => {
                    println!("{} {}", split[0], map.get(split[1]).is_some());
                }
                _ => unreachable!(),
            }
        });

    println!("Time: {:?}", now.elapsed());
    //./task/pubInst/b08192.txt = Time: 28.586461ms HashSet 24ms best/33ms worst
}
