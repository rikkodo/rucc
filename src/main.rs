use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Reserved(char),  // 記号
    Number(i32),  // 数値 i32としておく
    Eof, // 入力の終わりを表す
}

// ref :  https://qiita.com/nirasan/items/f7a232af3372ea370f4b
struct Lexer {
    // トークン列
    token: Vec<Token>,
    // トークン位置
    position: usize,
}

impl Lexer {
    fn new(input: &Vec<char>) -> Result<Lexer, String> {
        let tkn = Lexer::tokenize(input)?;
        Ok(Lexer {token: tkn, position: 0})
    }

    fn tokenize(input: &Vec<char>) -> Result<Vec<Token>, String> {
        let mut tkn: Vec<Token> = Vec::new();
        let mut cur: usize = 0;

        while cur < input.len() {
            let c = input.get(cur).unwrap();

            // 空白を読み飛ばす
            if c.is_ascii_whitespace() {
                cur += 1;
                continue;
            }

            let t = if c.is_ascii_digit() {
                // 数値のパース
                let mut tail = cur + 1;
                while tail < input.len() && input.get(tail).unwrap().is_ascii_digit() {
                    tail += 1;
                }
                let v = String::from_iter(&input[cur..tail]);
                let v = match v.parse::<i32>() {
                    Ok(num) => num,
                    Err(e) => return Err(format!("Parse error. {} {:?}", v, e)),
                };
                cur = tail - 1;
                Token::Number(v)
            } else {
                // 記号
                match c {
                    &'+' => Token::Reserved('+'),
                    &'-' => Token::Reserved('-'),
                    _ => return Err(format!("Unsupported token {}", c)),
                }
            };

            tkn.push(t);
            cur += 1;
        }

        tkn.push(Token::Eof);

        return Ok(tkn)
    }

    fn head(& self) -> &Token {
        self.token.get(self.position).unwrap()
    }

    fn deque(&mut self) -> &Token {
        self.position += 1;
        self.token.get(self.position - 1).unwrap()
    }
}

fn run_app() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Too few Argument.".to_owned());
    }

    let mut token = Lexer::new(&args[1].chars().collect())?;

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("    mov rax, {}", match token.deque() {
        Token::Number(v) => v,
        t => return Err(format!("Unexpected Token {:?}", t)),
    });

    while *token.head() != Token::Eof {
        let r = match token.deque() {
            Token::Reserved(r) => r,
            t => return Err(format!("Unexpected Token {:?}", t)),
        };

        let ope = match r {
            '+' => "add",
            '-' => "sub",
            _ => return Err(format!("Unexpected Reserved {}", r)),
        };

        let v = match token.deque() {
            Token::Number(v) => v,
            t => return Err(format!("Unexpected Token {:?}", t)),
        };

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
