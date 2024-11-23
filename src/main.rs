use std::{env, fs, io};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
/*
 Convert binary files created by the sieve utility into human readible csv's.
 */

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
        let mut prime_bytes = [0u8; 8];
        let mut next_value_bytes = [0u8; 8];
        
        match reader.read_exact(&mut prime_bytes) {
            Ok(_) => {
                reader.read_exact(&mut next_value_bytes)?;
                let prime = u64::from_le_bytes(prime_bytes);
                let next_value = u64::from_le_bytes(next_value_bytes);
                writeln!(writer, "{},{}", prime, next_value)?;
                count += 1;
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

fn process_directory(directory_path: &Path) -> io::Result<()> {
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
                                println!("converting from {:?} to {:?}.", input_path,output_path);
                                convert_file(&input_path, &output_path, true)?;
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

fn main() -> Result<(), io::Error> {
    // Get the first argument (after the program name)
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <directory_path>");
        eprintln!("Convert binary files created by the sieve utility into human-readable CSVs.");
        return Ok(()); // Change from return; to return Ok(())
    }

    let directory_path = Path::new(&args[1]);
    if let Err(e) = process_directory(directory_path) {
        eprintln!("Error: {}", e);
    }
    Ok(())
}

/*use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write, Read};
use std::path::PathBuf;
use clap::Parser;

/// Command line arguments for the binary to CSV converter
//#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory containing binary window files
    #[arg(short, long)]
    input_directory: PathBuf,

    /// Directory to write CSV files to
    #[arg(short, long)]
    output_directory: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}


fn main() -> io::Result<()> {
    let simulated_args = vec![
        "convert",
        "-i", "/home/minty/projects/rust/convert/data/",
        "-o", "/home/minty/projects/rust/convert/data/",
        "-v",
    ];
    let args = Args::parse_from(simulated_args);

    //let args = Args::parse();

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_directory)?;

    let mut window_num = 0;
    let mut total_files = 0;
    let _total_primes = 0;

    loop {
        let input_path = args.input_directory.join(format!("window_{}.bin", window_num));
        let output_path = args.output_directory.join(format!("window_{}.csv", window_num));

        match convert_file(&input_path, &output_path, args.verbose) {
            Ok(_) => {
                total_files += 1;
                if args.verbose {
                    println!("Successfully converted window_{}.bin", window_num);
                }
                window_num += 1;
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => break,
            Err(e) => return Err(e),
        }
    }

    println!("\nConversion Summary:");
    println!("------------------");
    println!("Total files converted: {}", total_files);
    println!("Input directory: {}", args.input_directory.display());
    println!("Output directory: {}", args.output_directory.display());

    Ok(())
}
*/
