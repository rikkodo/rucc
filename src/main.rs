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
#[derive(Debug)]
struct Lexer {
    // トークン列
    token: Vec<Token>,
    // トークン位置
    position: usize,
}

impl Lexer {
    fn new(input: &String) -> Result<Lexer,RuccErr> {
        let tkn = Lexer::tokenize(input)?;
        Ok(Lexer {token: tkn, position: 0})
    }

    fn tokenize(input: &String) -> Result<Vec<Token>, RuccErr> {
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
                        return  Err(RuccErr::ParseErr(
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

    fn parse_reserved(input: &[char], line: usize, cur: &mut usize) -> Result<Token, RuccErr> {
        let is: String = input.into_iter().collect();
        for ts in ["==", "!=", "<=", ">=", "+", "-", "*", "/", "(", ")", "<", ">"].iter() {
            if is.starts_with(ts) {
                let ret = Token::new(TokenKind::Reserved(ts), line, *cur);
                *cur += ts.len();
                return Ok(ret)
            }
        }
        return Err(RuccErr::ParseErr(
            format!("unexpected {}", input.get(0).unwrap()), Point {line, pos: *cur})
        );
    }

    fn consume_reserved(&mut self, t: &str) -> Result<bool, RuccErr> {
        if let TokenKind::Reserved(v) = self.head()?.kind {
            if v == t {
                self.position += 1;
                return Ok(true)
            }
        }
        Ok(false)
    }

    fn expect_reserved(&mut self, t: &str) -> Result<(), RuccErr> {
        if self.consume_reserved(t)? {
            return Ok(())
        }
        Err(RuccErr::TokenErr("unexpected".to_owned(), self.head()?.clone()))
    }

    fn expect_number(&mut self) -> Result<i32, RuccErr> {
        if let TokenKind::Integer(v) = self.head()?.kind {
            self.position += 1;
            return Ok(v)
        }
        Err(RuccErr::TokenErr("unexpected".to_owned(), self.head()?.clone()))
    }

    fn head(&self) -> Result<&Token, RuccErr> {
        if self.position < self.token.len() {
            Ok(self.token.get(self.position).unwrap())
        } else {
            Err(RuccErr::InsideErr(format!("inside {} {}", file!(), line!())))
        }
    }
}

/* 二分木 */
#[derive(Debug)]
enum BinTree<T> {
    Nil,
    Node{
        val: T,
        left: Box<BinTree<T>>,
        right: Box<BinTree<T>>,
    },
}
impl<T> BinTree<T> {
    fn postorder(&self, act: &dyn Fn(&T))  {
        BinTree::<T>::postorder_in(self, act)
    }

    fn postorder_in(t: &BinTree<T>, act: &dyn Fn(&T)){
        match t {
            BinTree::<T>::Nil => (),
            BinTree::<T>::Node{val, left, right} => {
                BinTree::<T>::postorder_in(&*left, act);
                BinTree::<T>::postorder_in(&*right, act);
                act(val)
            }
        }
    }
}


// 抽象構文木
// オペレータ
#[derive(Debug)]
enum NodeOperand {
    Integer(i32),  // 数値 i32としておく
}

// 二項オペランド
#[derive(Debug)]
enum NodeBinOperator {
    Plus,  // + 記号
    Minus,  // - 記号
    Mul,  // * 記号
    Div,  // / 記号
    Eq, // ==
    Neq, // !=
    LessThan, // <
    LessEq, // <=
}

#[derive(Debug)]
enum NodeKind {
    Operand(NodeOperand),
    BinOperator(NodeBinOperator),
}

#[derive(Debug)]
struct NodeTree {
}

impl NodeTree {
    fn parse(lex: &mut Lexer) -> Result<BinTree::<NodeKind>, RuccErr> {
        NodeTree::expr(lex)
    }

    fn expr(lex: &mut Lexer) -> Result<BinTree::<NodeKind>, RuccErr> {
        NodeTree::equality(lex)
    }

    fn equality(lex: &mut Lexer) -> Result<BinTree::<NodeKind>, RuccErr> {
        let mut l = NodeTree::relational(lex)?;
        loop {
            if lex.consume_reserved("==")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::Eq), l, NodeTree::relational(lex)?)
            } else if lex.consume_reserved("!=")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::Neq), l, NodeTree::relational(lex)?)
            } else {
                return Ok(l)
            }
        }
    }

    fn relational(lex: &mut Lexer) -> Result<BinTree::<NodeKind>, RuccErr> {
        let mut l = NodeTree::add(lex)?;
        loop {
            if lex.consume_reserved("<")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::LessThan), l, NodeTree::add(lex)?)
            } else if lex.consume_reserved("<=")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::LessEq), l, NodeTree::add(lex)?)
            } else if lex.consume_reserved(">")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::LessThan), NodeTree::add(lex)?, l)
            } else if lex.consume_reserved(">=")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::LessEq), NodeTree::add(lex)?, l)
            } else {
                return Ok(l)
            }
        }
    }

    fn add(lex: &mut Lexer) -> Result<BinTree::<NodeKind>, RuccErr> {
        let mut l = NodeTree::mul(lex)?;
        loop {
            if lex.consume_reserved("+")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::Plus), l, NodeTree::mul(lex)?)
            } else if lex.consume_reserved("-")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::Minus), l, NodeTree::mul(lex)?)
            } else {
                return Ok(l)
            }
        }
    }

    fn mul(lex: &mut Lexer) -> Result<BinTree::<NodeKind>, RuccErr> {
        let mut l = NodeTree::unary(lex)?;
        loop {
            if lex.consume_reserved("*")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::Mul), l, NodeTree::unary(lex)?)
            } else if lex.consume_reserved("/")? {
                l = NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::Div), l, NodeTree::unary(lex)?)
            } else {
                return Ok(l)
            }
        }
    }

    fn unary(lex: &mut Lexer) -> Result<BinTree::<NodeKind>, RuccErr> {
        if lex.consume_reserved("+")? {
            Ok(NodeTree::unary(lex)?)
        } else if lex.consume_reserved("-")? {
            Ok(NodeTree::new_node(NodeKind::BinOperator(NodeBinOperator::Minus), NodeTree::new_node_num(0), NodeTree::unary(lex)?))
        } else {
            Ok(NodeTree::primary(lex)?)
        }
    }

    fn primary(lex: &mut Lexer) -> Result<BinTree::<NodeKind>, RuccErr> {
        if lex.consume_reserved("(")? {
            let l = NodeTree::expr(lex)?;
            lex.expect_reserved(")")?;
            Ok(l)
        } else {
            Ok(NodeTree::new_node_num(lex.expect_number()?))
        }
    }

    fn new_node_num(val: i32) -> BinTree::<NodeKind> {
        BinTree::<NodeKind>::Node {
            val: NodeKind::Operand(NodeOperand::Integer(val)),
            left: Box::new(BinTree::<NodeKind>::Nil),
            right: Box::new(BinTree::<NodeKind>::Nil),
        }
    }

    fn new_node(k: NodeKind, l: BinTree::<NodeKind>, r: BinTree::<NodeKind>) -> BinTree::<NodeKind> {
        BinTree::<NodeKind>::Node {
            val: k,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    fn gencode(t: &BinTree::<NodeKind>) {
        let f = |n: &NodeKind| {
            match n {
                NodeKind::Operand(n) => {
                    match n {
                        NodeOperand::Integer(v) => {
                            println!("    push {}", v);
                        }
                    }
                },
                NodeKind::BinOperator(n) => {
                    println!("    pop rdi");
                    println!("    pop rax");
                    match n  {
                        NodeBinOperator::Plus => {
                            println!("    add rax, rdi");
                        },
                        NodeBinOperator::Minus => {
                            println!("    sub rax, rdi");
                        },
                        NodeBinOperator::Mul => {
                            println!("    imul rax, rdi");
                        },
                        NodeBinOperator::Div => {
                            println!("    cqo");
                            println!("    idiv rdi");
                        },
                        NodeBinOperator::Eq => {
                            println!("  cmp rax, rdi\n");
                            println!("  sete al\n");
                            println!("  movzb rax, al\n");
                        },
                        NodeBinOperator::Neq => {
                            println!("  cmp rax, rdi\n");
                            println!("  setne al\n");
                            println!("  movzb rax, al\n");
                        },
                        NodeBinOperator::LessThan => {
                            println!("  cmp rax, rdi\n");
                            println!("  setl al\n");
                            println!("  movzb rax, al\n");
                        },
                        NodeBinOperator::LessEq => {
                            println!("  cmp rax, rdi\n");
                            println!("  setle al\n");
                            println!("  movzb rax, al\n");
                        },
                    }
                    println!("    push rax")
                },
            }
        };
        t.postorder(&f)
    }
}


fn run_app(input: &String) -> Result<(), RuccErr> {

    let mut token = Lexer::new(input)?;

    let tree = NodeTree::parse(&mut token)?;

    /* 定型文 */
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    NodeTree::gencode(&tree);

    /* 終わり */
    println!("    pop rax");
    println!("    ret");
    Ok(())
}


fn errorat(input: &String, p: &Point)
{
    eprintln!("{}", input);
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

    let input = &args[1];

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
