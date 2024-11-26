use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .author("Your Name")
        .about("Does awesome things")
        .arg(
            Arg::with_name("input_file")
                .short('i')
                .long("input-file")
                .takes_value(true)
                .help("Specify an input file")
        )
        .arg(
            Arg::with_name("input_directory")
                .short('f')
                .long("input-directory")
                .takes_value(true)
                .help("Specify an input directory"),
        )
        .group(
            ArgGroup::new("exclusive_group")
                .args(&["input_file", "input_directory"])
                .multiple(false),
        )
        .subcommand(SubCommand::with_name("check")
            .about("Check something")
        )
        .subcommand(SubCommand::with_name("verbose")
            .about("Verbose mode"))
        .get_matches();

    if matches.is_present("input_file") && matches.is_present("input_directory") {
        eprintln!("Error: -i and -f cannot be used together");
        std::process::exit(1);
    }

    if let Some(matches) = matches.subcommand_matches("check") {
        println!("Running check mode");
        // Additional logic for the 'check' subcommand
    } else if let Some(matches) = matches.subcommand_matches("verbose") {
        println!("Verbose mode activated");
        // Additional logic for the 'verbose' subcommand
    }
}
/*use clap::{Parser, ArgGroup};
extern crate is_prime;
use is_prime::*;
use std::{fs, io};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
//use std::env;
//use std::thread;
//use std::time::Duration;


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

//use crate::elapsed_time::measure_elapsed_time;
mod elapsed_time;
#[derive(Parser, Debug)]
#[command(author, version, 
    about = "Convert sieve generated binary window file into a csv text file.", 
    long_about = "Convert sieve generated binary window file into a csv text file.")]
#[clap(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["input_directory", "input_file"]),
))]
struct Args {
    /// Input directory
    #[arg(short = 'i', long = "input-directory", value_name = "INPUTDIRECTORY")]
    input_directory: Option<String>,

    /// Input file
    #[arg(short = 'f', long = "input-file", value_name = "INPUTFILE")]
    input_file: Option<String>,

    /// Verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

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