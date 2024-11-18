//todo: write an enum which will have a field "value", which can have a string (which will contain sql-formatted info like id int not null...)

//then write a macro called !table, which will generate a vector of these values

macro_rules! table {
    (
        $( // Match each field
            $field_name:ident $field_type:ident $(($($args:expr),*))? $( $constraint:ident $(($($cargs:expr),*))? )*
        );* $(;)?
    ) => {
        vec![
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

    (@type BOOL) => { Type::BOOL };

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

    (@type ENUM($values:expr)) => { Type::ENUM($values) };
    (@type SET($values:expr)) => { Type::SET($values) };

    // Internal matcher to map shorthand constraints
    (@constraint NOT_NULL) => { Constraint::NotNull };
    (@constraint UNIQUE) => { Constraint::Unique };
    (@constraint DEFAULT($val:expr)) => { Constraint::Default($val.to_string()) };
    (@constraint PRIMARY_KEY) => { Constraint::PrimaryKey };
    (@constraint FOREIGN_KEY($table:expr, $field:expr)) => {
        Constraint::ForeignKey { table: $table.to_string(), field: $field.to_string() }
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

    ENUM(Vec<&'static str>),
    SET(Vec<&'static str>)
}

#[derive(PartialEq, Clone, Debug)]
enum Constraint {
    NotNull,
    Unique,
    Default(String),
    PrimaryKey,
    ForeignKey { table: String, field: String },
}

#[derive(PartialEq, Clone, Debug)]
struct Field {
    field_name: String,
    field_type: Type,
    constraints: Vec<Constraint>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_test1() {
        let fields = table! {
            id INT PRIMARY_KEY NOT_NULL;
            name VARCHAR(255) NOT_NULL UNIQUE;
            is_active BOOL DEFAULT("1");
            salary DECIMAL(10, 2) DEFAULT("0.00");
            department_id INT; // No constraints
        };

        let expected = vec![
            Field {
                field_name: "id".to_string(),
                field_type: Type::INT,
                constraints: vec![Constraint::PrimaryKey, Constraint::NotNull],
            },
            Field {
                field_name: "name".to_string(),
                field_type: Type::VARCHAR(255),
                constraints: vec![Constraint::NotNull, Constraint::Unique],
            },
            Field {
                field_name: "is_active".to_string(),
                field_type: Type::BOOL,
                constraints: vec![Constraint::Default("1".to_string())],
            },
            Field {
                field_name: "salary".to_string(),
                field_type: Type::DECIMAL(10, 2),
                constraints: vec![Constraint::Default("0.00".to_string())],
            },
            Field {
                field_name: "department_id".to_string(),
                field_type: Type::INT,
                constraints: vec![], // No constraints
            },
        ];

        assert_eq!(fields, expected);
    }

    
}