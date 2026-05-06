use rusqlite::{Connection, Error};

use crate::paths;

pub struct LoginResponse {
    pub user_id: String,
    pub token: String,
}

pub struct DB {
    conn: Connection,
}

pub fn setup() -> Result<DB, Error> {
    let path = paths::db_path();
    let conn = rusqlite::Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS auth (
          id   INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL,
          token TEXT NOT NULL
        )",
        (),
    )?;
    Ok(DB { conn })
}
impl DB {
    pub fn insert(&self, args: LoginResponse) -> Result<(), Error> {
        self.conn.execute(
            "
      INSERT INTO auth (name, token)
      VALUES (?,?)
      ",
            (args.user_id, args.token),
        )?;
        Ok(())
    }
}
