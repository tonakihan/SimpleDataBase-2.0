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
                        Дата_рождения, Группа, Адрес)
                    VALUES ({},'{}','{}','{}','{}','{}','{}')",
                    data_args.value[0], //Зачетка
                    data_args.value[2], //Ф
                    data_args.value[1], //И
                    data_args.value[3], //О
                    data_args.value[4], //Дата рожд
                    data_args.value[5], //Группа (сокращ)
                    data_args.value[6], //Адрес
            )},
            "Направление" => {
                format!(
                    "INSERT INTO 'Направление' (Название, Описание,
                        Код_направления, Дата_начала, Дата_окончания, Факультет)
                    SELECT '{}','{}',{},'{}','{}',f.id
                    FROM 'Факультет' f 
                        WHERE f.Наименование='{}'",
                    data_args.value[0],//Сокр назв
                    data_args.value[1],//Полное назв
                    data_args.value[2],//Код напр
                    data_args.value[3],//Дата нач
                    data_args.value[4],//Дата оконч
                    data_args.value[5],//Факультет
            )},
            "Посещаемость" => {
                format!(
                    "INSERT INTO 'Посещаемость' (Дата, 
                        Предмет, Студент, Присутствие, Оценка, 
                        Тема_занятия)
                    SELECT '{}', pr.id, st.Номер_зачетки, '{}', {}, pl.id
                    FROM 'Предмет' pr JOIN 'Cтудент' st JOIN 'План_обучения' pl
                        ON pr.Наименование='{}'
                        AND st.Имя='{}'
                        AND st.Фамилия='{}'
                        AND st.Отчество='{}'
                        AND pl.Тема_занятия='{}'",
                    data_args.value[0],//Дата
                    data_args.value[1],//Присут
                    data_args.value[2],//Оценка
                    data_args.value[3],//Предмет
                    data_args.value[5],//Ф
                    data_args.value[4],//И
                    data_args.value[6],//О
                    data_args.value[7],//Тема_занятия
            )},
            "Ведомость" => {
                format!("
                    INSERT INTO 'Ведомость' (Номер_предмет, 
                        Номер_студент, Оценка, Симестр)
                    SELECT p.id, s.Номер_зачетки, {}, {}
                    FROM 'Предмет' p JOIN 'Cтудент' s 
                        ON p.Наименование='{}' 
                        AND s.Имя='{}' 
                        AND s.Фамилия='{}'
                        AND s.Отчество='{}'",
                    data_args.value[0],//Оценка
                    data_args.value[1],//Симестр
                    data_args.value[2],//Предмет
                    data_args.value[4],//Ф
                    data_args.value[3],//И
                    data_args.value[5],//О
            )},
            "Тема занятия" => {
            //TODO: Предвижу баг, когда два препода ведут один
                format!(
                    "INSERT INTO 'План_обучения' (Предмет, Тема_занятия)
                    SELECT pr.id, '{}' FROM 'Предмет' pr 
                        WHERE pr.Наименование='{}'",
                    data_args.value[0],//Тема
                    data_args.value[1],//Предмет
            )},
            "Предмет" => {
                format!(
                    "INSERT INTO 'Предмет' (Наименование, Преподаватель) 
                    SELECT '{}', k.id FROM 'Кадры' k 
                        WHERE k.Имя='{}' 
                        AND k.Фамилия='{}' 
                        AND k.Отчество='{}'
                        AND k.Должность=6",
                    data_args.value[0],//Назв
                    data_args.value[2],//Ф
                    data_args.value[1],//И
                    data_args.value[3],//О
            )},
            "Факультет" => {
                format!(
                    "INSERT INTO 'Факультет' (Наименование, Адрес)
                    VALUES ('{}', '{}')",
                    data_args.value[0],//Назв
                    data_args.value[1],//Адрес
                )
            },
            _ => return Err("Проблема в target".into()),
        };

        let mut stmt = conn.prepare(&sql)?;
        stmt.insert(())?;
        Ok(())
    }

    pub fn select_db(args: &Vec<String>, path: &String) -> Result<(), CustomE> {
        let mut data_args = Self::get_data(&args)?;
        let conn = Connection::open(path)?;
        let sql: String = match data_args.target.as_str() {
            "Студент" => {
                data_args.column.resize(7, "".to_string());
                "SELECT coalesce(Номер_зачетки, 0, Фамилия, 0, Имя, 0, Отчество, 0, 
                    Дата_рождения, 0, Группа, 0, Адрес, 0) Номер_зачетки,Фамилия,Имя, 
                    Отчество,Дата_рождения,Группа,Адрес
                FROM 'Cтудент'
                GROUP BY Номер_зачетки".to_string()
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
            "Предмет" => {
                data_args.column.resize(4, "".to_string());
                "SELECT p.Наименование, k.Фамилия, k.Имя, k.Отчество 
                FROM 'Предмет' p INNER JOIN 'Кадры' k
                    ON k.id=p.Преподаватель".to_string()
            },
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

