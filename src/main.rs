//use clap::{Parser, Arg};
use clap::Parser;
extern crate is_prime;
extern crate elapsed_time;
use is_prime::*;
use std::{fs, io};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
//mod elapsed_time;

/// Converts a binary window file to CSV format
fn convert_file(input_path: &PathBuf, output_path: &PathBuf, verbose: bool, check: bool) -> io::Result<()> {
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
    if check {
        writeln!(writer, "prime, next_value, is_prime")?;
    } else {
        writeln!(writer, "prime, next_value")?;
    }

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
                if check {
                    let strval: String = format!("{}", prime);
                    let isprime : bool = is_prime(&strval);
                    writeln!(writer, "{},{},{}", prime, next_value, isprime)?;
                } else {
                    writeln!(writer, "{},{}", prime, next_value)?;
                }
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

fn process_directory(directory_path: &Path,verbose:bool,check:bool) -> io::Result<()> {
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
                                convert_file(&input_path, &output_path, verbose, check)?;
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
#[command(author, version, about, long_about = None)]
#[command(group(
    clap::ArgGroup::new("input")
        .required(true)
        .multiple(false)
        .args(["input_file", "input_directory"]),
))]
struct Cli {
    /// Input file path
    #[arg(short = 'f', long, value_name = "INPUT_FILE", conflicts_with = "input_directory")]
    input_file: Option<PathBuf>,

    /// Input directory path
    #[arg(short = 'i', long, value_name = "INPUT_DIRECTORY", conflicts_with = "input_file")]
    input_directory: Option<PathBuf>,

    /// Enable check mode
    #[arg(short = 'c', long, default_value_t = false)]
    check: bool,

    /// Enable verbose output
    #[arg(short = 'v', long, default_value_t = false)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    if let Some(input_file) = cli.input_file.as_ref() {
        let output_path = input_file.with_extension("csv");
        let elapsed_time = elapsed_time::measure_elapsed_time(|| {
            if let Err(e) = convert_file(input_file, &output_path, cli.verbose, cli.check) {
                eprintln!("Error processing file: {}", e);
                std::process::exit(1);
            }
        });
        println!("conversion of {:?} took {}.", output_path, elapsed_time);
    } else if let Some(input_dir) = cli.input_directory.as_ref() {
        let elapsed_time = elapsed_time::measure_elapsed_time(|| {
            if let Err(e) = process_directory(input_dir, cli.verbose, cli.check) {
                eprintln!("Error processing directory: {}", e);
                std::process::exit(1);
            }
        });
        println!("conversion of {:?} took {}.", input_dir, elapsed_time);
    }
    
    Ok(())
}