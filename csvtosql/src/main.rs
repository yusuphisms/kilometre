use anyhow::Result;
use csv::ReaderBuilder;
use rusqlite::Connection;
use std::io;

fn main() -> Result<()> {
    // We are gathered here today, to import a CSV to Sqlite
    // When importing to SQL for the first time, knowing headers ahead of time is important.
    // Since reading CSVs is tricky, using the csv crate to read and interpret headers vs rows is the right thing to do.

    let conn = Connection::open_in_memory()?;

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

    let row = csv_reader.records().next().unwrap()?;
    let prepared_insert = format!(
        r#"
        INSERT INTO testing ( {} ) VALUES ( {} )
        "#,
        headers.join(","), // it looks like you can't dynamically populate columns using prepared statements
        ["?"].repeat(headers.len()).join(",")
    );

    let mut prepared_insert = conn.prepare_cached(prepared_insert.as_str())?;

    let vals_string = row
        .iter()
        .map(|f| format!("'{}'", f))
        .collect::<Vec<String>>();
    let vals = vals_string
        .iter()
        .map(String::as_str)
        .collect::<Vec<&str>>();
    prepared_insert.execute::<&[&str; 5]>(vals[..].try_into()?)?;
    // So try_into is nice here, but the length of the columns will be dynamic in the future. So this is kind of a bummer.
    // I think it will require thinking more about what information would I have at runtime, what info would I need to request, etc.

    Ok(())
}
