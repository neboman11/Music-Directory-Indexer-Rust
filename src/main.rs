use std::error::Error;
use std::io;
use std::fs;
use csv::Reader;
use clap::{Arg, App};

fn main() -> io::Result<()> {
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
            .short("l")
            .long("sort-album")
            .help("Sort all output by album"))
        .get_matches();

    let input_file_given = given_options.is_present("infile");

    if input_file_given
    {
        let given_data = read_given_data(given_options.value_of("infile").unwrap());
    }

    println!("Hello, world!");

    Ok(())
}

fn read_given_data(path: &str) -> Result<Vec<csv::StringRecord>, Box<dyn Error>> {
    let mut read_data = Vec::new();

    let mut rdr = Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        read_data.push(record);
    }
    Ok(read_data)
}

fn index_directory(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut found_data = Vec::new();

    for artist in fs::read_dir(path)? {
        let artist = artist?;
        if artist.file_type()?.is_dir() {
            for album in fs::read_dir(artist.file_name()) {
                let album = album?;
                if album.file_type()?.is_dir() {
                    found_data.push(album.file_name());
                }
            }
        }
    }

    Ok(found_data)
}
