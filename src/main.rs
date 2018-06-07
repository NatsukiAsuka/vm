use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f: File = File::open("/home/evan/jvm/asset/Main.class").expect("file not found");
    let mut bytes = vec![];
    f.read_to_end(&mut bytes).expect("something went wrong reading the file");
    println!("With text:\n{:?}", bytes);
}
