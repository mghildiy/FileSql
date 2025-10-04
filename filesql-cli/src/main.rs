use std::fs::read_to_string;

mod parser;

fn main() {

    println!("Hello, welcome to world of querying your csv/json files!");

    let s = read_to_string("/Users/mghildiy/work/rust-learning/FileSQL/people-1000.csv")
        .expect("File failed ot be read");
    print!("File content is {}", s)
}
