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
            RuccErr::ParseErr(s, p) => write!(f, "{} at {}", s, p),
            RuccErr::TokenErr(s, t) => write!(f, "{} {} at {}", s, t.get_kind(), t.get_point()),
            RuccErr::InsideErr(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum TokenKind {
    Reserved(&'static str),  // 予約語
    Integer(i32),  // 数値 i32としておく
    Eof, // 入力の終わりを表す
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TokenKind::Reserved(s) => write!(f, "symbole {}", s),
            TokenKind::Integer(v) => write!(f, "integer {}", v),
            TokenKind::Eof => write!(f, "end of file"),
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
    fn new(input: &Vec<char>) -> Result<Lexer,RuccErr> {
        let tkn = Lexer::tokenize(input)?;
        Ok(Lexer {token: tkn, position: 0})
    }

    fn tokenize(input: &Vec<char>) -> Result<Vec<Token>, RuccErr> {
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
                let v = &input[cur..tail].into_iter().collect::<String>();
                let v = match v.parse::<i32>() {
                    Ok(num) => num,
                    Err(e) => {
                        return  Err(RuccErr::ParseErr(
                            format!("can't pares {} {}", v, e), Point {line, pos: cur}
                        ))
                    },
                };
                let t = Token::new(TokenKind::Integer(v), line, cur);
                cur = tail - 1;
                t
            } else {
                // 記号
                match *c {
                    '+' => Token::new(TokenKind::Reserved("+"), line, cur),
                    '-' => Token::new(TokenKind::Reserved("-"), line, cur),
                    '*' => Token::new(TokenKind::Reserved("*"), line, cur),
                    '/' => Token::new(TokenKind::Reserved("/"), line, cur),
                    _ => {
                        return Err(RuccErr::ParseErr(
                            format!("unexpected {}", c), Point {line, pos: cur})
                        );
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

    fn consume(&mut self, t: &str) -> Result<bool, RuccErr> {
        let h = self.head()?;
        if let TokenKind::Reserved(v) = h.kind {
            if v == t {
                self.position += 1;
                return Ok(true)
            }
        }

        Ok(false)
    }

    fn expect(&mut self, t: &str) -> Result<(), RuccErr> {
        if self.consume(t)? {
            return Ok(())
        }
        Err(RuccErr::TokenErr("unexpected".to_owned(), self.head()?.clone()))
    }

    fn expect_number(&mut self) -> Result<i32, RuccErr> {
        let h = self.head()?;
        match h.kind {
            TokenKind::Integer(v) => {
                self.position += 1;
                Ok(v)
            },
            _ => {
                Err(RuccErr::TokenErr("unexpected".to_owned(), h.clone()))
            },
        }
    }

    fn head(&self) -> Result<&Token, RuccErr> {
        if self.position < self.token.len() {
            Ok(self.token.get(self.position).unwrap())
        } else {
            Err(RuccErr::InsideErr(format!("inside {} {}", file!(), line!())))
        }
    }
}

fn run_app(input: &Vec<char>) -> Result<(), RuccErr> {

    let mut token = Lexer::new(input)?;

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let v = token.expect_number()?;
    println!("    mov rax, {}", v);

    while !token.is_eof() {
        let ope = if token.consume("+")? {
            "add"
        } else {
            token.expect("-")?;
            "sub"
        };

        let v = token.expect_number()?;
        println!("    {} rax, {}", ope, v);
    }

    println!("    ret");
    Ok(())
}


fn errorat(input: &Vec<char>, p: &Point)
{
    eprintln!("{}", input.into_iter().collect::<String>());
    let mut sp = String::new();
    for _ in 0..p.pos {
        sp = sp + " ";
    }
    eprintln!("{}^", sp);
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
            match &err {
                RuccErr::ParseErr(_, p) => errorat(input, &p),
                RuccErr::TokenErr(_, t) => errorat(input, &t.point),
                _ => ()
            }
            eprintln!("Error: {}", err);
            1
        },
    });
}
