use std::{env, error::Error, fmt, fs, io};

mod TokenDef;
mod Scanner;
mod GenerateAst;

#[derive(Debug, Clone)]
struct ErrorInfo {
    line: usize,
    err_start_idx: usize,
    err_msg: String,
}

impl ErrorInfo {
    fn new(line: usize, err_start_idx: usize, err_msg: String) -> Self {
        Self {line, err_start_idx, err_msg}
    } 
}

#[derive(Debug, Clone)]
enum SyntaxError {
    DescribedError(ErrorInfo),
    SimpleError(String),
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyntaxError::DescribedError(error_desc) => 
                write!(f, "Error happened on line {} starting at the {}. character:\n{}", error_desc.line, error_desc.err_start_idx, error_desc.err_msg),
            SyntaxError::SimpleError(error_str) => write!(f, "{}", error_str),
        }
    }
}

impl std::error::Error for SyntaxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            //not so meaningful but can help in the interoperability with other error types later...
            _ => None,
        }
    }
}

fn runFile(filename: &str) -> Result<(), io::Error>{
    let contents = fs::read_to_string(filename)?;
    run(&contents);
    Ok(())
}

fn runPrompt() -> Result<(), io::Error>{
    let mut current_input = String::new();

    loop {
        print!(">");
        io::stdin().read_line(&mut current_input)?;
        if current_input.trim() == "" {
            println!("Exiting lox...");
            break;
        }
        run(&current_input);
    }
    

    Ok(())
}

fn run(source: &str) -> Result<(), SyntaxError> {
    let mut scanner = Scanner::Scanner::new(source);
    let tokens: Vec<TokenDef::Token> = scanner.scan_tokens();

    for t in &tokens {
        println!("{:?}", t);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Found args: {args:?}");
    if args.len() > 1 {
        
        if args[1] == "-ast" {
            match args.get(2){
                None => (),
                Some(arg_str) => {
                    GenerateAst::generate(arg_str.clone())?;
                    return Ok(());
                }
            }
        }
    
        eprintln!("Usage: jlox -- file_name OR jlox -- -ast output_directory");
        return Err(Box::new(SyntaxError::SimpleError("Usage: jlox -- file_name OR jlox -- -ast output_directory".to_string())));
    
    }
    else if args.len() == 1 {
        runFile(&args[0]);
    }
    else {
        runPrompt();
    }

    Ok(())
    
}