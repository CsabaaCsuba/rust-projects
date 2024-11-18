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

    (@type ENUM($($value:expr),*)) => { Type::ENUM(vec![$(String::from($value)),*]) };
    (@type SET($($value:expr),*)) => { Type::SET(vec![$(String::from($value)),*]) };

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

    ENUM(Vec<String>),
    SET(Vec<String>)
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

    #[test]
    fn foreignkey_test() {
        let fields: Vec<Field> = table! {
            character_id INT PRIMARY_KEY NOT_NULL;
            name VARCHAR(100) NOT_NULL UNIQUE;
            health INT DEFAULT(100);
            level INT DEFAULT(1);
            inventory_id INT FOREIGN_KEY("inventory", "character_id");
        };

        let expected = vec![
            Field {
                field_name: "character_id".to_string(),
                field_type: Type::INT,
                constraints: vec![Constraint::PrimaryKey, Constraint::NotNull],
            },
            Field {
                field_name: "name".to_string(),
                field_type: Type::VARCHAR(100),
                constraints: vec![Constraint::NotNull, Constraint::Unique],
            },
            Field {
                field_name: "health".to_string(),
                field_type: Type::INT,
                constraints: vec![Constraint::Default("100".to_string())],
            },
            Field {
                field_name: "level".to_string(),
                field_type: Type::INT,
                constraints: vec![Constraint::Default("1".to_string())],
            },
            Field {
                field_name: "inventory_id".to_string(),
                field_type: Type::INT,
                constraints: vec![
                    Constraint::ForeignKey {
                        table: "inventory".to_string(),
                        field: "character_id".to_string(),
                    }
                ],
            },
        ];

        assert_eq!(fields, expected);
    }


        #[test]
    fn tinyint_test() {
        let fields: Vec<Field> = table! {
            id TINYINT PRIMARY_KEY NOT_NULL;
        };

        let expected = vec![
            Field {
                field_name: "id".to_string(),
                field_type: Type::TINYINT,
                constraints: vec![Constraint::PrimaryKey, Constraint::NotNull],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn smallint_test() {
        let fields: Vec<Field> = table! {
            id SMALLINT PRIMARY_KEY NOT_NULL;
        };

        let expected = vec![
            Field {
                field_name: "id".to_string(),
                field_type: Type::SMALLINT,
                constraints: vec![Constraint::PrimaryKey, Constraint::NotNull],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn mediumint_test() {
        let fields: Vec<Field> = table! {
            id MEDIUMINT PRIMARY_KEY NOT_NULL;
        };

        let expected = vec![
            Field {
                field_name: "id".to_string(),
                field_type: Type::MEDIUMINT,
                constraints: vec![Constraint::PrimaryKey, Constraint::NotNull],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn int_test() {
        let fields: Vec<Field> = table! {
            id INT PRIMARY_KEY NOT_NULL;
        };

        let expected = vec![
            Field {
                field_name: "id".to_string(),
                field_type: Type::INT,
                constraints: vec![Constraint::PrimaryKey, Constraint::NotNull],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn bigint_test() {
        let fields: Vec<Field> = table! {
            id BIGINT PRIMARY_KEY NOT_NULL;
        };

        let expected = vec![
            Field {
                field_name: "id".to_string(),
                field_type: Type::BIGINT,
                constraints: vec![Constraint::PrimaryKey, Constraint::NotNull],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn float_test() {
        let fields: Vec<Field> = table! {
            price FLOAT DEFAULT(0.0);
        };

        let expected = vec![
            Field {
                field_name: "price".to_string(),
                field_type: Type::FLOAT,
                constraints: vec![Constraint::Default("0".to_string())],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn double_test() {
        let fields: Vec<Field> = table! {
            weight DOUBLE DEFAULT(0.0);
        };

        let expected = vec![
            Field {
                field_name: "weight".to_string(),
                field_type: Type::DOUBLE,
                constraints: vec![Constraint::Default("0".to_string())],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn decimal_test() {
        let fields: Vec<Field> = table! {
            amount DECIMAL(10, 2) DEFAULT("0.00");
        };

        let expected = vec![
            Field {
                field_name: "amount".to_string(),
                field_type: Type::DECIMAL(10, 2),
                constraints: vec![Constraint::Default("0.00".to_string())],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn bool_test() {
        let fields: Vec<Field> = table! {
            is_active BOOL DEFAULT("1");
        };

        let expected = vec![
            Field {
                field_name: "is_active".to_string(),
                field_type: Type::BOOL,
                constraints: vec![Constraint::Default("1".to_string())],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn date_test() {
        let fields: Vec<Field> = table! {
            start_date DATE;
        };

        let expected = vec![
            Field {
                field_name: "start_date".to_string(),
                field_type: Type::DATE,
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn datetime_test() {
        let fields: Vec<Field> = table! {
            created_at DATETIME;
        };

        let expected = vec![
            Field {
                field_name: "created_at".to_string(),
                field_type: Type::DATETIME,
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn timestamp_test() {
        let fields: Vec<Field> = table! {
            updated_at TIMESTAMP;
        };

        let expected = vec![
            Field {
                field_name: "updated_at".to_string(),
                field_type: Type::TIMESTAMP,
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn time_test() {
        let fields: Vec<Field> = table! {
            duration TIME;
        };

        let expected = vec![
            Field {
                field_name: "duration".to_string(),
                field_type: Type::TIME,
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn year_test() {
        let fields: Vec<Field> = table! {
            birth_year YEAR;
        };

        let expected = vec![
            Field {
                field_name: "birth_year".to_string(),
                field_type: Type::YEAR,
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn char_test() {
        let fields: Vec<Field> = table! {
            code CHAR(5);
        };

        let expected = vec![
            Field {
                field_name: "code".to_string(),
                field_type: Type::CHAR(5),
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn varchar_test() {
        let fields: Vec<Field> = table! {
            name VARCHAR(255);
        };

        let expected = vec![
            Field {
                field_name: "name".to_string(),
                field_type: Type::VARCHAR(255),
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn tinytext_test() {
        let fields: Vec<Field> = table! {
            description TINYTEXT(100);
        };

        let expected = vec![
            Field {
                field_name: "description".to_string(),
                field_type: Type::TINYTEXT(100),
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn text_test() {
        let fields: Vec<Field> = table! {
            content TEXT(255);
        };

        let expected = vec![
            Field {
                field_name: "content".to_string(),
                field_type: Type::TEXT(255),
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn longtext_test() {
        let fields: Vec<Field> = table! {
            bio LONGTEXT(1000);
        };

        let expected = vec![
            Field {
                field_name: "bio".to_string(),
                field_type: Type::LONGTEXT(1000),
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn binary_test() {
        let fields: Vec<Field> = table! {
            data BINARY;
        };

        let expected = vec![
            Field {
                field_name: "data".to_string(),
                field_type: Type::BINARY,
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn varbinary_test() {
        let fields: Vec<Field> = table! {
            file VARBINARY;
        };

        let expected = vec![
            Field {
                field_name: "file".to_string(),
                field_type: Type::VARBINARY,
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn enum_test() {
        let fields: Vec<Field> = table! {
            status ENUM("Active", "Inactive", "Pending");
        };

        let expected = vec![
            Field {
                field_name: "status".to_string(),
                field_type: Type::ENUM(vec![
                    String::from("Active"),
                    String::from("Inactive"),
                    String::from("Pending"),
                ]),
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }

    #[test]
    fn set_test() {
        let fields: Vec<Field> = table! {
            items SET("Sword", "Shield", "Potion", "Armor");
        };

        let expected = vec![
            Field {
                field_name: "items".to_string(),
                field_type: Type::SET(vec![
                    String::from("Sword"),
                    String::from("Shield"),
                    String::from("Potion"),
                    String::from("Armor"),
                ]),
                constraints: vec![],
            },
        ];

        assert_eq!(fields, expected);
    }



    
}