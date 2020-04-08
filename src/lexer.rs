use super::ruccerr;

#[derive(Debug, Clone)]
pub struct Point {
    line: usize,
    pos: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line: {} pos: {}", self.line, self.pos)
    }
}

impl Point {
    pub fn get_pos(&self) -> usize {
        self.pos
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
pub struct Token {
    kind: TokenKind,
    point: Point,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at {}", self.kind, self.point)
    }
}

impl Token {
    fn new(kind: TokenKind, line: usize, pos: usize) -> Token {
        Token {kind, point: Point{line, pos}}
    }
    pub fn get_point(&self) -> &Point {
        &self.point
    }
}

// ref :  https://qiita.com/nirasan/items/f7a232af3372ea370f4b
#[derive(Debug)]
pub struct Lexer {
    // トークン列
    token: Vec<Token>,
    // トークン位置
    position: usize,
}

impl Lexer {
    pub fn new(input: &String) -> Result<Lexer,ruccerr::RuccErr> {
        let tkn = Lexer::tokenize(input)?;
        Ok(Lexer {token: tkn, position: 0})
    }

    fn tokenize(input: &String) -> Result<Vec<Token>, ruccerr::RuccErr> {
        let mut tkn: Vec<Token> = Vec::new();
        let mut cur: usize = 0;
        let input: Vec<char> = input.chars().collect();
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
                        return  Err(ruccerr::RuccErr::ParseErr(
                            format!("can't pares {} {}", v, e), Point {line, pos: cur}
                        ))
                    },
                };
                let t = Token::new(TokenKind::Integer(v), line, cur);
                cur = tail;
                t
            } else {
                // 記号
                Lexer::parse_reserved(&input[cur..], line, &mut cur)?
            };

            tkn.push(t);
        }

        tkn.push(Token::new(TokenKind::Eof, line, cur));

        return Ok(tkn)
    }

    fn parse_reserved(input: &[char], line: usize, cur: &mut usize) -> Result<Token, ruccerr::RuccErr> {
        let is: String = input.into_iter().collect();
        for ts in ["==", "!=", "<=", ">=", "+", "-", "*", "/", "(", ")", "<", ">"].iter() {
            if is.starts_with(ts) {
                let ret = Token::new(TokenKind::Reserved(ts), line, *cur);
                *cur += ts.len();
                return Ok(ret)
            }
        }
        return Err(ruccerr::RuccErr::ParseErr(
            format!("unexpected {}", input.get(0).unwrap()), Point {line, pos: *cur})
        );
    }

    pub fn consume_reserved(&mut self, t: &str) -> Result<bool, ruccerr::RuccErr> {
        if let TokenKind::Reserved(v) = self.head()?.kind {
            if v == t {
                self.position += 1;
                return Ok(true)
            }
        }
        Ok(false)
    }

    pub fn expect_reserved(&mut self, t: &str) -> Result<(), ruccerr::RuccErr> {
        if self.consume_reserved(t)? {
            return Ok(())
        }
        Err(ruccerr::RuccErr::TokenErr("unexpected".to_owned(), self.head()?.clone()))
    }

    pub fn expect_number(&mut self) -> Result<i32, ruccerr::RuccErr> {
        if let TokenKind::Integer(v) = self.head()?.kind {
            self.position += 1;
            return Ok(v)
        }
        Err(ruccerr::RuccErr::TokenErr("unexpected".to_owned(), self.head()?.clone()))
    }

    fn head(&self) -> Result<&Token, ruccerr::RuccErr> {
        if self.position < self.token.len() {
            Ok(self.token.get(self.position).unwrap())
        } else {
            Err(ruccerr::RuccErr::InsideErr(format!("inside {} {}", file!(), line!())))
        }
    }
}