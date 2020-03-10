use std::iter::FromIterator;

#[derive(Debug, Clone)]
struct Point {
    line: usize,
    pos: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line: {} pos: {}", self.line, self.pos)
    }
}

#[derive(Debug)]
enum RuccErr {
    ParseErr(String, Point),
    TokenErr(String, Token),
    InsideErr(String),
}

impl std::error::Error for RuccErr {}
impl std::fmt::Display for RuccErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuccErr::ParseErr(s, p) => write!(f, "Parse {} at {}", s, p),
            RuccErr::TokenErr(s, t) => write!(f, "Token {} {} at {}", s, t.get_kind(), t.get_point()),
            RuccErr::InsideErr(s) => write!(f, "Inside {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum TokenKind {
    Plus,  // + 記号
    Minus,  // - 記号
    Integer(i32),  // 数値 i32としておく
    Eof, // 入力の終わりを表す
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TokenKind::Plus => write!(f, "Symbole +"),
            TokenKind::Minus => write!(f, "Symbole -"),
            TokenKind::Integer(v) => write!(f, "Integer {}", v),
            TokenKind::Eof => write!(f, "End of File"),
        }
    }
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    point: Point,
}

impl Token {
    fn new(kind: TokenKind, line: usize, pos: usize) -> Token {
        Token {kind, point: Point{line, pos}}
    }

    fn get_kind(&self) -> &TokenKind {
        &self.kind
    }

    fn get_point(&self) -> &Point {
        &self.point
    }
}

// ref :  https://qiita.com/nirasan/items/f7a232af3372ea370f4b
struct Lexer {
    // トークン列
    token: Vec<Token>,
    // トークン位置
    position: usize,
}

impl Lexer {
    fn new(input: &Vec<char>) -> Result<Lexer, Box<dyn std::error::Error>> {
        let tkn = Lexer::tokenize(input)?;
        Ok(Lexer {token: tkn, position: 0})
    }

    fn tokenize(input: &Vec<char>) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        let mut tkn: Vec<Token> = Vec::new();
        let mut cur: usize = 0;
        let line: usize = 1;

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
                    Err(e) => {
                        let e = RuccErr::ParseErr(format!("Can't Pares {} {}", v, e), Point {line, pos: cur});
                        return Err(Box::new(e))
                    },
                };
                let t = Token::new(TokenKind::Integer(v), line, cur);
                cur = tail - 1;
                t
            } else {
                // 記号
                match *c {
                    '+' => Token::new(TokenKind::Plus, line, cur),
                    '-' => Token::new(TokenKind::Minus, line, cur),
                    _ => {
                        let e = RuccErr::ParseErr(format!("Unexpected {}", c), Point {line, pos: cur});
                        return Err(Box::new(e))
                    },
                }
            };

            tkn.push(t);
            cur += 1;
        }

        tkn.push(Token::new(TokenKind::Eof, line, cur));

        return Ok(tkn)
    }

    fn is_eof(&self) -> bool {
        *self.token.get(self.position).unwrap().get_kind() == TokenKind::Eof
    }

    fn deque(&mut self) -> Result<&Token, Box<dyn std::error::Error>> {
        if self.position < self.token.len() {
            self.position += 1;
            Ok(self.token.get(self.position - 1).unwrap())
        } else {
            let e = RuccErr::InsideErr(
                format!("DequeueErr {} {}", file!(), line!())
            );
            Err(Box::new(e))
        }
    }
}

fn run_app(input: &Vec<char>) -> Result<(), Box<dyn std::error::Error>> {

    let mut token = Lexer::new(input)?;

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let t = token.deque()?;
    let v = match t.get_kind() {
        TokenKind::Integer(v) => v,
        _ => {
            let e = RuccErr::TokenErr("Unexpected".to_owned(), t.clone());
            return Err(Box::new(e))
        },
    };
    println!("    mov rax, {}", v);

    while !token.is_eof() {
        let t = token.deque()?;
        let ope = match t.get_kind() {
            TokenKind::Plus => "add",
            TokenKind::Minus=> "sub",
            _ => {
                let e = RuccErr::TokenErr("Unexpected".to_owned(), t.clone());
                return Err(Box::new(e))
            },
        };

        let t = token.deque()?;
        let v = match t.get_kind() {
            TokenKind::Integer(v) => v,
            _ => {
                let e = RuccErr::TokenErr("Unexpected".to_owned(), t.clone());
                return Err(Box::new(e))
            },
        };

        println!("    {} rax, {}", ope, v);
    }

    println!("    ret");
    Ok(())
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Too few Argument.");
        std::process::exit(1)
    }

    let input = &args[1].chars().collect();

    std::process::exit(match run_app(input) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        },
    });
}
