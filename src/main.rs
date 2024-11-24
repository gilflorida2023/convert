use std::{fs, io};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use clap::Parser;
use std::env;
use std::thread;
use std::time::Duration;
use crate::elapsed_time::measure_elapsed_time;

mod elapsed_time;
/// Converts a binary window file to CSV format
fn convert_file(input_path: &PathBuf, output_path: &PathBuf, verbose: bool) -> io::Result<()> {
    if verbose {
        println!("convert_file invoked"); 
    }
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);

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
    let output_file = File::create(output_path)?;
    let mut writer: BufWriter<File> = BufWriter::new(output_file);
    writeln!(writer, "# Range: {} to {}", range_start, range_end)?;
    writeln!(writer, "prime,next_value")?;

    // Read and convert prime records
    let mut count = 0;
    loop {
        //thread::yield_now();
        //thread::sleep(Duration::from_millis(10)); 
        let mut prime_bytes = [0u8; 8];
        let mut next_value_bytes = [0u8; 8];
        
        match reader.read_exact(&mut prime_bytes) {
            Ok(_) => {
                reader.read_exact(&mut next_value_bytes)?;
                let prime = u64::from_le_bytes(prime_bytes);
                let next_value = u64::from_le_bytes(next_value_bytes);
                writeln!(writer, "{},{}", prime, next_value)?;
                count += 1;
                if  count % 100 == 0 {
                    thread::sleep(Duration::from_millis(10)); // be nice to system.
                }
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


/// Command line arguments for the binary to CSV converter
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Directory containing binary window files
    #[arg(short = 'i', long = "input_dir", required = true)]
    input_directory: PathBuf,

    /// Enable verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();
    let elapsed = measure_elapsed_time(|| {
        if let Err(e) = process_directory(&args.input_directory, args.verbose) {
            eprintln!("Error: {}", e);
        }
    });
    
    println!("Sieve processing completed in {}", elapsed);
    Ok(())
}