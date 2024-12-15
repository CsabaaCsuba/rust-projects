use std::{env, error, fmt, fs::{self, File}, io::{self, Write}};



pub fn generate(output_dir: String) -> Result<(), io::Error> {
    defineAst(output_dir, "Expr".to_string(), vec![
        "Binary     : Expr left, Token operator, Expr right".to_string(),
        "Grouping   : Expr Expression".to_string(),
        "Literal    : Object value".to_string(),
        "Unary      : Token operator, Expr right".to_string()
    ])?;
    Ok(())
}

fn defineAst(output_dir: String, base_name: String, types: Vec<String>) -> Result<(), io::Error> {
    let mut file = File::create(format!("{output_dir}/{base_name}.rs").as_str())?;
    file.write(format!("struct {base_name}").as_bytes())?;

    for t in types {
        let type_split: Vec<&str> = t.split(':').into_iter().collect();
        let class_name = type_split[0].trim();
        let fields = type_split[1].trim();

        defineType(&mut file, &base_name, class_name, fields)?;
    }

    Ok(())
}

fn defineType(file: &mut File, base_name: &str, class_name: &str, fields: &str) -> Result<(), io::Error> {
    file.write(format!("{{{}}}").as_bytes());

    Ok(())
}