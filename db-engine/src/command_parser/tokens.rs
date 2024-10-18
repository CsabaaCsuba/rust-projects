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
    TRUNCATE, //to quickly remove all rows from a table??
    TABLE,
    DATABASE
}
