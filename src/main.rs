#![allow(non_snake_case)]

fn main() {
    use std::process::exit;
    use std::env;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not found arguments");
        exit(1);
    }

    set_mode(&args);
}


fn set_mode(args: &Vec<String>) -> &str {
    //if let Some(index) = args.iter().position(|val| val == key){
    match(args[index].to_str()) {
        "-I" => return "-I",
        "-S" => return "-S",
        "-U" => println!("В разработке"),
        _ => {
            println!("Not found  arguments");
            return "";
        },
    }
}

fn get_data(args: &Vec<String>) {
    struct ins {
        table: String,
        column: Vec<String>,
        value: Vec<String>,
    }

    let data: ins;
    for index in 3..args.len() {
        match(args[index].to_str()) {
            "-t" => data.table = args[index+1],
            "-c" => , //Нужно сделать эту функцию рекурсивной и вылудить данные параметров
            "-v" => ,
            _ => ,
        } 
    }
}

fn insert_db(args: &Vec<String>);
fn select_db(args: &Vec<String>);
