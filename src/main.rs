use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let Some(path) = args.next() else {
        panic!("Missing file path!");
    };

    let contents = std::fs::read_to_string(path)
        .expect("Should have been able to read the file!")
        .lines()
        .skip(1);
}
