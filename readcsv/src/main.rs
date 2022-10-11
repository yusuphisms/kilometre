use csv::{Reader, ReaderBuilder, StringRecord};
use std::io::Read;
use std::iter::Iterator;
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    simple_reading_from_stdin()
}

fn simple_reading_from_stdin() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut v)?;
    let mut reader = ReaderBuilder::new().from_reader(io::Cursor::new(v));
    reader.headers()?; // skip the header row by reading it first
    let og_record_pos = reader.position().clone(); // get the position; this is the correct first record after headers

    // Reading by records

    let result = reader.records().next().unwrap();
    let record = result?;
    println!("{:?}", record.position());
    println!("{:?}", record);
    println!("{:?}", record.len());

    // Reading by fields
    reader.seek(og_record_pos)?;
    let mut r_n_f = RecordsAndFields::new(reader);
    let field_record = r_n_f.field_next().unwrap()?;
    println!("{:?}", field_record);
    println!("{:?}", field_record.len());

    let next_field_record = r_n_f.field_next().unwrap()?;
    println!("{:?}", next_field_record);
    println!("{:?}", next_field_record.len());

    // OK -- this works but isn't the API I was looking for; I want to be able to put this in a `for` loop NEXT

    Ok(())
}

struct RecordsAndFields<R> {
    reader: Reader<R>,
    current_field_iteration: usize,
}

impl<'i, R: io::Read + io::Seek> RecordsAndFields<R> {
    fn new(reader: Reader<R>) -> RecordsAndFields<R> {
        Self {
            reader,
            current_field_iteration: 0,
        }
    }

    // TODO: Kind of hit a wall with trying to read by field here without
    // elaborate internals work.
    fn field_next(&mut self) -> Option<Result<StringRecord, csv::Error>> {
        // Go through each record, pick out the current field of interest's value
        // Best way to do this is to track the index of the field
        let mut stringfield = StringRecord::new();
        // Probably want to increment after iteration is complete but I was fighting the borrow checker for too long
        // Apparently I really need to review Iter / IntoIterator trait implementation, like bad.
        let pos = self.current_field_iteration;
        let starting_reader_position = self.reader.position().clone();
        for result in self.iter_mut() {
            let record = result.ok()?;
            stringfield.push_field(record.get(pos)?);
        }
        self.current_field_iteration += 1;
        self.reader.seek(starting_reader_position).ok()?;
        Some(Ok(stringfield))
    }

    fn iter_mut(&'i mut self) -> RNFIter<'i, R> {
        RNFIter(self)
    }
}

struct RNFIter<'j, R: 'j>(&'j mut RecordsAndFields<R>);

impl<'i, R: io::Read> Iterator for RNFIter<'i, R> {
    type Item = Result<StringRecord, csv::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = StringRecord::new();
        match self.0.reader.read_record(&mut s) {
            Ok(true) => Some(Ok(s)),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

// impl<'i, R: io::Read> IntoIterator for RecordsAndFields<'i, R> {
//     type Item = &'i Self;

//     type IntoIter = Self;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
