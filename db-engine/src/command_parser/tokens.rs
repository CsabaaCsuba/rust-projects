#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum TypeNotation {
    INT,
    TEXT,
    DOUBLE,
    BOOL,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    CREATE,
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
    ALTER,
    DROP,
    USE,
    // GRANT,
    // REVOKE,
    // TRUNCATE, //to quickly remove all rows from a table??
    TABLE,
    DATABASE,

    NO_BACKTICK_NAME_UNIT(String), //fields upon creation

    NAME_UNIT(String),
    STRING_VALUE(String),
    INT_VALUE(i32),
    BOOL_VALUE(bool),
    DOUBLE_VALUE(f64),

    COLLATE,
    CHARACTER,
    SET,

    COMMA,
    OPEN_PARENTHESIS,
    CLOSING_PARENTHESIS,
    SEMICOLON,
    

    EQUAL_EXPRESSION,
    SMALLER_THAN_OPERATOR,
    BIGGER_THAN_OPERATOR,
    
    ASTERISK,

    FROM,
    WHERE,
    AND,
    OR,
    GROUP,
    BY,
    ORDER,
    HAVING,
    LIMIT,

    VALUES,

    TYPE_KEYWORD(TypeNotation),


    PRIMARY,
    KEY,
    NOT,
    NULL,
    UNIQUE,
    AUTO_INCREMENT,
    DEFAULT,

    CONSTRAINT,

    FOREIGN,
    REFERENCES,

    ON,
    





}
