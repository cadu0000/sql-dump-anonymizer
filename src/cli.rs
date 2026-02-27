use clap::Parser;
use std::path::PathBuf; 

#[derive(Parser, Debug)]
#[command(
    name = "blindfold",
    version, 
    about = "High-Throughput SQL Stream Sanitizer", 
    long_about = "Ferramenta CLI para anonimização e pseudonimização determinística de dumps SQL em alta performance."
)]
pub struct Args {
    // Caminho para o arquivo de configuração (ex: regras.toml)
    #[arg(short = 'c', long, value_name = "FILE")]
    pub config: PathBuf,

    // Arquivo SQL de entrada. Se omitido, lê da entrada padrão (STDIN / Pipe)
    #[arg(short = 'i', long, value_name = "INPUT_FILE")]
    pub input: Option<PathBuf>,

    // Arquivo SQL de saída. Se omitido, escreve na saída padrão (STDOUT / Pipe)
    #[arg(short = 'o', long, value_name = "OUTPUT_FILE")]
    pub output: Option<PathBuf>,

    #[arg(short = 's', long, env = "BLINDFOLD_SECRET", hide_env_values = true)]
    pub secret: String,

    // modo log
    // #[arg(short = 'v', long, action = clap::ArgAction::SetTrue)]
    // pub verbose: bool,
}