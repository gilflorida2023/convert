use clap::{Parser, ArgGroup};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["input_directory", "input_file"]),
))]
struct Args {
    /// Input parameter
    #[arg(short = 'i', long = "input-directory", value_name = "INPUTDIRECTORY")]
    input_directory: Option<String>,

    /// File parameter
    #[arg(short = 'f', long = "input-file", value_name = "INPUTFILE")]
    input_file: Option<String>,

    /// Verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}


/*use std::{fs, io};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use clap::{Parser,Args};
use std::env;
use std::thread;
use std::time::Duration;
//use crate::elapsed_time::measure_elapsed_time;

mod elapsed_time;
/// Converts a binary window file to CSV format
fn convert_file(input_path: &PathBuf, output_path: &PathBuf, verbose: bool) -> io::Result<()> {
    if verbose {
        println!("convert_file invoked"); 
    }
    let file = File::open(input_path)?;
    //let mut reader = BufReader::new(file);
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
    //let output_file = File::create(output_path)?;
    let file = File::create(output_path).unwrap();
    //let mut writer: BufWriter<File> = BufWriter::new(output_file);
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

                if  count % 100000 == 0 {
                    thread::yield_now();
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

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "Convert sieve generated binary window file into csv text file.",
    long_about = "Convert sieve generated binary window file into a csv text file."
)]
struct Cli {
    #[command(flatten)]
    input: InputArgs,

    /// Verbose mode
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

/// Command line arguments for the binary to CSV converter
#[derive(Args,Debug)]
#[group(required = true, multiple = false)]
pub struct InputArgs {
    /// Directory containing binary window files
    #[arg(short = 'd', long = "input_directory")]
    input_directory: PathBuf,

    /// File option
    #[arg(short = 'f', long = "input_file")]
    input_file: PathBuf,
  
    /// Help option
    #[arg(short = 'h', long = "help")]
    help: bool,

}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    match cli.input {
        InputArgs { input_directory: input_directory, .. } => println!("Input: {:?}", input_directory.to_str()),
        InputArgs { input_file: input_file, .. } => println!("File: {:?}", input_file.to_str()),
        InputArgs { help: true, .. } => println!("Help requested"),
        _ => unreachable!(),
    }

    if cli.verbose {
        println!("Verbose mode is enabled");
    }
    
    Ok(())
}*/