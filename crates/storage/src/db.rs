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

    conn.execute(
        "CREATE TABLE IF NOT EXISTS devices (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          user_id TEXT NOT NULL UNIQUE,
          device_id TEXT NOT NULL,
          device_name TEXT NOT NULL,
          platform TEXT NOT NULL
        )",
        (),
    )?;

    Ok(DB { conn })
}

#[derive(Debug, Clone)]
pub struct DeviceRegistration {
    pub user_id: String,
    pub device_id: String,
    pub device_name: String,
    pub platform: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub token: String,
    pub device_id: String,
}

impl DB {
    pub fn get_auth(&self) -> Result<Option<Session>, Error> {
        let mut statement = self.conn.prepare(
            "
            SELECT a.user_id, a.name, a.email, a.token, d.device_id
            FROM auth a
            JOIN devices d ON a.user_id = d.user_id
            ORDER BY a.id DESC
            LIMIT 1
            ",
        )?;

        let mut rows = statement.query(())?;
        if let Some(row) = rows.next()? {
            return Ok(Some(Session {
                user_id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                token: row.get(3)?,
                device_id: row.get(4)?,
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

    pub fn get_device_for_user(&self, user_id: &str) -> Result<Option<DeviceRegistration>, Error> {
        let mut statement = self.conn.prepare(
            "
            SELECT user_id, device_id, device_name, platform
            FROM devices
            WHERE user_id = ?
            LIMIT 1
            ",
        )?;

        let mut rows = statement.query((user_id,))?;
        if let Some(row) = rows.next()? {
            return Ok(Some(DeviceRegistration {
                user_id: row.get(0)?,
                device_id: row.get(1)?,
                device_name: row.get(2)?,
                platform: row.get(3)?,
            }));
        }

        Ok(None)
    }

    pub fn upsert_device_for_user(&self, device: DeviceRegistration) -> Result<(), Error> {
        self.conn.execute(
            "
            INSERT INTO devices (user_id, device_id, device_name, platform)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(user_id) DO UPDATE SET
              device_id = excluded.device_id,
              device_name = excluded.device_name,
              platform = excluded.platform
            ",
            (
                device.user_id,
                device.device_id,
                device.device_name,
                device.platform,
            ),
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
