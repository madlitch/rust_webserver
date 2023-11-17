
fn update_database(p0: &String) -> Result<()> {
    todo!()
}

fn insert_into_database(p0: &String) -> Result<()> {
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

fn delete_from_database(p0: &String) -> Result<()> {
    todo!()
}


fn init_database() -> Result<()> {
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