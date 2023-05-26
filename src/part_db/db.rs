use rusqlite::{Connection, Result}; 
use crate::part_db::error::CustomE;

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
        let mut index: usize = 2;
        while index < args.len() {
            match args[index].as_str() {
                "-t" => obj_data.target = args[index+1].clone(),
                "-c" => obj_data.column = get_value_args(args, index+1),
                "-v" => obj_data.value = get_value_args(args, index+1),
                _ => (),
            }
            index += 1;
        }
        return Ok(obj_data);

        fn get_value_args(args: &Vec<String>, index: usize) -> Vec<String> {
            // Оно принимает строку запуска + индекс от куда начать
            let mut result = Vec::<String>::new();
            for element in &args[index..] {
                // Тут чекаю является ли элемент -> ключом запуска
                if (element.chars().nth(0) != Some('-')) &&
                   (element.chars().nth(0) != None)
                { 
                    result.push(element.to_string()) 
                }
                else { break }
            }
            return result;
        }
    }

    pub fn insert_db(args: &Vec<String>, path: &String) -> Result<(), CustomE> {
        let data = Self::get_data(&args)?;
        let conn = Connection::open(path)?;

        match data.target.as_str() {
            "Студент" => {
                //TODO: Подумать как сделать
                conn.prepare(
                    "INSERT INTO 'Cтуденты' (?1) 
                    VALUES (?2)", [data.column, data.value],
                )?;
            },
            "Направление" => {
                conn.prepare("INSERT INTO 'Направления' () VALUES ()")?;
            },
            "Посещаемость" => {
                conn.prepare("INSERT INTO 'Посещаемость' () VALUES ()")?;
            },
            "Ведомость" => {
                conn.prepare("INSERT INTO 'Ведомость' () VALUES ()")?;
            },
            "Тема занятия" => {
                conn.prepare("INSERT INTO 'Тема_занятия' () VALUES ()")?;
            },
            "Предмет" => {
                conn.prepare("INSERT INTO 'Предметы' () VALUES ()")?;
            },
            "Факультет" => {
                conn.prepare("INSERT INTO 'Факультет' () VALUES ()")?;
            },
            _ => return Err("Проблема в target".into()),
        }

        Ok(())
    }

    pub fn select_db(args: &Vec<String>, path: &String) -> Result<(), CustomE> {
        let data = Self::get_data(&args)?;
        let conn = Connection::open(path)?;

        match data.target.as_str() {
            "Студент" => {
            },
            "Направление" => {
            },
            "Посещаемость" => {
            },
            "Ведомость" => {
            },
            "Тема занятия" => {
            },
            "Предмет" => {
            },
            "Факультет" => {
            },
            _ => return Err("Проблема в target".into()),
        }

        Ok(())
    }
}

