use std::env;
use std::io::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let language = &args[2];

    println!("; Processing input file {}", file_name);

    if language == "-s" {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }
    else {
        println!("doesn't work");
    }
}