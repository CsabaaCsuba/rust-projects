#[derive(Debug)]
pub enum Token {
    CREATE,
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
    ALTER,
    DROP,
    // GRANT,
    // REVOKE,
    // TRUNCATE, //to quickly remove all rows from a table??
    TABLE,
    DATABASE,

    NAME_UNIT(String),
    STRING_VALUE(String),
    INT_VALUE(i32),
    BOOL_VALUE(bool),
    DOUBLE_VALUE(f64),

    COLLATE,
    CHARACTER,
    SET,

    

    



}
