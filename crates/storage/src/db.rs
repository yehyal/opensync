use rusqlite::{Connection, Error};

use crate::paths;

pub fn setup() -> Result<Connection, Error> {
    let path = paths::db_path();
    let conn = rusqlite::Connection::open(path)?;

    conn.execute(
        "CREATE TABLE auth (
          id   INTEGER PRIMARY KEY,
          name TEXT NOT NULL,
          token TEXT NOT NULL
        )",
        (),
    )?;
    Ok(conn)
}
