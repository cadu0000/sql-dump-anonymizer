use clap::Parser;
use dotenvy::dotenv;

use sql_dump_anonymizer::io::{InputSource, OutputSource};
use sql_dump_anonymizer::cli::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); 
    let args = Args::parse();
    
    let mut reader = InputSource::new(args.input)
        .expect("Failed to open input.")
        .into_buffered();
    
    let mut writer = OutputSource::new(args.output)
        .expect("Failed to create output.")
        .into_buffered();
   
    Ok(())
}