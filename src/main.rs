use clap::Parser;
use dotenvy::dotenv;
use rand::thread_rng;

use sql_dump_anonymizer::cli::Args;
use sql_dump_anonymizer::engine::{Config};

fn main() {
    dotenv().ok(); 
    let args = Args::parse();
    let config = Config::load_from_file(&args.config).expect("Failed to load TOML file");
    let mut rng = thread_rng();

    println!("--- Rules Processing Test ---");
    for (table_name, table_config) in &config.tables {
        println!("Table: [{}]", table_name);
        
        for column in &table_config.columns {
            let original_value = Some("database_value");
            let masked_value = column.strategy.apply(original_value, &mut rng, &args.secret);
            
            println!("  -> Column: {:<15} | New Value: {:?}", column.name, masked_value);
        }
        println!("--------------------------------------------------");
    }
}