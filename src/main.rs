fn run_app() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Too few arguments.".to_string());
    }

    let val: u8 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => return Err("Parse error ".to_string() + &args[1]),
    };

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("    mov rax, {}", val);
    println!("    ret");
    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
