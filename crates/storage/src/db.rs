use rusqlite::{Connection, Error};

use crate::paths;

#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub user_id: String,
    pub name: String,
    pub email: String,
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
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          user_id TEXT NOT NULL,
          name TEXT NOT NULL,
          email TEXT NOT NULL,
          token TEXT NOT NULL
        )",
        (),
    )?;

    ensure_auth_column(&conn, "user_id", "TEXT NOT NULL DEFAULT ''")?;
    ensure_auth_column(&conn, "email", "TEXT NOT NULL DEFAULT ''")?;

    Ok(DB { conn })
}

impl DB {
    pub fn get_auth(&self) -> Result<Option<LoginResponse>, Error> {
        let mut statement = self.conn.prepare(
            "
            SELECT user_id, name, email, token
            FROM auth
            ORDER BY id DESC
            LIMIT 1
            ",
        )?;

        let mut rows = statement.query(())?;
        if let Some(row) = rows.next()? {
            return Ok(Some(LoginResponse {
                user_id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                token: row.get(3)?,
            }));
        }

        Ok(None)
    }

    pub fn insert(&self, args: LoginResponse) -> Result<(), Error> {
        self.conn.execute("DELETE FROM auth", ())?;
        self.conn.execute(
            "
      INSERT INTO auth (user_id, name, email, token)
      VALUES (?, ?, ?, ?)
      ",
            (args.user_id, args.name, args.email, args.token),
        )?;
        Ok(())
    }
}

fn ensure_auth_column(conn: &Connection, column: &str, definition: &str) -> Result<(), Error> {
    let mut statement = conn.prepare("PRAGMA table_info(auth)")?;
    let columns = statement.query_map((), |row| row.get::<_, String>(1))?;

    for existing_column in columns {
        if existing_column? == column {
            return Ok(());
        }
    }

    conn.execute(
        &format!("ALTER TABLE auth ADD COLUMN {column} {definition}"),
        (),
    )?;

    Ok(())
}
