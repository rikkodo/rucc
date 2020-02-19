fn torknize(l: &String) -> Result<Vec<String>, String> {
    /* strtol がないので無理やりそれっぽいものを作る */
    // let l: Vec<char> = l.chars().collect();
    let mut lex: Vec<String> = Vec::new();
    let mut cur: usize = 0;
    let mut head: usize = 0;

    while cur < l.len() {
        let c = l.chars().nth(cur).unwrap();
        if c.is_ascii_digit() {
            /* 何もしない */
        } else if c == '+' || c == '-' {
            if head != cur {
                lex.push(l[head..cur].to_string())
            }
            lex.push(c.to_string());
            head = cur + 1;
        } else {
            return Err("Unsupported token".to_owned());
        }
        cur += 1;
    }

    if head != cur {
        lex.push(l[head..cur].to_string())
    }

    return Ok(lex);

}

fn run_app() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Too few Argument.".to_owned());
    }

    let lex = torknize(&args[1])?;

    if lex.len() <= 0 {
        return Err("Format Err".to_owned());
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let v: i32 = match lex[0].parse() {
        Ok(num) => num,
        Err(_) => return Err(format!("Parse Error {}", lex[0]).to_owned()),
    };
    println!("    mov rax, {}", v);
    let mut i: usize = 1;
    loop {
        if lex.len() <= i {
            break;
        }
        let ope = match lex[i].as_str() {
            "+" => "add",
            "-" => "sub",
            _ => return Err(format!("Unexpected char {}", lex[i])),
        };
        i += 1;

        if lex.len() <= i {
            return Err("Format Err".to_owned());
        }

        let v: i32 = match lex[i].parse() {
            Ok(num) => num,
            Err(_) => return Err(format!("Parse Error {}", lex[i]).to_owned()),
        };
        i += 1;

        println!("    {} rax, {}", ope, v);
    }
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
