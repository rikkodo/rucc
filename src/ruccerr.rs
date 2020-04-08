use super::lexer;

#[derive(Debug)]
pub enum RuccErr {
    ParseErr(String, lexer::Point),
    TokenErr(String, lexer::Token),
    InsideErr(String),
}

impl std::error::Error for RuccErr {}
impl std::fmt::Display for RuccErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuccErr::ParseErr(s, p) => write!(f, "{} at {}", s, p),
            RuccErr::TokenErr(s, t) => write!(f, "{} {}", s, t),
            RuccErr::InsideErr(s) => write!(f, "{}", s),
        }
    }
}
