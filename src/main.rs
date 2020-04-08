use rucc::ruccerr;
use rucc::lexer;
use rucc::codegen;

fn errorat(input: &String, p: &lexer::Point)
{
    eprintln!("{}", input);
    let mut sp = String::new();
    for _ in 0..p.get_pos() {
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

    std::process::exit(match codegen::run_app(input) {
        Ok(_) => 0,
        Err(err) => {
            match &err {
                ruccerr::RuccErr::ParseErr(_, p) => errorat(input, &p),
                ruccerr::RuccErr::TokenErr(_, t) => errorat(input, t.get_point()),
                _ => ()
            }
            eprintln!("Error: {}", err);
            1
        },
    });
}
