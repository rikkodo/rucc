use super::ruccerr;
use super::lexer;
use super::bintree;
use super::parser;

fn gencode(t: &bintree::BinTree::<parser::NodeKind>) {
    let f = |n: &parser::NodeKind| {
        match n {
            parser::NodeKind::Operand(n) => {
                match n {
                    parser::NodeOperand::Integer(v) => {
                        println!("    push {}", v);
                    }
                }
            },
            parser::NodeKind::BinOperator(n) => {
                println!("    pop rdi");
                println!("    pop rax");
                match n  {
                    parser::NodeBinOperator::Plus => {
                        println!("    add rax, rdi");
                    },
                    parser::NodeBinOperator::Minus => {
                        println!("    sub rax, rdi");
                    },
                    parser::NodeBinOperator::Mul => {
                        println!("    imul rax, rdi");
                    },
                    parser::NodeBinOperator::Div => {
                        println!("    cqo");
                        println!("    idiv rdi");
                    },
                    parser::NodeBinOperator::Eq => {
                        println!("  cmp rax, rdi\n");
                        println!("  sete al\n");
                        println!("  movzb rax, al\n");
                    },
                    parser::NodeBinOperator::Neq => {
                        println!("  cmp rax, rdi\n");
                        println!("  setne al\n");
                        println!("  movzb rax, al\n");
                    },
                    parser::NodeBinOperator::LessThan => {
                        println!("  cmp rax, rdi\n");
                        println!("  setl al\n");
                        println!("  movzb rax, al\n");
                    },
                    parser::NodeBinOperator::LessEq => {
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

pub fn run_app(input: &String) -> Result<(), ruccerr::RuccErr> {

    let mut token = lexer::Lexer::new(input)?;

    let tree = parser::parse(&mut token)?;

    /* 定型文 */
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    gencode(&tree);

    /* 終わり */
    println!("    pop rax");
    println!("    ret");
    Ok(())
}