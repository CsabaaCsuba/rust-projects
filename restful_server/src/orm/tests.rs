#[cfg(test)]

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
        is_good BOOL DEFAULT(1);
    };
    assert_eq!(table.fields[0].constraints.len(), 4);
    assert_eq!(table.fields[0].constraints[0], Constraint::NotNull);
    assert_eq!(table.fields[0].constraints[1], Constraint::Unique);
    assert_eq!(table.fields[0].constraints[2], Constraint::Default(DefaultValue::Int(42)));
    assert_eq!(table.fields[0].constraints[3], Constraint::PrimaryKey);

    assert_eq!(table.fields[1].constraints[0], Constraint::Default(DefaultValue::Int(1)));
}

// #[test]

