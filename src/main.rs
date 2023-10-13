use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    //Vector of CMD Arguments
    let args: Vec<String> = env::args().collect();

    //File name and language from CMD line
    let file_name = &args[1];
    let language = &args[2];

    //Vector to store temporary words
    let mut temp_vec: Vec<String> = Vec::new();

    //Vector to store tokens
    let mut tokens: Vec<String> = Vec::new();

    //Lexical and syntax error booleans
    let mut l_error = false;
    let mut s_error = false;


    // Read a file in the local file system
    let mut data_file = File::open(file_name).unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();

    //====================LEXICAL ERROR CHECKING====================\\
    //For loop to check for lexical errors
    for current_char in file_content.chars() {
        if current_char.is_uppercase() {
            l_error = true;
            break;
        }
        else if current_char == '`' || current_char == '!' 
        || current_char == '@' || current_char == '#'
        || current_char == '$' || current_char == '%'
        || current_char == '^' || current_char == '&'
        || current_char == '*' || current_char == ';'
        || current_char == '_' || current_char == '~'
        || current_char == '+' || current_char == '|'
        || current_char == '\\' || current_char == '\''
        || current_char == '/' || current_char == '?'
        || current_char == '<' || current_char == '>'
        || current_char == '[' || current_char == ']'
        || current_char == '{' || current_char == '}' {
            l_error = true;
            break;
        }
    }
    //====================LEXICAL ERROR CHECKING====================\\


    //=======================TOKEN GENERATING=======================\\
    //Used to check the current word in the current program line in the second for loop
    let mut temp_word = String::new();

    //Used to check the number of quotations gone over in the current program line in the second for loop
    let mut quote_count = 2;

    //For loop used to generate words for the tokens
    for line in file_content.split("\n") {
        if temp_word.len() > 0 {
            temp_vec.push(temp_word);
        }
        temp_word = String::new();
        
        //For loop that goes through each line in the file and checks every individual character
        for current_char in line.chars() {
            //Checks if character is a comma
            if current_char == ',' && quote_count == 2 {
                if temp_word.len() != 0 {
                    temp_vec.push(temp_word);
                    temp_word = String::new();
                }
                let mut comma = String::new();
                comma.push(current_char);
                temp_vec.push(comma);
            }
            //Checks if character is a period
            else if current_char == '.' && quote_count == 2 {
                if temp_word.len() != 0 {
                    temp_vec.push(temp_word);
                    temp_word = String::new();
                }
                let mut period = String::new();
                period.push(current_char);
                temp_vec.push(period);
            }
            //Checks if character is a left parenthesis
            else if current_char == '(' && quote_count == 2 {
                if temp_word.len() != 0 {
                    temp_vec.push(temp_word);
                    temp_word = String::new();
                }
                let mut l_paren = String::new();
                l_paren.push(current_char);
                temp_vec.push(l_paren);
            }
            //Checks if character is a right parenthesis
            else if current_char == ')' && quote_count == 2 {
                if temp_word.len() != 0 {
                    temp_vec.push(temp_word);
                    temp_word = String::new();
                }
                let mut r_paren = String::new();
                r_paren.push(current_char);
                temp_vec.push(r_paren);
            }
            //Checks if character is a colon
            else if current_char == ':' && quote_count == 2 {
                if temp_word.len() != 0 {
                    temp_vec.push(temp_word);
                    temp_word = String::new();
                }
                let mut colon = String::new();
                colon.push(current_char);
                temp_vec.push(colon);
            }
            //Checks if character is an equals sign
            else if current_char == '=' && quote_count == 2 {
                if temp_word.len() != 0 {
                    temp_vec.push(temp_word);
                    temp_word = String::new();
                }
                let mut equals = String::new();
                equals.push(current_char);
                temp_vec.push(equals);
            }
            //Checks if character is a quotation mark
            else if current_char == '\"' && quote_count == 2 {
                if temp_word.len() != 0 {
                    temp_vec.push(temp_word);
                    temp_word = String::new();
                }
                quote_count = quote_count - 1;
                temp_word.push(current_char);
            }
            //Checks for the second quotation mark
            else if current_char == '\"' && quote_count == 1 {
                temp_word.push(current_char);
                temp_vec.push(temp_word);
                temp_word = String::new();
                quote_count = quote_count + 1;
            }
            //Checks if character is a whitespace
            else if !current_char.is_whitespace() && quote_count == 2 {
                temp_word.push(current_char);
            }
            //Checks for anything else
            else if quote_count == 1 {
                temp_word.push(current_char);
            }
        }
    }
    
    //For loop that goes through temp_vec, generates tokens based off of the element in temp_vec, and pushes token to tokens vector
    for element in &temp_vec {
        //Checks if element in temp_vec is "data", generates DATA token
        if element == "data" {
            tokens.push("DATA".to_string());
        }
        //Checks if element in temp_vec is "input", generates INPUT token
        else if element == "input" {
            tokens.push("INPUT".to_string());
        }
        //Checks if element in temp_vec is "process", generates PROCESS token
        else if element == "process" {
            tokens.push("PROCESS".to_string());
        }
        //Checks if element in temp_vec is "output", generates OUTPUT token
        else if element == "output" {
            tokens.push("OUTPUT".to_string());
        }
        //Checks if element in temp_vec is "end", generates END token
        else if element == "end" {
            tokens.push("END".to_string());
        }
        //Checks if element in temp_vec is "true", generates TRUE token
        else if element == "true" {
            tokens.push("TRUE".to_string());
        }
        //Checks if element in temp_vec is "false", generates FALSE token
        else if element == "false" {
            tokens.push("FALSE".to_string());
        }
        //Checks if element in temp_vec is "read", generates READ token
        else if element == "read" {
            tokens.push("READ".to_string());
        }
        //Checks if element in temp_vec is a colon, generates COLON token
        else if element == ":" {
            tokens.push("COLON".to_string());
        }
        //Checks if element in temp_vec is a comma, generates COMMA token
        else if element == "," {
            tokens.push("COMMA".to_string());
        }
        //Checks if element in temp_vec is a period, generates PERIOD token
        else if element == "." {
            tokens.push("PERIOD".to_string());
        }
        //Checks if element in temp_vec is a left parenthesis, generates LPAREN token
        else if element == "(" {
            tokens.push("LPAREN".to_string());
        }
        //Checks if element in temp_vec is a right parenthesis, generates RPAREN token
        else if element == ")" {
            tokens.push("RPAREN".to_string());
        }
        //Checks if element in temp_vec is an equals sign, generates ASSIGN token
        else if element == "=" {
            tokens.push("ASSIGN".to_string());
        }
        //Checks if element in temp_vec is "vector", generates VECTOR token
        else if element == "vector" {
            tokens.push("VECTOR".to_string());
        }
        //Checks if element in temp_vec is "number", generates NUMBER token
        else if element == "number" {
            tokens.push("NUMBER".to_string());
        }
        //Checks if element in temp_vec is "regressiona", generates REGRESSIONA token
        else if element == "regressiona" {
            tokens.push("REGRESSIONA".to_string());
        }
        //Checks if element in temp_vec is "regressionb", generates REGRESSIONB token
        else if element == "regressionb" {
            tokens.push("REGRESSIONB".to_string());
        }
        //Checks if element in temp_vec is "mean", generates MEAN token
        else if element == "mean" {
            tokens.push("MEAN".to_string());
        }
        //Checks if element in temp_vec is "stddev", generates STDDEV token
        else if element == "stddev" {
            tokens.push("STDDEV".to_string());
        }
        //Checks if element in temp_vec is "correlation", generates CORRELATION token
        else if element == "correlation" {
            tokens.push("CORRELATION".to_string());
        }
        //Checks for the remaining tokens
        else {
            //Goes through temp_vec element characters
            for c in element.chars() {
                //Checks if element in temp_vec is a number, generates NUM token
                if c.is_numeric() {
                    let mut num: String = "NUM ".to_owned();
                    let add: &str = element;
                    num.push_str(add);
                    tokens.push(num);
                    break;
                }
                //Checks if element in temp_vec is a string, generates STRING token
                else if c == '\"' {
                    let mut string: String = "STRING ".to_owned();
                    let add: &str = element;
                    string.push_str(add);
                    tokens.push(string);
                    break;
                }
                //Checks for ids, generates ID token
                else {
                    let mut id: String = "ID ".to_owned();
                    let add: &str = element;
                    id.push_str(add);
                    tokens.push(id);
                    break;
                }
            }
        }
    }
    //=======================TOKEN GENERATING=======================\\


    //=====================SYNTAX ERROR CHECKING=====================\\
    //Token vector index
    let mut t_index = 0;
    //Checks if while loop is done
    let mut done = false;

    //Checks if the DATA token is in the right place. If it isn't, returns a syntax error
    if tokens.get(t_index).expect("REASON").to_string() != "DATA" {
        s_error = true;
    }
    else {
        t_index = t_index + 1;
    }
    if tokens.get(t_index).expect("REASON").to_string() != "COLON" {
        s_error = true;
    }
    else {
        t_index = t_index + 1;
    }

    //First while loop that goes through the tokens vector until it reaches either the INPUT, PROCESS, or OUTPUT tokens
    while !done {
        if tokens.get(t_index).expect("REASON").to_string() == "INPUT" {
            done = true;
        }
        else if tokens.get(t_index).expect("REASON").to_string() == "PROCESS" {
            done = true;
        }
        else if tokens.get(t_index).expect("REASON").to_string() == "OUTPUT" {
            done = true;
        }
        else {
            t_index = t_index + 1;
        }
    }

    //Checks if the INPUT token is in the right place. If it isn't, returns a syntax error
    if tokens.get(t_index).expect("REASON").to_string() != "INPUT" {
        s_error = true;
    }
    else {
        t_index = t_index + 1;
    }
    if tokens.get(t_index).expect("REASON").to_string() != "COLON" {
        s_error = true;
    }
    else {
        t_index = t_index + 1;
    }

    //Second while loop that goes through the tokens vector until it reaches either the INPUT, PROCESS, or OUTPUT tokens
    done = false;
    while !done {
        if tokens.get(t_index).expect("REASON").to_string() == "INPUT" {
            done = true;
        }
        else if tokens.get(t_index).expect("REASON").to_string() == "PROCESS" {
            done = true;
        }
        else if tokens.get(t_index).expect("REASON").to_string() == "OUTPUT" {
            done = true;
        }
        else {
            t_index = t_index + 1;
        }
    }

    //Checks if the PROCESS token is in the right place. If it isn't, returns a syntax error
    if tokens.get(t_index).expect("REASON").to_string() != "PROCESS" {
        s_error = true;
    }
    else {
        t_index = t_index + 1;
    }
    if tokens.get(t_index).expect("REASON").to_string() != "COLON" {
        s_error = true;
    }
    else {
        t_index = t_index + 1;
    }
    
    //Third while loop that goes through the tokens vector until it reaches either the INPUT, PROCESS, or OUTPUT tokens
    done = false;
    while !done {
        if tokens.get(t_index).expect("REASON").to_string() == "INPUT" {
            done = true;
        }
        else if tokens.get(t_index).expect("REASON").to_string() == "PROCESS" {
            done = true;
        }
        else if tokens.get(t_index).expect("REASON").to_string() == "OUTPUT" {
            done = true;
        }
        else {
            t_index = t_index + 1;
        }
    }

    //Checks if the OUTPUT token is in the right place. If it isn't, returns a syntax error
    if tokens.get(t_index).expect("REASON").to_string() != "OUTPUT" {
        s_error = true;
    }
    else {
        t_index = t_index + 1;
    }
    if tokens.get(t_index).expect("REASON").to_string() != "COLON" {
        s_error = true;
    }
    //=====================SYNTAX ERROR CHECKING=====================\\


    //========================RUNNING PROGRAM========================\\
    //Checks if the program is to be written in scheme
    if language == "-s" {
        //Processing file statement
        println!("; processing input file {}", file_name);

        //Checks for lexical error
        if l_error {
            println!("; Lexical error found, stopping program");
        }
        //Checks for syntax error
        else if s_error {
            println!("; Syntax error found, stopping program");
        }
        //If no lexical or syntax errors, run program
        else {
            //Starting statement
            println!("; Lexical and Syntax analysis passed");

            let mut is_input = false;
            let mut is_process = false;
            let mut is_output = false;

            let mut return_i = String::from("(define ");
            let mut return_p = String::from("(define");
            let mut return_o = String::from("(display");

            for element in &tokens {
                if element == "INPUT" {
                    is_input = true;
                    is_process = false;
                    is_output = false;
                }
                else if element == "PROCESS" {
                    is_input = false;
                    is_process = true;
                    is_output = false;
                }
                else if element == "OUTPUT" {
                    is_input = false;
                    is_process = false;
                    is_output = true;
                }

                if is_input {
                    if element.contains("ID ") {
                        for c in element.chars() {
                            if c.is_lowercase() && !c.is_whitespace() {
                                return_i.push(c);
                            }
                        }
                    }
                    else if element == "READ" {
                        return_i.push_str(" (read-csv \"");
                    }
                    else if element.contains("STRING ") {
                        return_i.push_str("./");
                        for c in element.chars() {
                            if c.is_lowercase() && !c.is_whitespace() && c != '\"' {
                                return_i.push(c);
                            }
                        }
                        return_i.push_str("\" ");
                    }
                    else if element == "TRUE" {
                        return_i.push_str("#t ");
                    }
                    else if element == "FALSE" {
                        return_i.push_str("#f ");
                    }
                    else if element.contains("NUM ") {
                        for c in element.chars() {
                            if c.is_numeric() && !c.is_whitespace() {
                                return_i.push(c);
                            }
                        }
                    }
                    else if element == "RPAREN" {
                        return_i.push_str("))");
                        println!("{}", return_i);
                        return_i = String::from("(define ");
                    }
                }
                else if is_process {
                    if element.contains("ID ") {
                        return_p.push_str(" ");
                        for c in element.chars() {
                            if c.is_lowercase() && !c.is_whitespace() {
                                return_p.push(c);
                            }
                        }
                    }
                    else if element == "REGRESSIONA" {
                        return_p.push_str(" (regressiona");
                    }
                    else if element == "REGRESSIONB" {
                        return_p.push_str(" (regressionb");
                    }
                    else if element == "CORRELATION" {
                        return_p.push_str(" (correlation");
                    }
                    else if element == "MEAN" {
                        return_p.push_str(" (mean");
                    }
                    else if element == "STDDEV" {
                        return_p.push_str(" (stddev");
                    }
                    else if element == "RPAREN" {
                        return_p.push_str("))");
                        println!("{}", return_p);
                        return_p = String::from("(define");
                    }
                }
                else if is_output {
                    if element.contains("STRING ") {
                        for c in element.chars() {
                            if c.is_lowercase() || c.is_whitespace() || c == '\"' || c == '=' {
                                return_o.push(c);
                            }
                        }
                        return_o.push(')');
                        println!("{}", return_o);
                        println!("(newline)");
                        return_o = String::from("(display");
                    }
                    else if element.contains("ID ") {
                        return_o.push_str(" ");
                        for c in element.chars() {
                            if c.is_lowercase() && !c.is_whitespace() {
                                return_o.push(c);
                            }
                        }
                        return_o.push(')');
                        println!("{}", return_o);
                        println!("(newline)");
                        return_o = String::from("(display");
                    }
                }
            }
        }
    }
    //Checks if the program is to be written in prolog
    else if language == "-p" {
        //Processing file statement
        println!("/* processing input file {}", file_name);

        //Checks for lexical error
        if l_error {
            println!("   Lexical error found, stopping program */");
        }
        //Checks for syntax error
        else if s_error {
            println!("   Syntax error found, stopping program */");
        }
        //If no lexical or syntax errors, run program
        else {
            //Starting statement
            println!("   Lexical and Syntax analysis passed */");

            let mut count = 1;
            for element in &temp_vec {
                print!("{}", count);
                print!(": ");
                println!("{}", element); 
                count = count + 1;
            }
        }
    }
    //Prompts user to enter valid command line argument if not valid
    else {
        println!("Program failed. Please enter a valid command.");
    }
    //========================RUNNING PROGRAM========================\\
}