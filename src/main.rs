#![allow(non_snake_case)]

use rusqlite::{Connection, Result};


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


fn set_mode(args: &Vec<String>) -> Result<()>{//, &'static str> {
    match args[1].as_str() {
        "-I" => DataForDB::insert_db(&args)?,
        "-S" => DataForDB::select_db(&args)?,
        _ => {
            println!("Not found  arguments");
            //return Err("Not found arguments");
        },
    }
    Ok(())
}


struct DataForDB {
    target: String,
    column: Vec<String>,
    value: Vec<String>,
}

impl DataForDB {
    fn get_data(args: &Vec<String>) -> DataForDB {
        let obj_data: DataForDB;
        for index in 3..args.len() {
            match args[index].as_str() {
                "-t" => obj_data.target = args[index+1],
                "-c" => obj_data.column = get_value_args(args[index+1..]),
                "-v" => obj_data.value = get_value_args(args[index+1..]),
                _ => println!("Error: что-то пошло не так. {}", args[index]),
            } 
        }
        return obj_data;

        fn get_value_args(args: Vec<String>) {
            let result = String::from("");
            for element in args {
                if element.get(0) != Some("-") { result += &element }
                else { break }
            }
            return result;
        }
    }

    pub fn insert_db(args: &Vec<String>) -> Result<()>{
        Self::get_data(&args);
    }
    pub fn select_db(args: &Vec<String>) -> Result<()>{
        Self::get_data(&args);
    }
}
