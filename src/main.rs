use std::fs::File;
use std::error::Error;
use csv::Reader;
use clap::{Arg, App};

fn main() {
    let given_options = App::new("music_directory_indexer_rust")
        .version("1.0")
        .about("Indexes a directory with an Album-Artist folder structure.")
        .author("Michael Nesbitt")
        .arg(Arg::with_name("outfile")
            .short("o")
            .long("outfile")
            .value_name("output CSV")
            .help("The file to place the data into")
            .takes_value(true)
            .default_value("music-data.csv"))
        .arg(Arg::with_name("infile")
            .short("i")
            .long("infile")
            .value_name("input CSV")
            .help("A CSV file to compare against when indexing the directory")
            .takes_value(true))
        .arg(Arg::with_name("directory")
            .help("The directory to index the folders of")
            .required(false))
        .arg(Arg::with_name("sort_artist")
            .short("r")
            .long("sort-artist")
            .help("Sort all output by artist"))
        .arg(Arg::with_name("sort_album")
            .short("r")
            .long("sort-album")
            .help("Sort all output by album"))
        .get_matches();

    let input_file_given = given_options.is_present("infile");

    if input_file_given
    {
        let given_data = read_given_data(given_options.value_of("infile").unwrap());
    }

    println!("Hello, world!");
}

fn read_given_data(path: &str) /*-> Vec<csv::StringRecord>*/ {
    let mut readData = Vec::new();

    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        readData.push(record);
    }
}
