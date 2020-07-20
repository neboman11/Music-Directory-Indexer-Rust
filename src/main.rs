use std::error::Error;
use std::io;
use std::fs;
use csv::{Reader, Writer, StringRecord};
use clap::{Arg, App};

fn main() -> io::Result<()> {
    // Use clap to parse the command line
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
            .required(false)
            .default_value("."))
        .arg(Arg::with_name("sort_artist")
            .short("r")
            .long("sort-artist")
            .help("Sort all output by artist"))
        .arg(Arg::with_name("sort_album")
            .short("l")
            .long("sort-album")
            .help("Sort all output by album"))
        .get_matches();

    // Check if an input file was given
    let input_file_given = given_options.is_present("infile");
    let mut given_data = Vec::new();    // A vector containing the data held in the input file

    // If the user provided an input file
    if input_file_given
    {
        given_data = read_given_data(given_options.value_of("infile").unwrap()).unwrap();
    }

    let mut found_data = index_directory(given_options.value_of("directory").unwrap()).unwrap();

    // Sort the data first
    if given_options.is_present("sort_artist") {
        if input_file_given {
            // Sort given_data
            given_data.sort_by(|a, b| a.get(0).unwrap().partial_cmp(b.get(0).unwrap()).unwrap());
        }

        // Sort found_data
        found_data.sort_by(|a, b| a.get(0).unwrap().partial_cmp(b.get(0).unwrap()).unwrap());
    }

    else if given_options.is_present("sort_album") {
        if input_file_given {
            // Sort given_data
            given_data.sort_by(|a, b| a.get(1).unwrap().partial_cmp(b.get(1).unwrap()).unwrap());
        }

        // Sort found_data
        found_data.sort_by(|a, b| a.get(1).unwrap().partial_cmp(b.get(1).unwrap()).unwrap());
    }

    write_found_data(given_options.value_of("outfile").unwrap(), &found_data).unwrap();

    // If the user provided an input file
    if input_file_given {
        let mut new_records = Vec::new();

        for found_record in &found_data {
            // Boolean for determining if a matching artist-album combination was found in the read in data
            let mut match_found = false;

            // Loop through every element of the read in data
            for given_record in &given_data {
                // If both the artist and album match
                if found_record.get(0) == given_record.get(0) && found_record.get(1) == given_record.get(1) {
                    // A match was found
                    match_found = true;
                }
            }

            // If no match was found
            if !match_found {
                new_records.push(found_record);
            }
        }

        // Print the new data to stdout
		// Header
        println!("=====================NEW-DATA=====================");

        let mut wtr = Writer::from_writer(io::stdout());

        for record in new_records {
            wtr.write_record(record)?;
        }

        wtr.flush()?;

        println!();

        let mut missing_data = Vec::new();

        // Find missing data
		// Loop through the given data
        for given_record in given_data {
            let mut given_found = false;

            for found_record in &found_data {
                if given_record.get(0) == found_record.get(0) && given_record.get(1) == found_record.get(1) {
                    given_found = true;
                }
            }

            if !given_found {
                missing_data.push(given_record);
            }
        }

        // Print the missing data to stdout
		// Header
        println!("===================MISSING-DATA===================");

        for record in missing_data {
            wtr.write_record(&record)?;
        }

        wtr.flush()?;
    }

    // We're done
    Ok(())
}

fn read_given_data(path: &str) -> Result<Vec<csv::StringRecord>, Box<dyn Error>> {
    let mut read_data = Vec::new(); // Vector to store read data in

    // Create a reader from the given path
    let mut rdr = Reader::from_path(path)?;
    // Loop through all the records in the file
    for result in rdr.records() {
        let record = result?;
        // Add the record to the vector
        read_data.push(record);
    }

    Ok(read_data)
}

fn index_directory(path: &str) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let mut found_data = Vec::new();

    for artist in fs::read_dir(path)? {
        let artist = artist?;
        if artist.file_type()?.is_dir() {
            for album in fs::read_dir(artist.path())? {
                let album = album?;
                if album.file_type()?.is_dir() {
                    found_data.push(StringRecord::from(vec![artist.file_name().into_string().unwrap(),
                                                        album.file_name().into_string().unwrap()]));
                }
            }
        }
    }

    Ok(found_data)
}

fn write_found_data(path: &str, found_data: &Vec<StringRecord>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;

    wtr.write_record(&["Artist", "Album"])?;

    for record in found_data {
        wtr.write_record(record)?;
    }

    wtr.flush()?;

    Ok(())
}
