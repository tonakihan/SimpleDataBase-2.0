#![allow(non_snake_case)]

mod part_db;

// Включаем в область видимости структуры
use self::part_db::error::CustomE;
use self::part_db::db::DataForDB;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    use std::process::exit;
    use std::env;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not found arguments");
        exit(1);
    }

    let path_to_db = "data/test.db".to_string();
    set_mode(&args, &path_to_db)
        .expect("Проблемы из set_mode");
}


fn set_mode(args: &Vec<String>, path_db: &String) -> Result<(), CustomE> {
    match args[1].as_str() {
        "-I" => DataForDB::insert_db(&args, &path_db)?,
        "-S" => DataForDB::select_db(&args, &path_db)?,
        "-V" | "--version" => println!("Version: {}", VERSION.unwrap_or("unknown")),
        _ => return Err("Not found arguments".into()),
    }
    Ok(())
}

