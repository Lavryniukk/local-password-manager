use rusqlite::{params, Connection, Error, Result};

#[derive(Debug)]
struct Password {
    _id: i32,
    _name: String,
    _password: String,
}

trait Database {
    fn add_password(&self, name: &str, password: &str) -> Result<usize, Error>;
    fn get_password(&self, name: &str) -> Result<Option<String>, Error>;
    // fn delete_password(&self, name: &str) -> Result<()>;
    fn get_all_password_names(&self) -> Result<Vec<String>>;
    fn get_all_passwords(&self) -> Result<Vec<Password>>;
}

struct PasswordDatabase {
    connection: Connection,
}

impl Database for PasswordDatabase {
    fn get_all_password_names(&self) -> Result<Vec<String>> {
        let mut statement = self.connection.prepare("SELECT name FROM password")?;
        let password_iter = statement.query_map([], |row| row.get(0))?;
        let password_vec: Result<Vec<String>> = password_iter.collect();
        password_vec
    }

    fn add_password(&self, name: &str, password: &str) -> Result<usize, Error> {
        self.connection.execute(
            "INSERT INTO password (name, password) VALUES (?1, ?2)",
            params![name, password],
        )
    }

    fn get_password(&self, name: &str) -> Result<Option<String>, Error> {
        self.connection.query_row(
            "SELECT password FROM password WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )
    }

    fn get_all_passwords(&self) -> Result<Vec<Password>> {
        let mut statement = self.connection.prepare("SELECT * FROM password")?;
        let password_iter = statement.query_map([], |row| {
            Ok(Password {
                _id: row.get(0)?,
                _name: row.get(1)?,
                _password: row.get(2)?,
            })
        })?;

        let password_vec: Result<Vec<Password>> = password_iter.collect();
        password_vec
    }
}

fn main() {
    let connection = Connection::open("src/db/password_manager_db").unwrap();
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS password (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  password        TEXT NOT NULL
                  )",
            [],
        )
        .expect("Could not create table");

    let password_db = PasswordDatabase { connection };
    println!("Welcome, sir. What would you like to do? Options: add, get, get_all, exit");
    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "add" => {
                println!("What is the name of the password?");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();
                println!("What is the password?");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).unwrap();
                let password = password.trim();
                password_db.add_password(name, password).unwrap();
                println!("Password added");
                ()
            }
            "get" => {
                println!("What is the name of the password?");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();
                let password = match password_db.get_password(name) {
                    Ok(password) => password,
                    Err(_) => {
                        println!("Password not found");
                        continue;
                    }
                };
                match password {
                    Some(password) => println!("Password: {}", password),
                    None => println!("Password not found"),
                }
                ()
            }
            "get_all" => {
                let names = password_db.get_all_password_names().unwrap();
                for name in names {
                    println!("Name: {}", name);
                }
            }
            "exit" => break,
            _ => println!("Invalid input"),
        }
    }
}
