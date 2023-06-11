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

#[cfg(test)]
mod tests {
    use crate::HashTable;

    #[test]
    fn test_all_cases() {
        assert!(compare_all("./task/pubInst/"))
    }

    fn compare_all(dir: &str) -> bool {
        let mut entries = std::fs::read_dir(dir)
            .unwrap()
            .filter(|entry| {
                !entry
                    .as_ref()
                    .unwrap()
                    .path()
                    .to_string_lossy()
                    .ends_with(".sol")
            })
            .map(|entry| entry.unwrap().path());

        entries.all(|path| {
            let path = path.to_string_lossy().to_string();
            let res = convert_input(&path);
            let sol = std::fs::read_to_string(path + ".sol").unwrap();
            res == sol
        })
    }

    fn convert_input(path: &str) -> String {
        let mut map: HashTable = HashTable::hashCreate(8192);
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| {
                let split: Vec<&str> = line.split(' ').collect();
                match split[0] {
                    "ins" => {
                        format!("{} {}\n", split[0], map.hashInsert(split[1].to_string()))
                    }
                    "del" => {
                        format!("{} {}\n", split[0], map.hashRemove(split[1].to_string()))
                    }
                    "search" => {
                        format!("{} {}\n", split[0], map.hashSearch(split[1].to_string()))
                    }
                    _ => unreachable!(),
                }
            })
            .collect::<String>()
    }
}
