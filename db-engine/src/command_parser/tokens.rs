#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum TypeNotation {
    INT,
    TEXT,
    DOUBLE,
    BOOL,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum StringWrapper {
    Data(String),
    Filler,
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

    INTO,

    NO_BACKTICK_NAME_UNIT(StringWrapper), //fields upon creation

    NAME_UNIT(StringWrapper),
    STRING_VALUE(StringWrapper),
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

    LIKE,

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

    ASC,
    DESC,

    VALUES,

    UNION,

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
    CASCADE,
    NO,
    ACTION, //NO ACTION is an ON DELETE possibility
    RESTRICT,
}
