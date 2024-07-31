use mtg_database::connection::{check_connection, initialize_db};
use mtg_database::csv_entry;
//use mtg_database::{check_connection, initialize_db};
use std::{env, error::Error, ffi::OsString, process};

fn main() {
    if let Ok(connection_status) = check_connection() {
        println!("Database connection status: {:#?}", connection_status);
    } else {
        let db_connection = initialize_db();
        println!("Starting connection, {:#?}", db_connection);
    }
    if let Err(err) = run() {
        println!("{:#?}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut stored_entries: Vec<csv_entry::Entry> = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .flexible(true)
        .from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: csv_entry::Entry = result?;
        // let serialized = serde_json::to_string(&record)?;
        // println!("{:#?}", record);
        stored_entries.push(record);
    }
    Ok(())
}

// Return the first positional argument which will be the file path of the csv file
// If there are no functional arguments, this will return an error
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
