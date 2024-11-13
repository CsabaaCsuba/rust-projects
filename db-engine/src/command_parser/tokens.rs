#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum TypeNotation {
    INT,
    BIGINT,
    SMALLINT,
    TINYINT,

    FLOAT,
    DOUBLE,

    BOOL,

    CHAR,
    VARCHAR,
    TEXT,

    DATE,      //year, month, day
    TIME,      //hours, minutes, seconds
    DATETIME,  //year-month-day hours:minutes:seconds
    TIMESTAMP, //same as datetime but stores everything as UTC0 (so time zone has to be specified upon saving I guess?)
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum SWrap {
    Data(String),
    Filler,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum IsSequenceValid {
    True(usize),
    False,
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
    INDEX,
    TRIGGER,

    INTO,

    NO_BACKTICK_NAME_UNIT(SWrap), //fields upon creation

    NAME_UNIT(SWrap),

    STRING_VALUE(SWrap),
    INT_VALUE(i64),
    BOOL_VALUE(bool),
    DOUBLE_VALUE(f64),

    COLLATE,
    CHARACTER,
    SET,

    COMMA,
    OPEN_PARENTHESIS,
    CLOSING_PARENTHESIS,
    SEMICOLON,

    EQUAL_SIGN,
    SMALLER_THAN_OPERATOR,
    BIGGER_THAN_OPERATOR,

    IS,
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

    COUNT,
    SUM,
    AVG,
    MAX,
    MIN,
    //DATE_SUB,
    //INTERVAL,
    //CURDATE,
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
    CHECK,
    // CURRENT_TIMESTAMP,
    CONSTRAINT,

    FOREIGN,
    REFERENCES,

    ON,
    CASCADE,
    NO,
    ACTION, //NO ACTION is an ON DELETE possibility
    RESTRICT,
}
