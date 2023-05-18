
fn main() {
    use std::process::exit;
    use std::env;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not found arguments");
        exit(1);
    }

    //Думаю над индексами
    if let Some(index) = args.iter().position(|arg| arg == "-m") {
        if index + 1 < args.len() {
            println!("Text: {}", args[index + 1]);
        } else {
            println!("No text provided after -m");
        }
    }
}
