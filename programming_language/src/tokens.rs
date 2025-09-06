// Token types for lexical analysis
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals (literal values)
    IntLiteral(i64),
    UintLiteral(u64),
    FloatLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),

    // Identifiers and keywords
    Identifier(String),
    Keyword(Keyword),

    // Operators
    Operator(Operator),

    // Delimiters
    Delimiter(Delimiter),

    // Special tokens
    Newline,
    Whitespace,
    Comment(String),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    // Storage classes
    Auto,
    Register,
    Static,
    Extern,
    Typedef,

    // Type specifiers
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Struct,
    Union,
    Enum,

    // Type qualifiers
    Const,
    Volatile,

    // Control flow
    If,
    Else,
    Switch,
    Case,
    Default,
    While,
    For,
    Do,
    Break,
    Continue,
    Goto,
    Return,

    // Other
    Sizeof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    // Assignment
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LeftShiftAssign,
    RightShiftAssign,

    // Comparison
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Logical
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,

    // Unary
    Increment,
    Decrement,
    AddressOf,
    Dereference,

    // Other
    Conditional, // ?
    Comma,
    Arrow,
    Dot, // -> and .
}

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
    LeftParen,
    RightParen, // ( )
    LeftBrace,
    RightBrace, // { }
    LeftBracket,
    RightBracket, // [ ]
    Semicolon,    // ;
    Colon,        // :
}

pub fn print_tokens(tokens: &[Token]) {
    for token in tokens {
        println!("{:?}", token);
    }
}

pub fn tokenize(raw_code: String) -> Vec<Token> {
    let mut output = Vec::new();
    let chars: Vec<char> = raw_code.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        match ch {
            //Whitespaces and comments will be ignored for now, since they just reduce performance
            // Whitespace (excluding newlines)
            ' ' | '\t' | '\r' => {
                // output.push(Token::Whitespace);
                i += 1;
            }

            // Newlines
            '\n' => {
                // output.push(Token::Newline);
                i += 1;
            }

            // Comments
            '/' if i + 1 < chars.len() => {
                if chars[i + 1] == '/' {
                    // Single-line comment
                    let start = i + 2;
                    i += 2;
                    while i < chars.len() && chars[i] != '\n' {
                        i += 1;
                    }
                    // let comment = chars[start..i].iter().collect();
                    // output.push(Token::Comment(comment));
                } else if chars[i + 1] == '*' {
                    // Multi-line comment
                    let start = i + 2;
                    i += 2;
                    while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                        i += 1;
                    }
                    if i + 1 < chars.len() {
                        // let comment = chars[start..i].iter().collect();
                        // output.push(Token::Comment(comment));
                        i += 2; // Skip */
                    } else {
                        // Unclosed comment, treat as divide operator
                        // output.push(Token::Operator(Operator::Divide));
                        i = start - 2 + 1;
                    }
                } else if chars[i + 1] == '=' {
                    // output.push(Token::Operator(Operator::DivideAssign));
                    i += 2;
                } else {
                    // output.push(Token::Operator(Operator::Divide));
                    i += 1;
                }
            }

            // String literals
            '"' => {
                i += 1; // Skip opening quote
                let mut string_val = String::new();
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        // Handle escape sequences
                        i += 1;
                        match chars[i] {
                            'n' => string_val.push('\n'),
                            't' => string_val.push('\t'),
                            'r' => string_val.push('\r'),
                            '\\' => string_val.push('\\'),
                            '"' => string_val.push('"'),
                            '\'' => string_val.push('\''),
                            '0' => string_val.push('\0'),
                            _ => {
                                string_val.push('\\');
                                string_val.push(chars[i]);
                            }
                        }
                    } else {
                        string_val.push(chars[i]);
                    }
                    i += 1;
                }
                if i < chars.len() {
                    i += 1; // Skip closing quote
                }
                output.push(Token::StringLiteral(string_val));
            }

            // Character literals
            '\'' => {
                i += 1; // Skip opening quote
                let mut char_val = '\0';
                if i < chars.len() {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        // Handle escape sequences
                        i += 1;
                        char_val = match chars[i] {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '\\' => '\\',
                            '"' => '"',
                            '\'' => '\'',
                            '0' => '\0',
                            _ => chars[i],
                        };
                    } else {
                        char_val = chars[i];
                    }
                    i += 1;
                }
                if i < chars.len() && chars[i] == '\'' {
                    i += 1; // Skip closing quote
                }
                output.push(Token::CharLiteral(char_val));
            }

            // Numbers
            '0'..='9' => {
                let start = i;
                let mut has_dot = false;

                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    if chars[i] == '.' {
                        if has_dot {
                            break; // Second dot, stop parsing
                        }
                        has_dot = true;
                    }
                    i += 1;
                }

                let num_str: String = chars[start..i].iter().collect();
                if has_dot {
                    if let Ok(val) = num_str.parse::<f64>() {
                        output.push(Token::FloatLiteral(val));
                    }
                } else {
                    //if it fits in i64, else use u64
                    if let Ok(val) = num_str.parse::<i64>() {
                        output.push(Token::IntLiteral(val));
                    } else {
                        output.push(Token::UintLiteral(num_str.parse::<u64>().unwrap()))
                    }
                }
            }

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let ident: String = chars[start..i].iter().collect();

                let token = match ident.as_str() {
                    "auto" => Token::Keyword(Keyword::Auto),
                    "register" => Token::Keyword(Keyword::Register),
                    "static" => Token::Keyword(Keyword::Static),
                    "extern" => Token::Keyword(Keyword::Extern),
                    "typedef" => Token::Keyword(Keyword::Typedef),
                    "void" => Token::Keyword(Keyword::Void),
                    "char" => Token::Keyword(Keyword::Char),
                    "short" => Token::Keyword(Keyword::Short),
                    "int" => Token::Keyword(Keyword::Int),
                    "long" => Token::Keyword(Keyword::Long),
                    "float" => Token::Keyword(Keyword::Float),
                    "double" => Token::Keyword(Keyword::Double),
                    "signed" => Token::Keyword(Keyword::Signed),
                    "unsigned" => Token::Keyword(Keyword::Unsigned),
                    "struct" => Token::Keyword(Keyword::Struct),
                    "union" => Token::Keyword(Keyword::Union),
                    "enum" => Token::Keyword(Keyword::Enum),
                    "const" => Token::Keyword(Keyword::Const),
                    "volatile" => Token::Keyword(Keyword::Volatile),
                    "if" => Token::Keyword(Keyword::If),
                    "else" => Token::Keyword(Keyword::Else),
                    "switch" => Token::Keyword(Keyword::Switch),
                    "case" => Token::Keyword(Keyword::Case),
                    "default" => Token::Keyword(Keyword::Default),
                    "while" => Token::Keyword(Keyword::While),
                    "for" => Token::Keyword(Keyword::For),
                    "do" => Token::Keyword(Keyword::Do),
                    "break" => Token::Keyword(Keyword::Break),
                    "continue" => Token::Keyword(Keyword::Continue),
                    "goto" => Token::Keyword(Keyword::Goto),
                    "return" => Token::Keyword(Keyword::Return),
                    "sizeof" => Token::Keyword(Keyword::Sizeof),
                    _ => Token::Identifier(ident),
                };
                output.push(token);
            }

            // Delimiters
            '(' => {
                output.push(Token::Delimiter(Delimiter::LeftParen));
                i += 1;
            }
            ')' => {
                output.push(Token::Delimiter(Delimiter::RightParen));
                i += 1;
            }
            '{' => {
                output.push(Token::Delimiter(Delimiter::LeftBrace));
                i += 1;
            }
            '}' => {
                output.push(Token::Delimiter(Delimiter::RightBrace));
                i += 1;
            }
            '[' => {
                output.push(Token::Delimiter(Delimiter::LeftBracket));
                i += 1;
            }
            ']' => {
                output.push(Token::Delimiter(Delimiter::RightBracket));
                i += 1;
            }
            ';' => {
                output.push(Token::Delimiter(Delimiter::Semicolon));
                i += 1;
            }
            ':' => {
                output.push(Token::Delimiter(Delimiter::Colon));
                i += 1;
            }

            // Operators
            '+' => {
                if i + 1 < chars.len() {
                    match chars[i + 1] {
                        '+' => {
                            output.push(Token::Operator(Operator::Increment));
                            i += 2;
                        }
                        '=' => {
                            output.push(Token::Operator(Operator::PlusAssign));
                            i += 2;
                        }
                        _ => {
                            output.push(Token::Operator(Operator::Plus));
                            i += 1;
                        }
                    }
                } else {
                    output.push(Token::Operator(Operator::Plus));
                    i += 1;
                }
            }

            '-' => {
                if i + 1 < chars.len() {
                    match chars[i + 1] {
                        '-' => {
                            output.push(Token::Operator(Operator::Decrement));
                            i += 2;
                        }
                        '=' => {
                            output.push(Token::Operator(Operator::MinusAssign));
                            i += 2;
                        }
                        '>' => {
                            output.push(Token::Operator(Operator::Arrow));
                            i += 2;
                        }
                        _ => {
                            output.push(Token::Operator(Operator::Minus));
                            i += 1;
                        }
                    }
                } else {
                    output.push(Token::Operator(Operator::Minus));
                    i += 1;
                }
            }

            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    output.push(Token::Operator(Operator::MultiplyAssign));
                    i += 2;
                } else {
                    output.push(Token::Operator(Operator::Multiply));
                    i += 1;
                }
            }

            '%' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    output.push(Token::Operator(Operator::ModuloAssign));
                    i += 2;
                } else {
                    output.push(Token::Operator(Operator::Modulo));
                    i += 1;
                }
            }

            '=' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    output.push(Token::Operator(Operator::Equal));
                    i += 2;
                } else {
                    output.push(Token::Operator(Operator::Assign));
                    i += 1;
                }
            }

            '!' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    output.push(Token::Operator(Operator::NotEqual));
                    i += 2;
                } else {
                    output.push(Token::Operator(Operator::LogicalNot));
                    i += 1;
                }
            }

            '<' => {
                if i + 1 < chars.len() {
                    match chars[i + 1] {
                        '<' => {
                            if i + 2 < chars.len() && chars[i + 2] == '=' {
                                output.push(Token::Operator(Operator::LeftShiftAssign));
                                i += 3;
                            } else {
                                output.push(Token::Operator(Operator::LeftShift));
                                i += 2;
                            }
                        }
                        '=' => {
                            output.push(Token::Operator(Operator::LessEqual));
                            i += 2;
                        }
                        _ => {
                            output.push(Token::Operator(Operator::Less));
                            i += 1;
                        }
                    }
                } else {
                    output.push(Token::Operator(Operator::Less));
                    i += 1;
                }
            }

            '>' => {
                if i + 1 < chars.len() {
                    match chars[i + 1] {
                        '>' => {
                            if i + 2 < chars.len() && chars[i + 2] == '=' {
                                output.push(Token::Operator(Operator::RightShiftAssign));
                                i += 3;
                            } else {
                                output.push(Token::Operator(Operator::RightShift));
                                i += 2;
                            }
                        }
                        '=' => {
                            output.push(Token::Operator(Operator::GreaterEqual));
                            i += 2;
                        }
                        _ => {
                            output.push(Token::Operator(Operator::Greater));
                            i += 1;
                        }
                    }
                } else {
                    output.push(Token::Operator(Operator::Greater));
                    i += 1;
                }
            }

            '&' => {
                if i + 1 < chars.len() {
                    match chars[i + 1] {
                        '&' => {
                            output.push(Token::Operator(Operator::LogicalAnd));
                            i += 2;
                        }
                        '=' => {
                            output.push(Token::Operator(Operator::BitwiseAndAssign));
                            i += 2;
                        }
                        _ => {
                            output.push(Token::Operator(Operator::BitwiseAnd));
                            i += 1;
                        }
                    }
                } else {
                    output.push(Token::Operator(Operator::BitwiseAnd));
                    i += 1;
                }
            }

            '|' => {
                if i + 1 < chars.len() {
                    match chars[i + 1] {
                        '|' => {
                            output.push(Token::Operator(Operator::LogicalOr));
                            i += 2;
                        }
                        '=' => {
                            output.push(Token::Operator(Operator::BitwiseOrAssign));
                            i += 2;
                        }
                        _ => {
                            output.push(Token::Operator(Operator::BitwiseOr));
                            i += 1;
                        }
                    }
                } else {
                    output.push(Token::Operator(Operator::BitwiseOr));
                    i += 1;
                }
            }

            '^' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    output.push(Token::Operator(Operator::BitwiseXorAssign));
                    i += 2;
                } else {
                    output.push(Token::Operator(Operator::BitwiseXor));
                    i += 1;
                }
            }

            '~' => {
                output.push(Token::Operator(Operator::BitwiseNot));
                i += 1;
            }

            '?' => {
                output.push(Token::Operator(Operator::Conditional));
                i += 1;
            }

            ',' => {
                output.push(Token::Operator(Operator::Comma));
                i += 1;
            }

            '.' => {
                output.push(Token::Operator(Operator::Dot));
                i += 1;
            }

            // Skip unknown characters
            _ => {
                i += 1;
            }
        }
    }

    output.push(Token::EOF);
    output
}
