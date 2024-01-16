mod db;
mod user_input_handler;
use crate::db::database::{Database, PasswordDatabase};
use crate::user_input_handler::handle_user_input;
fn check_or_set_master_password(database: &PasswordDatabase) {
    let result = database.connection.query_row(
        "SELECT password FROM password WHERE name = ?1",
        ["master"],
        |row| Ok(row.get::<_, String>(0)?),
    );

    match result {
        Ok(password) => {
            println!("Insert your master password: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input != password {
                println!("Wrong password");
                return;
            }
        }
        Err(_) => {
            println!(
                "You are new here, please set a master password so I could identify you later"
            );
            println!("Insert your master password: ");

            let mut new_master_password = String::new();
            std::io::stdin()
                .read_line(&mut new_master_password)
                .unwrap();
            let new_master_password = new_master_password.trim();

            database
                .add_password("master", new_master_password)
                .expect("Could not set master password");
            println!("Master password set!")
        }
    }
}

fn create_password_table(database: &PasswordDatabase) {
    database
        .connection
        .execute(
            "CREATE TABLE IF NOT EXISTS password (
                      id              INTEGER PRIMARY KEY,
                      name            TEXT NOT NULL,
                      password        TEXT NOT NULL
                      )",
            [],
        )
        .expect("Could not create table");
}

fn main() {
    let connection = PasswordDatabase::connect().unwrap();

    check_or_set_master_password(&connection);

    create_password_table(&connection);

    println!("Welcome, sir. What would you like to do? Options: add, get, get_all, exit");

    let mut input = String::new();

    handle_user_input(&mut input, &connection)
}
