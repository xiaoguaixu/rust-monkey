use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::Environment;
use crate::parser::Parser;

const_str_val_declare!(PROMPT, ">> ");
pub fn start() {
    let mut env = Environment::new();
    loop {
        eprint!("{}", PROMPT);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let l = Lexer::new(&input);
        let mut p = Parser::new(Box::new(l));
        let program = p.parse_program();
        if p.errors().len() > 0 {
            print_parser_errors(&p.errors());
        }

        let evaluated = eval(&*program, &mut env);
        if evaluated.is_none() {
            continue;
        }
        let evaluated = evaluated.unwrap();

        println!("{}", evaluated.inspect());
    }
}


const MONKEY_FACE: &str = r#"            __,__
.--.  .-"     "-.  .--.
/ .. \/  .-. .-.  \/ .. \
| |  '|  /   Y   \  |'  | |
| \   \  \ 0 | 0 /  /   / |
\ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;

fn print_parser_errors(errors: &Vec<String>) {
    println!("{}", MONKEY_FACE);
    println!("{}", "Woops! We ran into some monkey business here!");
    println!("{}", " parser errors:");
    for msg in errors {
        println!("{}", msg);
    }
}