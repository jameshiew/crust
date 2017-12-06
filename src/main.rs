use std::env;

fn main() {
    for (i, arg) in env::args().enumerate() {
        if i == 0 {
            continue
        }
        if i == 1 {
            println!("{}", arg);
        }
    }
}
