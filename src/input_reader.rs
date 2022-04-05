use std::io;
use std::io::Write;

fn print_parse_error(e: String) {
    eprintln!("Failed to parse input!\nError: {}", e);
}

pub fn input_string(msg: &str) -> String {
    let mut input = String::new();

    print!("{}", msg);
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    return input;
}

pub fn input_i32(msg: &str) -> i32  {
    let input: String = input_string(msg);

    match input.trim().parse::<i32>() {
        Ok(result) => return result,
        Err(e) => print_parse_error(e.to_string())
    }   

    return -1;
}

pub fn input_f32(msg: &str) -> f32 {
    let input: String = input_string(msg);

    match input.trim().parse::<f32>() {
        Ok(result) => return result,
        Err(e) => print_parse_error(e.to_string()) 
    }

    return 0.0;
}
