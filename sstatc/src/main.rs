use std::env::args;
use std::fs::File;
use std::io::Read;

use sstat::transpile;

fn main() {
    let filename = args().nth(1).unwrap();
    let mut file = File::open(filename.clone()).unwrap();

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();

    match transpile(filename, source) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
