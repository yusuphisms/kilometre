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
    let field_record = r_n_f.iter_mut().next().unwrap()?;
    println!("{:?}", field_record);
    println!("{:?}", field_record.len());

    let next_field_record = r_n_f.iter_mut().next().unwrap()?;
    println!("{:?}", next_field_record);
    println!("{:?}", next_field_record.len());

    // Can use it in a for loop now
    // Implementing Iter makes sense when you've done it, but can be hard to reason about without consistent practice.
    // I think it was also harder in this case because there was an added complexity in creating a wrapper struct.
    for realz in r_n_f.iter_mut() {
        let realsies = realz?;
        println!("{:?}", realsies);
        println!("{:?}", realsies.len());
    }

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

    fn iter_mut(&'i mut self) -> RNFIter<'i, R> {
        RNFIter(self)
    }
}

struct RNFIter<'j, R: 'j>(&'j mut RecordsAndFields<R>);

impl<'i, R: io::Read + io::Seek> Iterator for RNFIter<'i, R> {
    type Item = Result<StringRecord, csv::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut stringfield = StringRecord::new();
        let field_index = self.0.current_field_iteration;
        let starting_reader_position = self.0.reader.position().clone();
        for record in self.0.reader.records() {
            let unwrapped_record = record.ok()?;
            stringfield.push_field(unwrapped_record.get(field_index)?);
        }
        self.0.current_field_iteration += 1;
        self.0.reader.seek(starting_reader_position).ok()?;
        Some(Ok(stringfield))
    }
}
