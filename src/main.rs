use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let language = &args[2];

    if language == "-s" {
        println!("{}", file_name);
        println!("{}", language);
    }
    else {
        println!("do something else");
    }
}
