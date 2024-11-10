use postgres::{Client, NoTls, Error};

pub fn test_query() -> Result<(), Error> {
    // Set up connection parameters
    let host = "localhost";         // Hostname or IP address
    let user = "postgres";          // Database user
    let password = "password";      // Database password
    let dbname = "mydatabase";      // Database name
    let connection_string = format!("host={} user={} password={} dbname={}", host, user, password, dbname);

    // Connect to the database
    let mut client = Client::connect(&connection_string, NoTls)?;

    // Execute a sample query (e.g., fetch rows from a table)
    for row in client.query("SELECT id, name FROM mytable", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        println!("ID: {}, Name: {}", id, name);
    }

    Ok(())
}

pub fn create_database(db_name: &str) -> Result<(), Error> {
    // Connection string for the default "postgres" database
    let connection_string = "host=localhost user=postgres password=Pass1234 dbname=postgres";

    // Connect to the default "postgres" database
    let mut client = Client::connect(connection_string, NoTls)?;

    // Create a new database using the provided db_name
    let create_db_query = format!("CREATE DATABASE {}", db_name);
    client.batch_execute(&create_db_query)?;

    println!("Database '{}' created successfully.", db_name);
    Ok(())
}