use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let language = &args[2];
    let mut temp_vec: Vec<String> = Vec::new();
    //let tokens: Vec<String> = Vec::new();

    if language == "-s" {
        println!("; processing input file {}", file_name);

        // Read a file in the local file system
        let mut data_file = File::open(file_name).unwrap();
        // Create an empty mutable string
        let mut file_content = String::new();
        // Copy contents of file to a mutable string
        data_file.read_to_string(&mut file_content).unwrap();

        let mut temp_word = String::new();
        let mut quote_count = 2;
        for line in file_content.split("\n") {
            if temp_word.len() > 0 {
                temp_vec.push(temp_word);
            }
            temp_word = String::new();
            
            for current_char in line.chars() {
                if current_char == ',' && quote_count == 2 {
                    if temp_word.len() != 0 {
                        temp_vec.push(temp_word);
                        temp_word = String::new();
                    }
                    let mut comma = String::new();
                    comma.push(current_char);
                    temp_vec.push(comma);
                }
                else if current_char == '.' && quote_count == 2 {
                    if temp_word.len() != 0 {
                        temp_vec.push(temp_word);
                        temp_word = String::new();
                    }
                    let mut period = String::new();
                    period.push(current_char);
                    temp_vec.push(period);
                }
                else if current_char == '(' && quote_count == 2 {
                    if temp_word.len() != 0 {
                        temp_vec.push(temp_word);
                        temp_word = String::new();
                    }
                    let mut l_paren = String::new();
                    l_paren.push(current_char);
                    temp_vec.push(l_paren);
                }
                else if current_char == ')' && quote_count == 2 {
                    if temp_word.len() != 0 {
                        temp_vec.push(temp_word);
                        temp_word = String::new();
                    }
                    let mut r_paren = String::new();
                    r_paren.push(current_char);
                    temp_vec.push(r_paren);
                }
                else if current_char == ':' && quote_count == 2 {
                    if temp_word.len() != 0 {
                        temp_vec.push(temp_word);
                        temp_word = String::new();
                    }
                    let mut colon = String::new();
                    colon.push(current_char);
                    temp_vec.push(colon);
                }
                else if current_char == '=' && quote_count == 2 {
                    if temp_word.len() != 0 {
                        temp_vec.push(temp_word);
                        temp_word = String::new();
                    }
                    let mut equals = String::new();
                    equals.push(current_char);
                    temp_vec.push(equals);
                }
                else if current_char == '\"' && quote_count == 2 {
                    if temp_word.len() != 0 {
                        temp_vec.push(temp_word);
                        temp_word = String::new();
                    }
                    quote_count = quote_count - 1;
                    temp_word.push(current_char);
                }
                else if current_char == '\"' && quote_count == 1 {
                    temp_word.push(current_char);
                    temp_vec.push(temp_word);
                    temp_word = String::new();
                    quote_count = quote_count + 1;
                }
                else if !current_char.is_whitespace() && quote_count == 2 {
                    temp_word.push(current_char);
                }
                else if quote_count == 1 {
                    temp_word.push(current_char);
                }
            }
        }

        let mut count = 1;
        for element in &temp_vec {
            print!("{}", count);
            print!(": ");
            println!("{}", element); 
            count = count + 1;
        }
        println!("Vector Size: {}", temp_vec.len());
    }
    else if language == "-p" {
        println!("/* processing input file {}", file_name);
    }
    else {
        println!("Program failed. Please enter a valid command.");
    }
}