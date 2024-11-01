mod command_parser;
#[allow(unused_imports)]
use command_parser::{syntax_checker, tokenizer, tokens};
use tokens::{StringWrapper, Token};
// mod engine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_table() {
        let table1 =
            "CREATE TABLE `t1` ( id INT NOT NULL AUTO_INCREMENT , name TEXT UNIQUE ) ;".to_string();
        let table1_tokens = tokenizer::parse_command(&table1);
        assert_eq!(true, syntax_checker::validate_syntax(&table1_tokens));

        let table2 =
            "CREATE TABLE `t1` ( id INT NULL AUTO_INCREMENT , name TEXT UNIQUE ) ;".to_string();
        let table2_tokens = tokenizer::parse_command(&table2);
        assert_eq!(false, syntax_checker::validate_syntax(&table2_tokens));

        let table3 =
            "CREATE TABLE `t1` ( foreign_id INT , name TEXT UNIQUE , FOREIGN KEY ( foreign_id ) REFERENCES foreign_table ( id ) ON DELETE CASCADE ) ;".to_string();
        let table3_tokens = tokenizer::parse_command(&table3);
        assert_eq!(true, syntax_checker::validate_syntax(&table3_tokens));

        //must be false, because after FOREIGN KEY, nothing can be used
        let table4 =
            "CREATE TABLE `t1` ( foreign_id INT , name TEXT UNIQUE , FOREIGN KEY ( foreign_id ) REFERENCES foreign_table ( id ) ON DELETE SET NULL AUTO_INCREMENT ) ;".to_string();
        let table4_tokens = tokenizer::parse_command(&table4);
        assert_eq!(false, syntax_checker::validate_syntax(&table4_tokens));

        //should fail, because of NOT NULL
        let table5 =
            "CREATE TABLE `t1` ( foreign_id INT , table2_id INT name TEXT UNIQUE , PRIMARY KEY ( foreign_id , table2_id ) NOT NULL ) ;".to_string();
        let table5_tokens = tokenizer::parse_command(&table5);
        assert_eq!(false, syntax_checker::validate_syntax(&table5_tokens));

        //shouldn't fail
        let table6 =
            "CREATE TABLE `t1` ( foreign_id INT , table2_id INT name TEXT UNIQUE , PRIMARY KEY ( foreign_id , table2_id ) ) ;".to_string();
        let table6_tokens = tokenizer::parse_command(&table6);
        assert_eq!(true, syntax_checker::validate_syntax(&table6_tokens));
    }

    #[test]
    fn create_db() {
        let db1 = "CREATE DATABASE `db1` CHARACTER SET \"UTF8_HUNGARIAN_CI\" COLLATE \"UTF8\" ;"
            .to_string();
        let db1_tokens = tokenizer::parse_command(&db1);
        assert_eq!(true, syntax_checker::validate_syntax(&db1_tokens));

        let false_db = "CREATE DB `bruh` CHARACTER \"a\" ;".to_string();
        let false_tokens = tokenizer::parse_command(&false_db);
        assert_eq!(false, syntax_checker::validate_syntax(&false_tokens));

        let false_db2 = "CREATE `bruh` CHARACTER SET \"a\" ;".to_string();
        let false_tokens2 = tokenizer::parse_command(&false_db2);
        assert_eq!(false, syntax_checker::validate_syntax(&false_tokens2));
    }

    #[test]
    fn insert_into() {
        let insert1 = "INSERT INTO `bruh` VALUES (\"Marcos\", 18, false);".to_string();
        let insert1_tokens = tokenizer::parse_command(&insert1);
        //just to check if there are no more parser bugs
        let expected_result = vec![
            Token::INSERT,
            Token::INTO,
            Token::NAME_UNIT(StringWrapper::Data(String::from("bruh"))),
            Token::VALUES,
            Token::OPEN_PARENTHESIS,
            Token::STRING_VALUE(StringWrapper::Data(String::from("Marcos"))),
            Token::COMMA,
            Token::INT_VALUE(18),
            Token::COMMA,
            Token::BOOL_VALUE(false),
            Token::CLOSING_PARENTHESIS,
            Token::SEMICOLON,
        ];
        assert_eq!(expected_result, insert1_tokens);
        // assert_eq!(true, syntax_checker::validate_syntax(&insert1_tokens));

        let insert2 = "INSERT INTO `bruh` VALUES (\"Marcos\", 18, false), (\"Alexos\", 20, true);"
            .to_string();
        let insert2_tokens = tokenizer::parse_command(&insert2);
        assert_eq!(true, syntax_checker::validate_syntax(&insert2_tokens));

        //due to the differing amount of fields from the first value, it is going to spit an error
        let insert3 = "INSERT INTO `bruh` VALUES (\"Marcos\", 18, false), (\"Alexos\", 20, true), (\"Jack\", \"corruptdata\", 19, true);".to_string();
        let insert3_tokens = tokenizer::parse_command(&insert3);
        assert_eq!(false, syntax_checker::validate_syntax(&insert3_tokens));

        let insert4 =
            "INSERT INTO `bruh` VALUES (\"Marcos\", 18, false), (\"Alexos\", 20, true), ;"
                .to_string();
        let insert4_tokens = tokenizer::parse_command(&insert4);
        assert_eq!(false, syntax_checker::validate_syntax(&insert4_tokens));
    }

    #[test]
    fn select_from() {
        // SELECT -> COLUMNS | * -> FROM `table_name` -> WHERE (optional) -> AND / OR (if there's an other criteria)
        // -> GROUP BY (optional) -> HAVING (optional) -> ORDER BY (optional) -> LIMIT (optional);

        //ORDER BY intcolumn LIMIT 3;
        let select = "SELECT column1, intcolumn FROM `my_table` WHERE intcolumn<3;".to_string();
        let select_tokens = tokenizer::parse_command(&select);
        assert_eq!(true, syntax_checker::validate_syntax(&select_tokens));

        let select = "SELECT column1, intcolumn FROM `my_table` WHERE intcolumn<3 AND (column1 LIKE \"halo\" OR column1 LIKE \"SZIA\");".to_string();
        let select_tokens = tokenizer::parse_command(&select);
        assert_eq!(true, syntax_checker::validate_syntax(&select_tokens));
    }
}

fn main() {
    command_parser::read_command_loop();
}
