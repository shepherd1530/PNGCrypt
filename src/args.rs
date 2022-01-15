use clap::Parser;

// make all our arguments positional

#[derive(Debug, Clone, Parser)]
#[clap(
    author = "Samuel Ajisegiri",
    version = "0.1.0",
    about = "A command line utility for embedding secret messages in PNG images"
)]
pub struct Args {
    // type of operation to perform
    #[clap(validator(validate_operation))]
    pub operation: String,

    // file path
    #[clap(short, long)]
    pub file_path: String,

    // chunk type
    #[clap(short, long)]
    pub chunk_type: Option<String>,

    // message
    #[clap(short, long, required_if_eq("operation", "encode"))]
    pub message: Option<String>,

    // output file
    #[clap(long)]
    pub output_file: Option<String>,
}

fn validate_operation(operation: &str) -> Result<(), String> {
    // check if value is either encode, decode, remove or print
    match operation {
        "encode" | "decode" | "remove" | "print" => Ok(()),
        _ => Err(format!("Invalid operation: {}", operation)),
    }
}
