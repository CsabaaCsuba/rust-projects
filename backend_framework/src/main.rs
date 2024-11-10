mod db_connection;

fn main() {
    println!("Hello, world!");
    let _ = db_connection::create_database("halo");
    let _ = db_connection::test_query();
}

