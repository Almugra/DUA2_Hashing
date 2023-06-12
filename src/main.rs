use std::env;
use std::time::Instant;
pub mod hashtable;
use crate::hashtable::HashTable;

const INS: &str = "ins";
const DEL: &str = "del";
const SEARCH: &str = "search";

fn main() {
    let mut args = env::args();
    args.next();

    let Some(path) = args.next() else {
        panic!("missing file path");
    };

    let mut map: HashTable = HashTable::hashCreate(8000);
    let now = Instant::now();

    let data = std::fs::read_to_string(path).expect("tried reading file");

    for line in data.lines().skip(1) {
        let split: Vec<&str> = line.split(' ').collect();

        match split[0] {
            INS => {
                println!("{} {}", INS, map.hashInsert(split[1].to_string()));
            }
            DEL => {
                println!("{} {}", DEL, map.hashRemove(split[1].to_string()));
            }
            SEARCH => {
                println!("{} {}", SEARCH, map.hashSearch(split[1].to_string()));
            }
            _ => unreachable!(),
        }
    }

    println!("Time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::HashTable;
    use crate::DEL;
    use crate::INS;
    use crate::SEARCH;

    #[test]
    fn test_all_cases() {
        assert!(compare_all("./task/pubInst/"))
    }

    fn compare_all(dir: &str) -> bool {
        let mut entries = std::fs::read_dir(dir)
            .unwrap()
            .filter(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .path()
                    .extension()
                    .map_or(true, |ext| ext != "sol")
            })
            .map(|entry| entry.unwrap().path());

        entries.all(|path| {
            let res = convert_input(path.clone());
            let sol = std::fs::read_to_string(path.with_extension("txt.sol")).unwrap();
            res == sol
        })
    }

    fn convert_input(path: PathBuf) -> String {
        let mut map: HashTable = HashTable::hashCreate(8192);
        let data = std::fs::read_to_string(path).expect("tried reading file");

        data.lines()
            .skip(1)
            .map(|line| {
                let split: Vec<&str> = line.split(' ').collect();
                match split[0] {
                    INS => {
                        format!("{} {}\n", INS, map.hashInsert(split[1].to_string()))
                    }
                    DEL => {
                        format!("{} {}\n", DEL, map.hashRemove(split[1].to_string()))
                    }
                    SEARCH => {
                        format!("{} {}\n", SEARCH, map.hashSearch(split[1].to_string()))
                    }
                    _ => unreachable!(),
                }
            })
            .collect::<String>()
    }
}
