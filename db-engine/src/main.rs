mod command_parser;
use command_parser::syntax_checker;
// mod engine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_table() {
        let table1 =
            "CREATE TABLE `t1` ( id INT NOT NULL AUTO_INCREMENT , name TEXT UNIQUE ) ;".to_string();
        let table1_tokens = command_parser::parse_command(&table1);
        assert_eq!(true, syntax_checker::validate_syntax(&table1_tokens));

        let table2 =
            "CREATE TABLE `t1` ( id INT NULL AUTO_INCREMENT , name TEXT UNIQUE ) ;".to_string();
        let table2_tokens = command_parser::parse_command(&table2);
        assert_eq!(false, syntax_checker::validate_syntax(&table2_tokens));

        let table3 =
            "CREATE TABLE `t1` ( foreign_id INT , name TEXT UNIQUE , FOREIGN KEY ( foreign_id ) REFERENCES foreign_table ( id ) ON DELETE CASCADE ) ;".to_string();
        let table3_tokens = command_parser::parse_command(&table3);
        assert_eq!(true, syntax_checker::validate_syntax(&table3_tokens));

        //must be false, because after FOREIGN KEY, nothing can be used
        let table4 =
            "CREATE TABLE `t1` ( foreign_id INT , name TEXT UNIQUE , FOREIGN KEY ( foreign_id ) REFERENCES foreign_table ( id ) ON DELETE SET NULL AUTO_INCREMENT ) ;".to_string();
        let table4_tokens = command_parser::parse_command(&table4);
        assert_eq!(false, syntax_checker::validate_syntax(&table4_tokens));

        //should fail, because of NOT NULL
        let table5 =
            "CREATE TABLE `t1` ( foreign_id INT , table2_id INT name TEXT UNIQUE , PRIMARY KEY ( foreign_id , table2_id ) NOT NULL ) ;".to_string();
        let table5_tokens = command_parser::parse_command(&table5);
        assert_eq!(false, syntax_checker::validate_syntax(&table5_tokens));

        //shouldn't fail
        let table6 =
            "CREATE TABLE `t1` ( foreign_id INT , table2_id INT name TEXT UNIQUE , PRIMARY KEY ( foreign_id , table2_id ) ) ;".to_string();
        let table6_tokens = command_parser::parse_command(&table6);
        assert_eq!(true, syntax_checker::validate_syntax(&table6_tokens));


    }

    #[test]
    fn create_db() {
        let db1 = "CREATE DATABASE `db1` CHARACTER SET \"UTF8_HUNGARIAN_CI\" COLLATE \"UTF8\" ;"
            .to_string();
        let db1_tokens = command_parser::parse_command(&db1);
        assert_eq!(true, syntax_checker::validate_syntax(&db1_tokens));

        let false_db = "CREATE DB `bruh` CHARACTER \"a\" ;".to_string();
        let false_tokens = command_parser::parse_command(&false_db);
        assert_eq!(false, syntax_checker::validate_syntax(&false_tokens));

        let false_db2 = "CREATE `bruh` CHARACTER SET \"a\" ;".to_string();
        let false_tokens2 = command_parser::parse_command(&false_db2);
        assert_eq!(false, syntax_checker::validate_syntax(&false_tokens2));
    }
}

fn main() {
    command_parser::read_command_loop();
}
