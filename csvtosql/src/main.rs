use anyhow::Result;
use csv::ReaderBuilder;
use rusqlite::Connection;
use std::io;

fn main() -> Result<()> {
    // We are gathered here today, to import a CSV to Sqlite
    // When importing to SQL for the first time, knowing headers ahead of time is important.
    // Since reading CSVs is tricky, using the csv crate to read and interpret headers vs rows is the right thing to do.

    let mut conn = Connection::open_in_memory()?;

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

    let prepared_insert = format!(
        r#"
        INSERT INTO testing ( {} ) VALUES ( {} )
        "#,
        headers.join(","), // it looks like you can't dynamically populate columns using prepared statements
        ["?"].repeat(headers.len()).join(",")
    );

    // re-using the prepared statement
    // Great read on insert performance improvements -- https://stackoverflow.com/questions/1711631/improve-insert-per-second-performance-of-sqlite
    // let's try adding a transaction
    let transaction = conn.transaction()?;
    // oh this is fun and interesting - switch from using conn to using transaction
    // so much to learn about SQL!

    for record in csv_reader.records() {
        let mut prepared_insert = transaction.prepare_cached(prepared_insert.as_str())?;
        let row = record?;
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
    }
    transaction.commit()?;

    Ok(())
}
