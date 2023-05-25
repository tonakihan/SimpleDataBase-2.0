//use std::error::Error; //TODO:Для impl CustomError, но потом
use std::fmt; //Для ipml CustomError


// Создал свой тип ошибки. Т.к. я хочу свой 
// текст и rusqlite::Error в одном.
#[derive(Debug)]
pub struct CustomE {
    message: String,
}

impl fmt::Display for CustomE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Err: {}", self.message)
    }
}

//TODO:Классная штука, но потом.
//impl Error for CustomError {} 

// Вот эти штуки для того, чтобы писать сообщение сразу, 
// а не возвращать объект
impl From<rusqlite::Error> for CustomE { 
    fn from(err: rusqlite::Error) -> Self {
        Self { message: err.to_string() }
    }
}
impl From<&str> for CustomE {
    fn from(msg: &str) -> Self {
        Self { message: msg.to_owned() }
    }
}
impl From<String> for CustomE {
    fn from(msg: String) -> Self {
        Self { message: msg }
    }
}

