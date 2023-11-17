use rusqlite::{Connection, Result, params, Row};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    id: i32,
    title: String,
    content: String,
}

pub fn init_database() -> Result<()> {
    let conn = Connection::open("database.sqlite").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
             id INTEGER PRIMARY KEY,
             title TEXT NOT NULL,
             content TEXT
         )",
        [],
    ).unwrap();
    Ok(())
}

pub fn get_from_database(id: &str) -> Result<Note> {
    let conn = Connection::open("database.sqlite").unwrap();
    let mut query = conn.prepare(&*format!("SELECT id, title, content FROM notes WHERE id = ?;"))?;
    let note = query.query_row(params![id], |row: &Row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
        })
    })?;
    Ok(note)
}


pub fn update_database(id: &str, title: &str, content: &str) -> Result<()> {
    let conn = Connection::open("database.sqlite").unwrap();
    let query = &*format!("UPDATE notes SET title = ?, content = ? WHERE id = ?;");
    conn.execute(query, [title, content, id]).unwrap();
    Ok(())
}

pub fn insert_into_database(title: &str, content: &str) -> Result<()> {
    let conn = Connection::open("database.sqlite").unwrap();
    let query = &*format!("INSERT INTO notes (title, content) VALUES (?, ?);");
    conn.execute(query, [title, content]).unwrap();
    Ok(())
}

pub fn delete_from_database(id: &str) -> Result<()> {
    let conn = Connection::open("database.sqlite").unwrap();
    let query = &*format!("DELETE FROM notes WHERE id = ?;");
    conn.execute(query, [id]).unwrap();
    Ok(())
}


