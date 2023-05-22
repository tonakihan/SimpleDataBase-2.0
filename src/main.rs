#![allow(non_snake_case)]

//use std::error::Error; //TODO:Для impl CustomError, но потом
use rusqlite::{Connection, Result};
use std::fmt; //Для ipml CustomError


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


// Создал свой тип ошибки. Т.к. я хочу свой 
// текст и rusqlite::Error в одном.
#[derive(Debug)]
struct CustomE {
    message: String,
}

impl fmt::Display for CustomE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

//TODO:Классная штука, но потом.
//impl Error for CustomError {} 

impl From<rusqlite::Error> for CustomE {
    fn from(err: rusqlite::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

fn set_mode(args: &Vec<String>) -> Result<(), CustomE> {
    match args[1].as_str() {
        "-I" => DataForDB::insert_db(&args)?,
        "-S" => DataForDB::select_db(&args)?,
        _ => {
            println!("Not found  arguments");
            return Err(CustomE {
                message: "Not found arguments".to_string()
            });
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
    fn new() -> Self {
        Self {
            target: String::new(),
            column: Vec::<String>::new(),
            value: Vec::<String>::new(),
        }
    }

    fn get_data(args: &Vec<String>) -> Self {
        //Вылуживает данные из строки запуска
        let mut obj_data = Self::new();
        for index in 3..args.len() {
            match args[index].as_str() {
                "-t" => obj_data.target = args[index+1].clone(),
                "-c" => obj_data.column = get_value_args(args, index+1),
                "-v" => obj_data.value = get_value_args(args, index+1),
                //TODO:Переделать на нормальную ошибку
                _ => println!("Error: что-то пошло не так. {}", args[index]),
            } 
        }
        return obj_data;

        fn get_value_args(args: &Vec<String>, index: usize) -> Vec<String> {
            //Оно принимает строку запуска + индекс от куда начать
            let mut result = Vec::<String>::new();
            for element in &args[index..] {
                //Тут чекаю является ли элемент > ключом запуска
                if (element.chars().nth(0) != Some('-')) ||
                   (element.chars().nth(0) != None)
                { 
                    result.push(element.to_string()) 
                }
                else { break }
            }
            return result;
        }
    }

    pub fn insert_db(args: &Vec<String>) -> Result<()> {
        Self::get_data(&args);
        println!("Тут типо вставил");
        Ok(())
    }
    pub fn select_db(args: &Vec<String>) -> Result<()> {
        Self::get_data(&args);
        println!("Тут типо запросил");
        Ok(())
    }
}

