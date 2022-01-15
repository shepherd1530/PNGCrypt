use super::args;
use super::chunk;
use super::chunk_type;
use super::png;

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use colored::*;
use rand::Rng;

pub struct Commands {}

impl Commands {
    pub fn encode(input_path: &Path, message: String, output_file_path: String) -> Result<()> {
        let path = input_path
            .canonicalize()
            .context(format!("Invalid path given {input_path:?}. File not found."))?
            .into_os_string()
            .into_string()
            .unwrap();

        let file = File::open(&path).context(format!(
            "Invalid input file. Can not find file to be encoded. {path}"
        ))?;

        let bytes: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();

        let mut png = png::Png::try_from(bytes.as_ref())
            .context("Failed to reconstruct a valid png struct from the given file.")?;

        let chunk_type_str = Commands::new_chunk_type();
        let chunk_type = chunk_type::ChunkType::from_str(&chunk_type_str).unwrap();
        let chunk_bytes = chunk::Chunk::new(chunk_type, message.bytes().collect::<Vec<u8>>());

        png.append_chunk(chunk_bytes);

        let bytes = png.as_bytes();

        let output_parent = input_path.parent().unwrap().to_str().unwrap();

        let output_path = if !output_parent.is_empty() {
            format!("{}/{}", output_parent, output_file_path)
        } else {
            output_file_path
        };

        let mut output_file = File::create(&output_path)
            .context(format!("Unable to create output file at {output_path}."))?;

        output_file.write_all(bytes.as_ref())?;
        output_file.flush()?;

        println!("Secret encoded successfully The token is {}, please keep it a secret. It will be used for decoding your message.", &chunk_type_str.white().bold());

        Ok(())
    }

    pub fn decode(input_path: &Path, chunk_type: String) -> Result<String> {
        let path = input_path
            .canonicalize()
            .context(format!("Invalid path given {input_path:?}. File not found."))?
            .into_os_string()
            .into_string()
            .unwrap();

        let file = File::open(&path).context(format!(
            "Invalid input file. Can not find file to be encoded. {path}"
        ))?;
        let bytes: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();

        let png = png::Png::try_from(bytes.as_ref())?;

        let chunk = png.chunk_by_type(&chunk_type).context("Can not decode. Critical chunk not found!!")?;

        chunk.data_as_string()
    }

    pub fn remove(input_path: &Path, chunk_type: String) -> Result<String> {
        let path = input_path
            .canonicalize()
            .context(format!("Invalid path given {input_path:?}. File not found."))?
            .into_os_string()
            .into_string()
            .unwrap();

        let file = File::open(&path).context(format!(
            "Invalid input file. Can not find file to be encoded. {path}"
        ))?;
        let bytes: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();

        let mut png = png::Png::try_from(bytes.as_ref())?;

        let chunk = png.remove_chunk(&chunk_type).context("Can not remove message. Critical chunk not found!!")?;

        let bytes = png.as_bytes();

        let mut output_file = File::create(&path).context("Unable to create file at {path}")?;
        output_file.write_all(bytes.as_ref())?;
        output_file.flush()?;

        chunk.data_as_string()
    }

    pub fn new_chunk_type() -> String {
        let mut rng = rand::thread_rng();
        let mut chunk_type = String::new();

        for _ in 0..2 {
            chunk_type.push(rng.gen_range(b'a'..b'z') as char);
        }
        for _ in 0..2 {
            chunk_type.push(rng.gen_range(b'A'..b'Z') as char);
        }

        chunk_type
    }

    pub fn from_args(args: args::Args) -> Result<()> {
        match args.operation.as_str() {
            "encode" => {
                let path = Path::new(&args.file_path);

                let message = args.message.expect("Message is required");
                let output_file = args.output_file.expect("Output file is required");

                Commands::encode(path, message, output_file)?;

                Ok(())
            }
            "decode" => {
                let chunk_type = args.chunk_type.expect("Chunk type is required");
                let path = Path::new(&args.file_path);

                println!("{}", Commands::decode(path, chunk_type)?);

                Ok(())
            }
            "remove" => {
                let path = Path::new(&args.file_path);
                let chunk_type = args.chunk_type.expect("Chunk type is required");

                println!("{}", Commands::remove(path, chunk_type)?);

                Ok(())
            }
            _ => panic!("Invalid operation"),
        }
    }
}
