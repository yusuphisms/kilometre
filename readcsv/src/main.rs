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

    // Reading by records

    // for result in reader.records() {
    //     let record = result?;
    //     println!("{:?}", record);
    //     println!("{:?}", record.get(0));
    // }

    // Reading by fields
    // let mut r_n_f = RecordsAndFields::new(&mut reader);
    // let field_record = r_n_f.field_next().unwrap()?;
    // println!("{:?}", field_record);
    // println!("{:?}", field_record.get(0));
    // Problem! The reader can only be read once, so we can't iterate to get the next field if we have to read all records to read the first one...
    // Looks like there is a Seek implementation, so this might just have been a limitation of using io::stdin() directly.
    // I switched to using a Vec<u8> buffer and io::Cursor

    // Back to basics:
    test_flatten(); // flatten can work, and is getting closer to a solution like ndarray

    let all_records: Vec<Result<StringRecord, csv::Error>> = reader.records().collect();
    let unwrapped_records: Vec<StringRecord> = all_records
        .iter()
        .map(|r| {
            if r.is_err() {
                StringRecord::new()
            } else {
                r.as_ref().ok().unwrap().to_owned()
            }
        })
        .collect();
    let col1: Vec<&str> = unwrapped_records
        .iter()
        .map(|x| x.get(0).unwrap())
        .collect();
    println!("{:?}", col1);

    let col2: Vec<&str> = unwrapped_records
        .iter()
        .map(|x| x.get(3).unwrap())
        .collect();
    println!("{:?}", col2);

    println!("{:?}", reader.headers());
    Ok(())
}

struct RecordsAndFields<'i, R: 'i> {
    reader: &'i mut Reader<R>,
    current_field_iteration: usize,
}

impl<'i, R: io::Read> RecordsAndFields<'i, R> {
    fn new(reader: &'i mut Reader<R>) -> RecordsAndFields<'i, R> {
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
        self.current_field_iteration += 1;
        for result in self {
            let record = result.ok()?;
            stringfield.push_field(record.get(pos)?);
        }
        Some(Ok(stringfield))
    }
}

impl<'i, R: io::Read> Iterator for RecordsAndFields<'i, R> {
    type Item = Result<StringRecord, csv::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = StringRecord::new();
        match self.reader.read_record(&mut s) {
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

fn test_flatten() {
    let v = vec![vec!["1", "2", "3"], vec!["4", "5", "6"]];
    let rez: Vec<Option<&&str>> = v.iter().map(|x| x.get(0)).collect();
    println!("{:?}", rez);
}
