use anyhow::Result;
use csv::ReaderBuilder;
use rusqlite::Connection;
use std::io::{self, Read};

fn main() -> Result<()> {
    // We are gathered here today, to import a CSV to Sqlite
    // When importing to SQL for the first time, knowing headers ahead of time is important.
    // Since reading CSVs is tricky, using the csv crate to read and interpret headers vs rows is the right thing to do.

    let conn = Connection::open_in_memory()?;
    // let mut bytes: Vec<u8> = Vec::new();
    // io::stdin().read_to_end(&mut bytes)?;
    let mut csv_reader = ReaderBuilder::new().from_reader(io::stdin());
    let cloned = csv_reader.headers().cloned()?;
    let headers: Vec<&str> = cloned.iter().collect();

    let create_table = format!(
        r#"
        CREATE TABLE testing ({})
        "#,
        headers.join(",")
    );

    conn.execute(create_table.as_str(), ())?;

    let mut stmt = conn.prepare("SELECT * FROM testing")?;
    let mut rows = stmt.query([])?;

    let prepared_insert = r#"
    INSERT INTO testing ( col1 ) VALUES ( 1 )
    "#;

    let mut prepared_insert = conn.prepare_cached(prepared_insert)?;
    let row = csv_reader.records().next().unwrap()?;
    prepared_insert.execute([])?;
    // prepared_insert.execute([rusqlite::named_params! {
    //     ":cols": headers.join(","),
    //     ":vals": row.iter().collect::<Vec<&str>>().join(",")
    // }])?;

    // A simple insert works, but the insert I want with world replacements does not.
    // I think it's because I'm being too liberal about prepared statements parameter replacements.

    Ok(())
}
