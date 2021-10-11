use std::collections::HashMap;

// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

use std::collections::HashMap;
use std::io::Read;
use std::thiserror::Error;

#[derive(Debug)]
struct Record {
    id: i64,
    name: String,
    email: Option<String>
}

#[derive(Debug)]
struct Records {
    inner: HashMap<i64, Record>
}

impl Records {
    // Self is records
    fn new() -> Self {
        // create the records structure by using Self
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record);
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("id must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),
    #[error("empty record")]
    EmptyRecord,
    #[error("missing field: {0}")]
    MissingField(String)
}
// parse_record takes a record,
fn parse_record(record: &str) -> Result<Record, ParseError> {
    // ',' in our data we have each field separated by a comma
    // then going to collect all those pieces of data into a vector of &str
    // that we have a variable name filed
    let fields: Vec<&str> = record.split(',').collect();
    // trying to get the first field, which is field number zero
    // when working with vectors or most things with programming, the indexes always start at zero
    let id = match fields.get(0) {
        // here we have id, then try to convert the string representation
        // of the id into a number we can work with
        // mean: convert a string into a number and the radix part is 10
        // we're using base 10, like just decimal numbers
        // ?: if it failds, which it will in many cases, it's automatically going to return a pass error
        //and the variant will be the invalid ID, because this function returns a ParseError
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };
    let name = match fields.get(1).filter(|name| **name != "") {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };
    let email = fields.get(2).map(|email| email.to_string()).filter(|email| email != "");
    Ok(Record{id, name, email})


}

fn parse_records(records: String, verbose: bool) -> Records {
    let mut recs = Records::new();
    // enumerate: provides us an index number of how many times we've
    // gone through this for and loop, and it just returns a tuple
    // the tuple is the current iteation number that we're on and then
    // the data that we're iterating upon
    // index: num
    for (num, record) in records.split('\n').enumerate() {
        // ignore blank line
        if record != "" {
            match parse_records(record) {
                Ok(rec) => recs.add(rec),
                Err(e) => {
                    if verbose {
                        println!(
                            "error on line number {}: {}\n > \"{}\"\n",
                            // line number start 0 (index)
                            num + 1,
                            e,
                            // attempted to be passed but failed, and that would be
                            // stored here on its own line
                            record
                        );
                    }
                }
            }
        }
    }
    return recs
}

// PathBuf: used to work with paths to files, which is exactly what we need when we're working
// with file data,
// since it saves on our disk, it provides easy options for adding diẻctories, file extensions and file
// return io results because we're going to be working with files and lots of thing can go wrong
// and if everything goes right, we'll just return our records structure that we created earlier
fn load_records(file_name: PathBuf, verbose: bool) -> std::io::Result<Records> {
    let mut file = File::open(file_name)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_records(buffer, verbose))
}

fn main() {}
