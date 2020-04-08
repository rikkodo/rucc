use super::ruccerr;
use super::lexer;
use super::bintree;

// 抽象構文木
// オペレータ
#[derive(Debug)]
pub enum NodeOperand {
    Integer(i32),  // 数値 i32としておく
}

// 二項オペランド
#[derive(Debug)]
pub enum NodeBinOperator {
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
pub enum NodeKind {
    Operand(NodeOperand),
    BinOperator(NodeBinOperator),
}

pub fn parse(lex: &mut lexer::Lexer) -> Result<bintree::BinTree::<NodeKind>, ruccerr::RuccErr> {
    expr(lex)
}

fn expr(lex: &mut lexer::Lexer) -> Result<bintree::BinTree::<NodeKind>, ruccerr::RuccErr> {
    equality(lex)
}

fn equality(lex: &mut lexer::Lexer) -> Result<bintree::BinTree::<NodeKind>, ruccerr::RuccErr> {
    let mut l = relational(lex)?;
    loop {
        if lex.consume_reserved("==")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::Eq), l, relational(lex)?)
        } else if lex.consume_reserved("!=")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::Neq), l, relational(lex)?)
        } else {
            return Ok(l)
        }
    }
}

fn relational(lex: &mut lexer::Lexer) -> Result<bintree::BinTree::<NodeKind>, ruccerr::RuccErr> {
    let mut l = add(lex)?;
    loop {
        if lex.consume_reserved("<")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::LessThan), l, add(lex)?)
        } else if lex.consume_reserved("<=")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::LessEq), l, add(lex)?)
        } else if lex.consume_reserved(">")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::LessThan), add(lex)?, l)
        } else if lex.consume_reserved(">=")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::LessEq), add(lex)?, l)
        } else {
            return Ok(l)
        }
    }
}

fn add(lex: &mut lexer::Lexer) -> Result<bintree::BinTree::<NodeKind>, ruccerr::RuccErr> {
    let mut l = mul(lex)?;
    loop {
        if lex.consume_reserved("+")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::Plus), l, mul(lex)?)
        } else if lex.consume_reserved("-")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::Minus), l, mul(lex)?)
        } else {
            return Ok(l)
        }
    }
}

fn mul(lex: &mut lexer::Lexer) -> Result<bintree::BinTree::<NodeKind>, ruccerr::RuccErr> {
    let mut l = unary(lex)?;
    loop {
        if lex.consume_reserved("*")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::Mul), l, unary(lex)?)
        } else if lex.consume_reserved("/")? {
            l = bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::Div), l, unary(lex)?)
        } else {
            return Ok(l)
        }
    }
}

fn unary(lex: &mut lexer::Lexer) -> Result<bintree::BinTree::<NodeKind>, ruccerr::RuccErr> {
    if lex.consume_reserved("+")? {
        Ok(unary(lex)?)
    } else if lex.consume_reserved("-")? {
        Ok(bintree::BinTree::<NodeKind>::new_node(NodeKind::BinOperator(NodeBinOperator::Minus), new_node_num(0), unary(lex)?))
    } else {
        Ok(primary(lex)?)
    }
}

fn primary(lex: &mut lexer::Lexer) -> Result<bintree::BinTree::<NodeKind>, ruccerr::RuccErr> {
    if lex.consume_reserved("(")? {
        let l = expr(lex)?;
        lex.expect_reserved(")")?;
        Ok(l)
    } else {
        Ok(new_node_num(lex.expect_number()?))
    }
}

fn new_node_num(val: i32) -> bintree::BinTree::<NodeKind> {
    bintree::BinTree::<NodeKind>::new_leaf(
        NodeKind::Operand(NodeOperand::Integer(val)),
    )
}
