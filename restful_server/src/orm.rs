//todo: write an enum which will have a field "value", which can have a string (which will contain sql-formatted info like id int not null...)

//then write a macro called !table, which will generate a vector of these values

use mysql::*;
use mysql::prelude::*;


macro_rules! table {
    (
        $table_name:ident
        $( 
            $field_name:ident $field_type:ident $(($($args:expr),*))? $( $constraint:ident $(($($cargs:expr),*))? )*
        );* $(;)? 
    ) => {
        Table {
            name: stringify!($table_name).to_string(),
            fields: vec![ 
                $( 
                    Field {
                        field_name: stringify!($field_name).to_string(),
                        field_type: table!(@type $field_type $(($($args),*))?),
                        constraints: vec![ 
                            $( 
                                table!(@constraint $constraint $(($($cargs),*))?) 
                            ),*
                        ],
                    }
                ),*
            ]
        }
    };

    // Internal matcher to map shorthand to Type variants
    (@type TINYINT) => { Type::TINYINT };
    (@type SMALLINT) => { Type::SMALLINT };
    (@type MEDIUMINT) => { Type::MEDIUMINT };
    (@type INT) => { Type::INT };
    (@type BIGINT) => { Type::BIGINT };
    (@type FLOAT) => { Type::FLOAT };
    (@type DOUBLE) => { Type::DOUBLE };
    (@type DECIMAL($p:expr, $s:expr)) => { Type::DECIMAL($p, $s) };

    (@type DATE) => { Type::DATE };
    (@type DATETIME) => { Type::DATETIME };
    (@type TIMESTAMP) => { Type::TIMESTAMP };
    (@type TIME) => { Type::TIME };
    (@type YEAR) => { Type::YEAR };

    (@type CHAR($len:expr)) => { Type::CHAR($len) };
    (@type VARCHAR($len:expr)) => { Type::VARCHAR($len) };

    (@type TINYTEXT($len:expr)) => { Type::TINYTEXT($len) };
    (@type TEXT($len:expr)) => { Type::TEXT($len) };
    (@type LONGTEXT($len:expr)) => { Type::LONGTEXT($len) };

    (@type BINARY) => { Type::BINARY };
    (@type VARBINARY) => { Type::VARBINARY };

    (@type ENUM($($value:expr),*)) => { Type::ENUM(vec![$(String::from($value)),*]) };
    (@type SET($($value:expr),*)) => { Type::SET(vec![$(String::from($value)),*]) };

    // Added back BOOL type variant
    (@type BOOL) => { Type::BOOL };

    // Internal matcher to map shorthand constraints
    (@constraint NOT_NULL) => { Constraint::NotNull };
    (@constraint UNIQUE) => { Constraint::Unique };
    // Adjusted to infer the type and create a DefaultValue variant
    (@constraint DEFAULT($val:expr)) => {
        Constraint::Default(table!(@default_value $val))
    };
    (@constraint PRIMARY_KEY) => { Constraint::PrimaryKey };
    (@constraint FOREIGN_KEY($table:expr, $field:expr)) => {
        Constraint::ForeignKey { table: $table.to_string(), field: $field.to_string() }
    };

    // Handle TRUE and FALSE as custom tokens for DefaultValue::Int(1) and DefaultValue::Int(0)
    (@default_value true) => { DefaultValue::Int(1) };
    (@default_value false) => { DefaultValue::Int(0) };

    // Internal matcher to infer the type of DefaultValue
    // Special handling for bool, to avoid treating it as a numeric type
    (@default_value $val:expr) => {
        if let Some(val) = table!(@as_true $val) {
            DefaultValue::Int(1)
        }
        else if let Some(val) = table!(@as_false $val) {
            DefaultValue::Int(0)
        }
        else if let Some(val) = table!(@as_int $val) {
            DefaultValue::Int(val)
        } else if let Some(val) = table!(@as_float $val) {
            DefaultValue::Float(val)
        } else {
            DefaultValue::String($val.to_string())
        }
    };

    // Matchers to try to convert the value to specific types
    // Handle boolean explicitly
    (@as_true $val:expr) => {
        if $val == true { Some(1) } else { None }
    };
    (@as_false $val:expr) => {
        if $val == false { Some(0) } else { None }
    };
    (@as_int $val:expr) => {
        if let Ok(val) = ($val as i64).to_string().parse::<i64>() { Some(val) } else { None }
    };
    // Handle float parsing explicitly
    (@as_float $val:expr) => {
        if let Ok(val) = ($val as f64).to_string().parse::<f64>() { Some(val) } else { None }
    };
}















#[derive(PartialEq, Clone, Debug)]
enum Type {
    TINYINT,
    SMALLINT,
    MEDIUMINT,
    INT,
    BIGINT,
    FLOAT,
    DOUBLE,
    DECIMAL(u8, u8),

    BOOL,

    DATE,
    DATETIME,
    TIMESTAMP,
    TIME,
    YEAR,

    CHAR(u8),
    VARCHAR(u16),

    TINYTEXT(u8),
    TEXT(u16),
    LONGTEXT(u32),

    BINARY,
    VARBINARY,

    ENUM(Vec<String>),
    SET(Vec<String>)
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::TINYINT => "TINYINT".to_string(),
            Type::SMALLINT => "SMALLINT".to_string(),
            Type::MEDIUMINT => "MEDIUMINT".to_string(),
            Type::INT => "INT".to_string(),
            Type::BIGINT => "BIGINT".to_string(),
            Type::FLOAT => "FLOAT".to_string(),
            Type::DOUBLE => "DOUBLE".to_string(),
            Type::DECIMAL(p, s) => format!("DECIMAL({}, {})", p, s),
            Type::BOOL => "BOOL".to_string(),
            Type::DATE => "DATE".to_string(),
            Type::DATETIME => "DATETIME".to_string(),
            Type::TIMESTAMP => "TIMESTAMP".to_string(),
            Type::TIME => "TIME".to_string(),
            Type::YEAR => "YEAR".to_string(),
            Type::CHAR(len) => format!("CHAR({})", len),
            Type::VARCHAR(len) => format!("VARCHAR({})", len),
            Type::TINYTEXT(len) => format!("TINYTEXT({})", len),
            Type::TEXT(len) => format!("TEXT({})", len),
            Type::LONGTEXT(len) => format!("LONGTEXT({})", len),
            Type::BINARY => "BINARY".to_string(),
            Type::VARBINARY => "VARBINARY".to_string(),
            Type::ENUM(values) => format!("ENUM({})", values.join(", ")),
            Type::SET(values) => format!("SET({})", values.join(", ")),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Constraint {
    NotNull,
    Unique,
    Default(DefaultValue),
    PrimaryKey,
    ForeignKey { table: String, field: String },
}

impl ToString for Constraint {
    fn to_string(&self) -> String {
        match self {
            Constraint::NotNull => "NOT NULL".to_string(),
            Constraint::Unique => "UNIQUE".to_string(),
            Constraint::Default(value) => format!("DEFAULT {}", value.to_string()),
            Constraint::PrimaryKey => "PRIMARY KEY".to_string(),
            Constraint::ForeignKey { table, field } => {
                format!("REFERENCES {}({})", table, field)
            },
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum DefaultValue {
    String(String),
    Int(i64),
    Float(f64),
}

impl ToString for DefaultValue {
    fn to_string(&self) -> String {
        match self {
            DefaultValue::String(val) => format!("'{}'", val),  // Strings are quoted
            DefaultValue::Int(val) => val.to_string(),
            DefaultValue::Float(val) => val.to_string(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Field {
    field_name: String,
    field_type: Type,
    constraints: Vec<Constraint>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Table {
    name: String,
    fields: Vec<Field>,
}

pub struct DBConnector {
    tables: Vec<Table>,
    db_name: String,
    pool: Pool,
    connection: PooledConn,
}

impl DBConnector {
    pub fn new(username: &str, password: &str, address: &str, port: usize, db_name: &str) -> Result<Self, mysql::Error> {
        
        let init_pool = Pool::new(format!("mysql://{username}:{password}@{address}:{port}").as_str())?;
        let mut init_conn = init_pool.get_conn()?;

        init_conn.query_drop(format!("CREATE DATABASE IF NOT EXISTS {db_name}"))?;

        init_conn.query_drop(format!("USE {db_name}"))?;

    
        Ok(DBConnector{tables: vec![], db_name: db_name.to_string(), pool: init_pool, connection: init_conn})
    }
    // pub fn query(&mut self, command: impl AsRef<str>) -> Result<(), mysql::Error>{
    //     let query_result = self.connection.query(command)?;


    //     Ok(())
    // }
    pub fn add_table(&mut self, table: Table){
        let mut query = format!("CREATE TABLE {} (", table.name);

        for field in &table.fields {
            query += &format!("{} {} ", &field.field_name, &field.field_type.to_string());

        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tinyint() {
        let table = table! {
            TestTable
            id TINYINT;
        };
        assert_eq!(table.name, "TestTable");
        assert_eq!(table.fields.len(), 1);
        assert_eq!(table.fields[0].field_name, "id");
        assert_eq!(table.fields[0].field_type, Type::TINYINT);
        assert!(table.fields[0].constraints.is_empty());
    }

    #[test]
    fn test_smallint() {
        let table = table! {
            TestTable
            id SMALLINT;
        };
        assert_eq!(table.fields[0].field_type, Type::SMALLINT);
    }

    #[test]
    fn test_mediumint() {
        let table = table! {
            TestTable
            id MEDIUMINT;
        };
        assert_eq!(table.fields[0].field_type, Type::MEDIUMINT);
    }

    #[test]
    fn test_int() {
        let table = table! {
            TestTable
            id INT;
        };
        assert_eq!(table.fields[0].field_type, Type::INT);
    }

    #[test]
    fn test_bigint() {
        let table = table! {
            TestTable
            id BIGINT;
        };
        assert_eq!(table.fields[0].field_type, Type::BIGINT);
    }

    #[test]
    fn test_float() {
        let table = table! {
            TestTable
            id FLOAT;
        };
        assert_eq!(table.fields[0].field_type, Type::FLOAT);
    }

    #[test]
    fn test_double() {
        let table = table! {
            TestTable
            id DOUBLE;
        };
        assert_eq!(table.fields[0].field_type, Type::DOUBLE);
    }

    #[test]
    fn test_decimal() {
        let table = table! {
            TestTable
            id DECIMAL(10, 2);
        };
        assert_eq!(table.fields[0].field_type, Type::DECIMAL(10, 2));
    }

    #[test]
    fn test_bool() {
        let table = table! {
            TestTable
            id BOOL;
        };
        assert_eq!(table.fields[0].field_type, Type::BOOL);
    }

    #[test]
    fn test_date() {
        let table = table! {
            TestTable
            id DATE;
        };
        assert_eq!(table.fields[0].field_type, Type::DATE);
    }

    #[test]
    fn test_datetime() {
        let table = table! {
            TestTable
            id DATETIME;
        };
        assert_eq!(table.fields[0].field_type, Type::DATETIME);
    }

    #[test]
    fn test_timestamp() {
        let table = table! {
            TestTable
            id TIMESTAMP;
        };
        assert_eq!(table.fields[0].field_type, Type::TIMESTAMP);
    }

    #[test]
    fn test_time() {
        let table = table! {
            TestTable
            id TIME;
        };
        assert_eq!(table.fields[0].field_type, Type::TIME);
    }

    #[test]
    fn test_year() {
        let table = table! {
            TestTable
            id YEAR;
        };
        assert_eq!(table.fields[0].field_type, Type::YEAR);
    }

    #[test]
    fn test_char() {
        let table = table! {
            TestTable
            id CHAR(255);
        };
        assert_eq!(table.fields[0].field_type, Type::CHAR(255));
    }

    #[test]
    fn test_varchar() {
        let table = table! {
            TestTable
            id VARCHAR(255);
        };
        assert_eq!(table.fields[0].field_type, Type::VARCHAR(255));
    }

    #[test]
    fn test_tinytext() {
        let table = table! {
            TestTable
            id TINYTEXT(255);
        };
        assert_eq!(table.fields[0].field_type, Type::TINYTEXT(255));
    }

    #[test]
    fn test_text() {
        let table = table! {
            TestTable
            id TEXT(255);
        };
        assert_eq!(table.fields[0].field_type, Type::TEXT(255));
    }

    #[test]
    fn test_longtext() {
        let table = table! {
            TestTable
            id LONGTEXT(255);
        };
        assert_eq!(table.fields[0].field_type, Type::LONGTEXT(255));
    }

    #[test]
    fn test_binary() {
        let table = table! {
            TestTable
            id BINARY;
        };
        assert_eq!(table.fields[0].field_type, Type::BINARY);
    }

    #[test]
    fn test_varbinary() {
        let table = table! {
            TestTable
            id VARBINARY;
        };
        assert_eq!(table.fields[0].field_type, Type::VARBINARY);
    }

    #[test]
    fn test_enum() {
        let table = table! {
            TestTable
            id ENUM("value1", "value2");
        };
        assert_eq!(table.fields[0].field_type, Type::ENUM(vec!["value1".to_string(), "value2".to_string()]));
    }

    #[test]
    fn test_set() {
        let table = table! {
            TestTable
            id SET("option1", "option2");
        };
        assert_eq!(table.fields[0].field_type, Type::SET(vec!["option1".to_string(), "option2".to_string()]));
    }

    #[test]
    fn test_not_null_constraint() {
        let table = table! {
            TestTable
            id INT NOT_NULL;
        };
        assert_eq!(table.fields[0].constraints, vec![Constraint::NotNull]);
    }

    #[test]
    fn test_unique_constraint() {
        let table = table! {
            TestTable
            id INT UNIQUE;
        };
        assert_eq!(table.fields[0].constraints, vec![Constraint::Unique]);
    }

    #[test]
    fn test_default_constraint() {
        let table = table! {
            TestTable
            id INT DEFAULT(42);
        };
        assert_eq!(table.fields[0].constraints, vec![Constraint::Default(DefaultValue::Int(42))]);
    }

    #[test]
    fn test_primary_key_constraint() {
        let table = table! {
            TestTable
            id INT PRIMARY_KEY;
        };
        assert_eq!(table.fields[0].constraints, vec![Constraint::PrimaryKey]);
    }

    #[test]
    fn test_foreign_key_constraint() {
        let table = table! {
            TestTable
            id INT FOREIGN_KEY("OtherTable", "other_field");
        };
        assert_eq!(table.fields[0].constraints, vec![Constraint::ForeignKey { table: "OtherTable".to_string(), field: "other_field".to_string() }]);
    }

    // Combination of constraints
    #[test]
    fn test_combined_constraints() {
        let table = table! {
            TestTable
            id INT NOT_NULL UNIQUE DEFAULT(42) PRIMARY_KEY;
            is_good BOOL DEFAULT(true);
        };
        assert_eq!(table.fields[0].constraints.len(), 4);
        assert_eq!(table.fields[0].constraints[0], Constraint::NotNull);
        assert_eq!(table.fields[0].constraints[1], Constraint::Unique);
        assert_eq!(table.fields[0].constraints[2], Constraint::Default(DefaultValue::Int(42)));
        assert_eq!(table.fields[0].constraints[3], Constraint::PrimaryKey);

        assert_eq!(table.fields[1].constraints[0], Constraint::Default(DefaultValue::Int(1)));
    }
}
