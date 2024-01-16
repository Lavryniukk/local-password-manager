use rusqlite::{params, Connection, Error, Result};
pub struct PasswordDatabase {
    pub(crate) connection: Connection,
}

#[derive(Debug)]
pub struct Password {
    _id: i32,
    _name: String,
    _password: String,
}

pub trait Database {
    fn connect() -> Result<PasswordDatabase>;
    fn add_password(&self, name: &str, password: &str) -> Result<usize, Error>;
    fn get_password(&self, name: &str) -> Result<Option<String>, Error>;
    // fn delete_password(&self, name: &str) -> Result<()>;
    fn get_all_password_names(&self) -> Result<Vec<String>>;
    fn get_all_passwords(&self) -> Result<Vec<Password>>;
}

impl Database for PasswordDatabase {
    fn connect() -> Result<PasswordDatabase> {
        let connection = Connection::open("src/db/password_manager_db")?;
        Ok(PasswordDatabase { connection })
    }
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
