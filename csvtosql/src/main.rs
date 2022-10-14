use rusqlite::vtab::csvtab;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // We are gathered here today, to import a CSV to Sqlite
    // Right off the bat, we have two options we can consider: virtual table or not.
    // The Virtual Table route: https://www.sqlite.org/csv.html
    let conn = Connection::open_in_memory()?;
    csvtab::load_module(&conn)?;
    let schema = "
        CREATE VIRTUAL TABLE my_csv_data
        USING csv(filename = 'testfile.csv')
    ";
    // the specs in the link above say that data is an available keyword where you can just put direct text in, but rusqlite's csvtab does not support that yet.
    // And I can't figure out what the correct way is to tell it the path of the filename. Kinda crazy.
    conn.execute_batch(schema)?;

    Ok(())
}
