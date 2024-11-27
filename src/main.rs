use clap::{Parser, Arg};

//use clap::{Parser, ArgGroup};
extern crate is_prime;
use is_prime::*;
use std::{fs, io};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
//use std::env;
//use std::thread;
//use std::time::Duration;
//use crate::elapsed_time::measure_elapsed_time;
mod elapsed_time;


/// Converts a binary window file to CSV format
fn convert_file(input_path: &PathBuf, output_path: &PathBuf, verbose: bool) -> io::Result<()> {
    let file = File::open(input_path)?;
    let mut reader = BufReader::with_capacity(1024*1024*2,file);

    // Read range header
    let mut range_start_bytes = [0u8; 8];
    let mut range_end_bytes = [0u8; 8];
    reader.read_exact(&mut range_start_bytes)?;
    reader.read_exact(&mut range_end_bytes)?;
    let range_start = u64::from_le_bytes(range_start_bytes);
    let range_end = u64::from_le_bytes(range_end_bytes);

    if verbose {
        println!("Converting {} (range: {} to {})", 
            input_path.file_name().unwrap().to_string_lossy(),
            range_start,
            range_end);
    }

    // Create CSV file and write header
    let file = File::create(output_path).unwrap();
    let mut writer = BufWriter::with_capacity(1024*1024*2, file);
    writeln!(writer, "# Range: {} to {}", range_start, range_end)?;
    writeln!(writer, "prime,next_value")?;

    // Read and convert prime records
    let mut count = 0;
    loop {
        let mut prime_bytes = [0u8; 8];
        let mut next_value_bytes = [0u8; 8];
        
        match reader.read_exact(&mut prime_bytes) {
            Ok(_) => {
                reader.read_exact(&mut next_value_bytes)?;
                let prime = u64::from_le_bytes(prime_bytes);
                let next_value = u64::from_le_bytes(next_value_bytes);
                writeln!(writer, "{},{}", prime, next_value)?;
                count += 1;

                /*if  count % 100000 == 0 {
                    thread::yield_now();
                    thread::sleep(Duration::from_millis(10)); // be nice to system.
                }*/
            },
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
    }
    if verbose {
        println!("Wrote {} prime records to {}", 
        count,
        output_path.file_name().unwrap().to_string_lossy());
    }
    Ok(())
}

fn process_directory(directory_path: &Path,verbose:bool) -> io::Result<()> {
    // Check if the directory exists and is a directory
    if !directory_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "The path provided is not a directory"));
    }
    // Read all files in the directory
    match fs::read_dir(directory_path) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        if let Some(extension) = entry.path().extension() {
                            if extension == "bin" {
                                // Process the file
                                let  input_path: PathBuf = entry.path().to_path_buf(); // Create a mutable PathBuf from the original path
                                let mut output_path: PathBuf = entry.path().to_path_buf(); // Create a mutable PathBuf from the original path
                                output_path.set_extension("csv"); // Change the extension to .csv
                                if verbose {
                                    println!("converting from {:?} to {:?}.", input_path,output_path);
                                }
                                convert_file(&input_path, &output_path, verbose)?;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading directory entry: {}", e);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
    Ok(())
}

#[derive(Parser)]
#[command(author, version, 
    about = "Convert sieve generated binary window file into a csv text file.", 
    long_about = "Convert sieve generated binary window file into a csv text file.")]
struct Command {
    #[clap(short = 'i', long = "input_directory", value_name = "INPUT_DIRECTORY")]
    input_directory: Option<String>,

    #[clap(short = 'f', long = "input_file", value_name = "INPUT_FILE")]
    input_file: Option<String>,

    #[clap(short = 'c', long = "check", default_value_t = false)]
    check: bool,

    #[clap(short = 'v', long = "verbose", default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Command::parse();

    if let Some(input_directory) = args.input_directory {
        if args.verbose {
            println!("Input Directory: {}", input_directory);
        }
        let input_path = Path::new(&input_directory);
        let elapsed_time = elapsed_time::measure_elapsed_time(|| {
            let _ = process_directory(input_path, args.verbose);
         });
         println!("conversion of {:?} took {}.",input_path, elapsed_time);
    } else if let Some(input_file) = args.input_file {
        let input_path = PathBuf::from(input_file);
        let mut output_path = input_path.clone();
        output_path.set_extension("csv"); // Change the extension to .csv
        if args.verbose {
            println!("converting from {:?} to {:?}.", input_path,output_path);
        }
        let elapsed_time = elapsed_time::measure_elapsed_time(|| {
            let _ = convert_file(&input_path, &output_path, args.verbose);
         });
         println!("conversion of {:?} took {}.",output_path, elapsed_time);

    } else {
        eprintln!("Either -i or -f must be specified");
        std::process::exit(1);
    }

    if args.check {
        println!("Check mode is enabled");
    }

}
/*
fn main() -> io::Result<()> {
    let args = Args::parse();
    if let Some(input_directory) = args.input_directory.as_ref() {
        let input_path = Path::new(input_directory);
        let elapsed_time = elapsed_time::measure_elapsed_time(|| {
            let _ = process_directory(input_path, args.verbose);
         });
         println!("conversion of {:?} took {}.",input_path, elapsed_time);
    }  else if let Some(input_file) = args.input_file.as_ref() {
        let input_path = PathBuf::from(input_file);
        let mut output_path = input_path.clone();
        output_path.set_extension("csv"); // Change the extension to .csv
        if args.verbose {
            println!("converting from {:?} to {:?}.", input_path,output_path);
        }
        let elapsed_time = elapsed_time::measure_elapsed_time(|| {
            let _ = convert_file(&input_path, &output_path, args.verbose);
         });
         println!("conversion of {:?} took {}.",output_path, elapsed_time);
    }
    Ok(()) 
}
*/