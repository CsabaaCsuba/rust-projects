#[macro_export]
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
    (@constraint AUTO_INCREMENT) => { Constraint::AutoIncrement };
    (@constraint FOREIGN_KEY($table:expr, $field:expr)) => {
        Constraint::ForeignKey { table: $table.to_string(), field: $field.to_string() }
    };

    // Internal matcher to infer the type of DefaultValue
    // Special handling for bool, to avoid treating it as a numeric type
    (@default_value $val:expr) => {
        if let Some(val) = table!(@as_int $val) {
            DefaultValue::Int(val)
        } else if let Some(val) = table!(@as_float $val) {
            DefaultValue::Float(val)
        } else {
            DefaultValue::String($val.to_string())
        }
    };

    // Matchers to try to convert the value to specific types
    // Handle boolean explicitly
    (@as_int $val:expr) => {
        if let Ok(val) = ($val as i64).to_string().parse::<i64>() { Some(val) } else { None }
    };
    // Handle float parsing explicitly
    (@as_float $val:expr) => {
        if let Ok(val) = ($val as f64).to_string().parse::<f64>() { Some(val) } else { None }
    };
}

pub(crate) use table;