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
        let mut data_args = Self::get_data(&args)?;
        let conn = Connection::open(path)?;
        let sql: String = match data_args.target.as_str() {
            "Студент" => data_args.query_generic("Cтудент", "-I")?,
            "Направление" => {
                data_args.replace_data("Факультет", vec!["id","Наименование"], &conn)?
                    .query_generic("Направления", "-I")?
            },
            "Посещаемость" => {
                // Особенность при встаке надо использовать "ФИО"
                // Внешние ключи: Студент, Предмет
                // Ввод: -c Дата,\Предмет,\Студент,Присут,Оценка,Тема -v
                data_args.replace_data(
                    "Предмет", vec!["id", "Наименование"], &conn)?;
                //TODO: Проблема - Имя и Отчество (проще изменить БД)
                data_args.replace_data(
                    "Студент", vec!["Номер_зачетки", "Фамилия"], &conn)?;
                data_args.query_generic("Посещаемость", "-I")?
            },
            "Ведомость" => {
                data_args.query_generic("", "-I")?;
                "".to_string()
            },
            "Тема занятия" => {
                data_args.query_generic("", "-I")?;
                "".to_string()
            },
            "Предмет" => {
                data_args.query_generic("", "-I")?;
                "".to_string()
            },
            "Факультет" => data_args.query_generic("Факультет", "-I")?,
            _ => return Err("Проблема в target".into()),
        };

        conn.execute(&sql, ())?;
        Ok(())
    }

    fn replace_data(&mut self, target: &str, requir_columns: Vec<&str>, 
    conn: &Connection) -> Result<&Self, CustomE> {
        // Принимает цель - назв таблицы где искать и список столбцов
        // Возращает измененные входные данные на значения из target

        let mut two_obj = Self::new();
        // Задаются столбцы для поиска
        two_obj.column = requir_columns.into_iter().map(|i| i.to_string()).collect();
        //Достаю данные из таблици target
        let two_query = two_obj.query_generic(target, "-S")?;
        let two_data = two_obj.get_data_db(two_query.as_str(), conn)?;
        
        // Тут я ищу index target из строки вводной
        let mut id_elem_args = None;
        for index in 0..self.column.len() {
            if self.column[index] == target {
                id_elem_args = Some(index);
                break;
            }
        }
            
        if id_elem_args == None {
            return Err(
                format!("Нет искомого значения в args -> {}", target).into()
        )}

        // Заменяю значение в вводной строке
        for two_part in two_data {
            if self.value[id_elem_args.unwrap()] == two_part[1] {
                self.value[id_elem_args.unwrap()] = two_part[0].clone();
                break;
            }
        }
        Ok(self)
    }

    fn query_generic(&self, table: &str, key: &str) 
    -> Result<String, CustomE> {
        if (key == "-I") && (self.column.len() < 1)
            { return Err("Проблема - нет значения column".into()); }
        if (key == "-I") && (self.value.len() < 1) 
            { return Err("Проблема - нет значения value".into()); }
        if (key == "-I") && (self.value.len() != self.column.len()) 
            { return Err("Проблема - нехватка значений -> value != column".into()); }

        //TODO: КАК ВЫТАСКИВАТЬ ДАННЫЕ ИЗ S и I
        let mut sql = String::new();
        if key == "-I" {
            sql = format!(
                "INSERT INTO '{}' ({}) VALUES ('{}');",
                table,
                self.column.join(", "),
                self.value.join("', '"),
        )}
        else if key == "-S" {
            if self.column.len() < 1 {
                sql = format!(
                    "SELECT * FROM '{}'",
                    table,
            )} else {
                sql = format!(
                    // 0 - будет значением для избежания ошибок
                    "SELECT coalesce({}, 0){} FROM '{}'",
                    self.column.join(", 0, "),
                    self.column.join(", "),
                    table,
            )}
        }
        Ok(sql)
    }

    pub fn select_db(args: &Vec<String>, path: &String) -> Result<(), CustomE> {
        let data_args = Self::get_data(&args)?;
        let conn = Connection::open(path)?;
        let sql: String = match data_args.target.as_str() {
            "Студент" => data_args.query_generic("Cтудент", "-S")?,
            "Направление" => {
                data_args.query_generic("", "-S")?;
                "".to_string()
            },
            "Посещаемость" => {
                data_args.query_generic("", "-S")?;
                "".to_string()
            },
            "Ведомость" => {
                data_args.query_generic("", "-S")?;
                "".to_string()
            },
            "Тема занятия" => {
                data_args.query_generic("", "-S")?;
                "".to_string()
            },
            "Предмет" => {
                data_args.query_generic("", "-S")?;
                "".to_string()
            },
            "Факультет" => data_args.query_generic("Факультет", "-S")?,
            _ => return Err("Проблема в target".into()),
        };

        let result = data_args.get_data_db(&sql, &conn)?;
        for part in result {
            println!("{:?}", part);
        }
        Ok(())
    }

    fn get_data_db(&self, sql: &str, conn: &Connection) -> Result<Vec<Vec<String>>, CustomE> {
        //TODO: Пока не рализовано для случая '*'
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(
            [], 
            |row| {
                let mut index = 0;
                let mut res_vec = Vec::<String>::new();
                while index < self.column.len() {
                    if let Some(res_temp) = row.get::<_,String>(index).ok() {
                        res_vec.push(res_temp);
                    } else if let Some(res_temp) = row.get::<_,i32>(index).ok() {
                        res_vec.push(res_temp.to_string());
                    }
                    index += 1;
                }
                Ok(res_vec)
        })?;
        // Конвертирую в нормальные данные
        let mut result = Vec::new();
        for part in rows {
            result.push(part?)
        }
        Ok(result)
    }
}

