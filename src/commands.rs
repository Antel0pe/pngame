use crate::{png::Png, chunk::Chunk, chunk_type::ChunkType, args::{Cli, Commands}};
use std::{fs, str::FromStr};

pub fn process_cli_args(args: Cli){
    match &args.command {
        Commands::Encode { file_path, chunk_type, chunk_data, } => {
            let mut parsed_png: Png = parse_png_contents(file_path);

            let new_chunk_type: ChunkType = ChunkType::from_str(&chunk_type).unwrap();
            let new_chunk: Chunk = Chunk::new(new_chunk_type, chunk_data.clone().into_bytes());

            parsed_png.append_chunk(new_chunk);

            write_to_png(file_path, parsed_png.as_bytes());
        },
        Commands::Decode { file_path, chunk_type, } => {
            let parsed_png: Png = parse_png_contents(file_path);

            let chunk_by_type: Option<&Chunk> = parsed_png.chunk_by_type(chunk_type);

            if let Some(matching_chunk_type) = chunk_by_type {
                println!("Found matching chunk: {}", matching_chunk_type.data_as_string().unwrap());
            } else {
                println!("No matching chunk type found...");
            }

        },
        Commands::Remove { file_path, chunk_type, } => {
            let mut parsed_png: Png = parse_png_contents(file_path);

            match parsed_png.remove_chunk(&chunk_type) {
                Ok(removed_chunk) => {
                    println!("Removing chunk: {}", removed_chunk.data_as_string().unwrap());
                },
                Err(e) => {
                    println!("{}", e);
                }
            }

            write_to_png(file_path, parsed_png.as_bytes());
        },
        Commands::Print { file_path,} => {
            let parsed_png: Png = parse_png_contents(file_path);

            println!("Signature: {:?}", parsed_png.header());

            println!("Contains {} chunks...", parsed_png.chunks().len());

            for (i, chunk) in parsed_png.chunks().iter().enumerate(){
                println!("{}: {}", i, chunk.chunk_type());
            }
        },
    }
}

fn parse_png_contents(file_path: &String) -> Png{
    let file_contents: Vec<u8> = fs::read(file_path).unwrap();
    let png: Png = Png::try_from(file_contents.as_slice()).unwrap();

    png
}   

fn write_to_png(file_path: &String, png_contents: Vec<u8>){
    fs::write(file_path, png_contents).unwrap();
}