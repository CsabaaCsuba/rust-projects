use std::{env, error, fmt, fs, io};

#[derive(Debug, Clone)]
struct ErrorInfo {
    line: usize,
    err_start_idx: usize,
    err_msg: String,
}

#[derive(Debug, Clone)]
enum MyError {
    DescribedError(ErrorInfo),
    SimpleError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::DescribedError(error_desc) => 
                write!(f, "Error happened on line {} starting at the {}. character:\n{}", error_desc.line, error_desc.err_start_idx, error_desc.err_msg),
            MyError::SimpleError(error_str) => write!(f, "{}", error_str),
        }
    }
}

impl std::error::Error for MyError {
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

fn run(source: &str) -> Result<(), MyError> {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for t in &tokens {
        println!("{:?}", t);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        eprintln!("Usage: jlox -- file_name");
        return;
    }
    else if args.len() == 1 {
        runFile(&args[0]);
    }
    else {
        runPrompt();
    }
    
}