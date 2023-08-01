use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands  {
    //pngme encode ./dice.png ruSt "This is a secret message!
    Encode { file_path: String, chunk_type: String, chunk_data: String },

    // pngme decode ./dice.png ruSt
    Decode { file_path: String, chunk_type: String },

    // pngme remove ./dice.png ruSt
    Remove { file_path: String, chunk_type: String },

    // pngme print ./dice.png
    Print { file_path: String },
}

