#[allow(unused_imports)] // ЭТО ВРЕМЕННО!
use rusqlite::{Connection, Result}; 
use crate::part_db::error::CustomE;

#[derive(Debug)] // ЭТО ВРЕМЕННО!
pub struct DataForDB {
    target: String,
    column: Vec<String>,
    value: Vec<String>,
}

impl DataForDB {
    pub fn new() -> Self {
        Self {
            target: String::new(),
            column: Vec::<String>::new(),
            value: Vec::<String>::new(),
        }
    }

    fn get_data(args: &Vec<String>) -> Result<Self, CustomE> {
        // Вылуживает данные из строки запуска
        let mut obj_data = Self::new();
        let mut index = 2;
        //TODO: Найти и решить проблему - нет данных у -с и -v
        while index < args.len() {
            match args[index].as_str() {
                "-t" => obj_data.target = args[index+1].clone(),
                "-c" => obj_data.column = get_value_args(args, index+1),
                "-v" => obj_data.value = get_value_args(args, index+1),
                _ => index += 1,
            }
            index += 1;
        }
        return Ok(obj_data);

        fn get_value_args(args: &Vec<String>, index: usize) -> Vec<String> {
            // Оно принимает строку запуска + индекс от куда начать
            let mut result = Vec::<String>::new();
            for element in &args[index..] {
                // Тут чекаю является ли элемент -> ключом запуска
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

    pub fn insert_db(args: &Vec<String>) -> Result<(), CustomE> {
        let data = Self::get_data(&args)?;
        println!("\
            Тут типо вставил\n\
            Данные из структуры\n\
            {:?}", data
        );
        Ok(())
    }

    pub fn select_db(args: &Vec<String>) -> Result<()> {
        //let data = Self::get_data(&args);
        println!("Тут типо запросил");
        Ok(())
    }
}

