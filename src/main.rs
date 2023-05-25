#![allow(non_snake_case)]

mod part_db;

// Включаем в область видимости структуры
use crate::part_db::error::CustomE;
use crate::part_db::db::DataForDB;
//use std::num::ParseIntError; //TODO:Возможно оно не надо тут


fn main() {
    use std::process::exit;
    use std::env;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not found arguments");
        exit(1);
    }
    set_mode(&args)
        .expect("Проблемы из set_mode");
}


fn set_mode(args: &Vec<String>) -> Result<(), CustomE> {
    match args[1].as_str() {
        "-I" => DataForDB::insert_db(&args)?,
        "-S" => DataForDB::select_db(&args)?,
        _ => return Err("Not found arguments".into()),
    }
    Ok(())
}

