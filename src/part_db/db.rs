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
                "-c" => obj_data.column = get_val_from_args(args, index+1),
                "-v" => obj_data.value = get_val_from_args(args, index+1),
                _ => (),
            }
            index += 1;
        }
        return Ok(obj_data);

        fn get_val_from_args(args: &Vec<String>, index: usize) -> Vec<String> {
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
        let data_args = Self::get_data(&args)?;
        let conn = Connection::open(path)?;
        let sql: String = match data_args.target.as_str() {
            "Студент" => {
                format!(
                    "INSERT INTO 'Cтудент' (Номер_зачетки, Имя, Фамилия, Отчество,
                        Дата_рождения, Группа, Адрес),
                    VALUE ({},'{}','{}','{}','{}','{}','{}')",
                    data_args.column[0],//Зачетка
                    data_args.column[1],//И
                    data_args.column[2],//Ф
                    data_args.column[3],//О
                    data_args.column[4],//Дата рожд
                    data_args.column[5],//Группа (сокращ)
                    data_args.column[6],//Адрес
            )},
            "Направление" => {
                format!(
                    "INSERT INTO 'Направление' (Название, Описание,
                        Код_направления, Дата_начала, Дата_окончания, Факультет)
                    SELECT '{}','{}',{},'{}','{}',f.id
                    FROM 'Факультет' f 
                        WHERE f.Наименование={} ",
                    data_args.column[0],//Сокр назв
                    data_args.column[1],//Полное назв
                    data_args.column[2],//Код напр
                    data_args.column[3],//Дата нач
                    data_args.column[4],//Дата оконч
                    data_args.column[5],//Факультет
            )},
            "Посещаемость" => {
                format!(
                    "INSERT INTO 'Посещаемость' (Дата, 
                        Предмет, Студент, Присутствие, Оценка, 
                        Тема_занятия)
                    SELECT '{}', pr.id, st.Номер_зачетки, '{}', {}, pr.id
                    FROM 'Предмет' pr JOIN 'Студент' st 
                        ON pr.Наименование='{}'
                        AND st.Имя='{}'
                        AND s.Фамилия='{}'
                        AND s.Отчество='{}'",
                    data_args.column[0],//Дата
                    data_args.column[1],//Присут
                    data_args.column[2],//Оценка
                    data_args.column[3],//Предмет
                    data_args.column[4],//И
                    data_args.column[5],//Ф
                    data_args.column[6],//О
            )},
            "Ведомость" => {
                format!("
                    INSERT INTO Ведомость (Номер_предмет, 
                        Номер_студент, Оценка, Симестр)
                    SELECT p.id, s.Номер_зачетки, {}, {}
                    FROM Предмет p JOIN Cтудент s 
                        ON p.Наименование='{}' 
                        AND s.Имя='{}' 
                        AND s.Фамилия='{}'
                        AND s.Отчество='{}'",
                    data_args.column[0],//Оценка
                    data_args.column[1],//Симестр
                    data_args.column[2],//Предмет
                    data_args.column[3],//И
                    data_args.column[4],//Ф
                    data_args.column[5],//О
            )},
            "Тема занятия" => {
            //TODO: Предвижу баг, когда два препода ведут один
                format!(
                    "INSERT INTO 'План_обучения' (Предмет, Тема_занятия
                    SELECT pr.id, '{}' FROM 'Предмет' pr 
                        WHERE pr.Наименование='{}'",
                    data_args.column[1],//Тема
                    data_args.column[0],//Предмет
            )},
            "Предмет" => {
                format!(
                    "INSRT INTO 'Предмет' (Наименование, Преподаватель) 
                    SELECT '{}', k.id FROM 'Кадры' k 
                        WHERE k.Имя='{}' 
                        AND k.Фамилия='{}' 
                        AND k.Отчество='{}'
                        AND k.Дожность=6",
                    data_args.column[0],//Назв
                    data_args.column[1],//И
                    data_args.column[2],//Ф
                    data_args.column[3],//О
            )},
            "Факультет" => {
                format!(
                    "INSERT INTO 'Факультет' (Наименование, Адрес)
                    VALUE ('{}', '{}')",
                    data_args.column[0],//Назв
                    data_args.column[1],//Адрес
                )
            },
            _ => return Err("Проблема в target".into()),
        };

        conn.execute(&sql, ())?;
        Ok(())
    }

/*    fn replace_data(&mut self, target: &str, requir_columns: Vec<&str>, 
    conn: &Connection) -> Result<&Self, CustomE> {
        // Используется только для вставки
        // Принимает цель - назв таблицы где искать и список столбцов
        // Возращает измененные входные данные на значения из target

        let mut two_obj = Self::new();
        // Задаются столбцы для поиска
        two_obj.column = requir_columns.into_iter().map(|i| i.to_string()).collect();
        //Достаю данные из таблици target
        let two_query = two_obj.get_sql(target, "-S")?;
        let two_data = two_obj.get_data_from_db(two_query.as_str(), conn)?;
        
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

    fn get_sql(&self, table: &str, key: &str) -> 
    Result<String, CustomE> {
        // Генерирует синтаксис для запроса в БД - простой
        if (key == "-I") && (self.column.len() < 1)
            { return Err("Проблема - нет значения column".into()); }
        if (key == "-I") && (self.value.len() < 1) 
            { return Err("Проблема - нет значения value".into()); }
        if (key == "-I") && (self.value.len() != self.column.len()) 
            { return Err("Проблема - нехватка значений -> value != column".into()); }

        //TODO: КАК ВЫТАСКИВАТЬ ДАННЫЕ ИЗ S и I
        let mut sql = String::new();
        match key {
            "-I" => {
                sql = format!(
                    "INSERT INTO '{}' ({}) VALUES ('{}')",
                    table,
                    self.column.join(", "),
                    self.value.join("', '"),
            )}
            "-S" => {
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
            },
/*            "-R" => {
                //TODO: Рекурсия - 
                sql = format!("INNER JOIN {}")
            },
*/
            _ => return Err("Неверный ключ в get_sql".into()),
        }
        Ok(sql)
    }
*/

    pub fn select_db(args: &Vec<String>, path: &String) -> Result<(), CustomE> {
        let mut data_args = Self::get_data(&args)?;
        let conn = Connection::open(path)?;
        let sql: String = match data_args.target.as_str() {
            "Студент" => {
                data_args.column.resize(7, "".to_string());
                "SELECT coalesce(Номер_зачетки, 0, Фамилия, 0, Имя, 0, Отчество, 0, 
                    Дата_рождения, 0, Группа, 0, Адрес, 0) Номер_зачетки,Фамилия,Имя, 
                    Отчество,Дата_рождения,Группа,Адрес
                FROM 'Cтудент'".to_string()
            },
            "Направление" => {
                data_args.column.resize(6, "".to_string());
                "SELECT coalesce(n.Название, 0, n.Описание, 0, n.Код_направления, 0, n.Дата_начала, 
                    0, n.Дата_окончания, 0, f.Наименование, 0)Название,Описание,Код_направления,
                    Дата_начала,Дата_окончания,Наименование
                FROM 'Направление' n 
                INNER JOIN 'Факультет' f ON n.Факультет=f.id
                GROUP BY Название".to_string()
            },
            "Посещаемость" => {
                data_args.column.resize(8, "".to_string());
                "SELECT coalesce(pr.Наименование, 0, pl.Тема_занятия, 0, s.Фамилия,0,  
                    s.Имя, 0, s.Отчество, 0, po.Дата, 0, po.Присутствие, 0, po.Оценка, 0)
                    Наименование,pl.Тема_занятия,Фамилия,Имя,Отчество,Дата,Присутствие,Оценка
                FROM 'Посещаемость' po
                INNER JOIN 'Cтудент' s ON s.Номер_зачетки=po.Студент
                INNER JOIN 'Предмет' pr ON pr.id=po.Предмет
                INNER JOIN 'План_обучения' pl ON pl.id=po.Тема_занятия
                GROUP BY Дата".to_string()
            },
            "Ведомость" => {
                data_args.column.resize(6, "".to_string());
                "SELECT coalesce(s.Фамилия, 0, s.Имя, 0, s.Отчество, 0, 
                    p.Наименование, 0, v.Симестр, 0, v.Оценка, 0)Фамилия,
                    Имя,Отчество,Наименование,Симестр,Оценка
                FROM 'Ведомость' v
                INNER JOIN 'Cтудент' s ON s.Номер_зачетки=v.Номер_студент
                INNER JOIN 'Предмет' p ON p.id=v.Номер_предмет
                GROUP BY Симестр, Наименование".to_string()
            },
            "Тема занятия" => {
                data_args.column.resize(2, "".to_string());
                "SELECT Предмет, Тема_занятия 
                FROM 'План_обучения' 
                GROUP BY Предмет".to_string()
            },
/*            "Предмет" => {
                // Препода (потом)
                data_args.get_sql("", "-S")?;
                "".to_string()
            },
*/
            "Факультет" => {
                data_args.column.resize(3, "".to_string());
                "SELECT coalesce(id, Наименование, 0, Адрес, 0) 
                    id, Наименование, Адрес
                FROM 'Факультет'
                GROUP BY id".to_string()
            }
            _ => return Err("Проблема в target".into()),
        };
        
        let result = data_args.get_data_from_db(&sql, &conn)?;
        for part in result {
            println!("{}", part.join(" | "));
        }
        Ok(())
    }

    fn get_data_from_db(&self, sql: &str, conn: &Connection) -> 
    Result<Vec<Vec<String>>, CustomE> {
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

